//! High-level DMA channel operations and types.
use crate::hw::dma;
use crate::hw::dma::{cdrom, mdec_in, mdec_out, pio, spu};
use crate::hw::dma::{BlockControl, ChannelControl, MemoryAddress};
use crate::hw::Register;
use core::convert::TryInto;
use strum_macros::IntoStaticStr;

mod gpu;
mod otc;

pub use gpu::GPU;
pub use otc::OTC;

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
pub enum Error {
    UnalignedAddress,
    OversizedBlock,
    EmptyBlock,
    BadBlockPartition,
    InvalidChop,
}

type Result<T> = core::result::Result<T, Error>;

/// A marker trait for DMA linked lists.
pub trait LinkedList {}

/// Specifies the DMA channel's block mode, number and length.
#[derive(Debug)]
pub enum BlockMode {
    /// A single block of fixed size.
    Single(u32),
    /// Multiple blocks of fixed size.
    Multi {
        /// The size of each block.
        words: u16,
        /// The number of blocks.
        blocks: u16,
    },
    /// A variable number of variably-sized blocks represented by a linked-list.
    LinkedList,
}

impl From<u32> for BlockMode {
    fn from(words: u32) -> BlockMode {
        BlockMode::Single(words)
    }
}

impl From<usize> for BlockMode {
    fn from(words: usize) -> BlockMode {
        BlockMode::Single(words as u32)
    }
}

/// Specifies the DMA channel's transfer mode.
#[derive(Debug)]
pub enum TransferMode {
    /// Start transfer immediately and all at once.
    Immediate = 0,
    /// Sync blocks to DMA requests.
    Request,
    /// Transfer blocks in linked-list mode.
    LinkedList,
}

/// The DMA channel's transfer direction.
#[derive(Debug)]
pub enum Direction {
    /// To RAM from a device.
    ToMemory = 0,
    /// From RAM to a device.
    FromMemory,
}

/// The DMA channel's memory address step.
#[derive(Debug)]
pub enum Step {
    /// Step forwards by 4 bytes.
    Forward = 0,
    /// Step backwards by 4 bytes.
    Backward,
}

/// The DMA channel's CPU/transfer window sizes.
#[derive(Debug)]
pub struct Chop {
    /// The size of the DMA window.
    pub dma_window: u32,
    /// The size of the CPU window.
    pub cpu_window: u32,
}

/// A handle to a DMA channel represented by a triple of registers. These should
/// be created by calling [`Self::new`] through the type aliases in the
/// [`dma`][`crate::dma`] module.
struct Channel<A: MemoryAddress, B: BlockControl, C: ChannelControl> {
    madr: A,
    bcr: B,
    control: C,
}

/// The DMA channel for transfers from RAM to the Macroblock decoder
pub struct MDECIn(Channel<mdec_in::Address, mdec_in::Block, mdec_in::Control>);
/// The DMA channel for transfers from the Macroblock decoder to RAM
pub struct MDECOut(Channel<mdec_out::Address, mdec_out::Block, mdec_out::Control>);
/// The DMA channel for CD-ROM transfers
pub struct CDROM(Channel<cdrom::Address, cdrom::Block, cdrom::Control>);
/// The DMA channel for SPU transfers
pub struct SPU(Channel<spu::Address, spu::Block, spu::Control>);
/// The DMA channel for PIO transfers
pub struct PIO(Channel<pio::Address, pio::Block, pio::Control>);

impl<A: MemoryAddress, B: BlockControl, C: ChannelControl> Channel<A, B, C> {
    /// Creates a handle to a DMA channel, initializing the channel if
    /// necessary.
    pub fn new() -> Self {
        let mut ctrl = dma::GlobalControl::new();
        if !ctrl.enabled(C::NAME) {
            ctrl.enable(C::NAME).store();
        }
        Channel {
            madr: A::skip_load(),
            bcr: B::skip_load(),
            control: C::new(),
        }
    }

    fn block_ptr(&self, block: &[u32]) -> Result<*const u32> {
        match block.last() {
            Some(last) => match self.control.get_step() {
                Step::Forward => Ok(block.as_ptr()),
                Step::Backward => Ok(last as *const u32),
            },
            None => Err(Error::EmptyBlock),
        }
    }

    /// Sends a buffer through a DMA channel in single-block mode and call `f`
    /// while the transfer completes.
    ///
    /// This blocks if the function `f` returns before the transfer completes.
    /// Returns `f`'s return value or `None` if the buffer is too large.
    pub fn send_and<F: FnOnce() -> R, R>(&mut self, block: &[u32], f: F) -> Result<R> {
        let ptr = self.block_ptr(block)?;
        self.madr.set_address(ptr).expect("u32 slice isn't word-aligned").store();
        self.bcr.set_block(block.len())?.store();
        self.control
            .set_mode(TransferMode::Immediate)
            .start()
            .store();
        core::hint::black_box(block);
        let res = f();
        self.control.wait();
        Ok(res)
    }

    /// Sends a buffer through a DMA channel in multi-block mode and call `f`
    /// while the transfer completes.
    ///
    /// This blocks if the function `f` returns before the transfer completes.
    /// Returns `f`'s return value or `None` if the buffer is too large.
    pub fn send_blocks_and<F: FnOnce() -> R, R>(
        &mut self, block: &[u32], size: usize, f: F,
    ) -> Result<R> {
        let ptr = self.block_ptr(block)?;
        self.madr.set_address(ptr)?.store();
        if block.len() % size != 0 {
            return Err(Error::BadBlockPartition)
        }
        let words = (block.len() / size)
            .try_into()
            .map_err(|_| Error::OversizedBlock)?;
        let block_len = BlockMode::Multi {
            words,
            blocks: size.try_into().map_err(|_| Error::OversizedBlock)?,
        };
        self.bcr.set_block(block_len)?.store();
        self.control.start().store();
        core::hint::black_box(block);
        let res = f();
        self.control.wait();
        Ok(res)
    }

    /// Sends a linked list through a DMA channel and call `f` while the
    /// transfer completes.
    ///
    /// This blocks if the function `f` returns before the transfer completes.
    /// Returns `f`'s return value.
    pub fn send_list_and<F: FnOnce() -> R, R, L: LinkedList + ?Sized>(
        &mut self, list: &L, f: F,
    ) -> R {
        let ptr = list as *const L as *const u32;
        self.madr
            .set_address(ptr)
            .expect("LinkedList implementor was not word-aligned")
            .store();
        self.control
            .set_mode(TransferMode::LinkedList)
            .start()
            .store();
        core::hint::black_box(list);
        let res = f();
        self.control.wait();
        res
    }
}
