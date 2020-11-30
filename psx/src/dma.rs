//! CPU-side DMA channel routines.

use crate::gpu::primitive::OT;
use crate::gpu::texture::{Clut, TexPage};
use crate::mmio::register::{Read, Update, Write};
use crate::mmio::{dma, gpu};
use crate::tim::TIM;

pub enum BlockSize {
    Single(usize),
    Multi { words: u16, blocks: u16 },
    LinkedList,
}

impl From<usize> for BlockSize {
    fn from(words: usize) -> Self {
        BlockSize::Single(words)
    }
}
impl From<u32> for BlockSize {
    fn from(words: u32) -> Self {
        BlockSize::Single(words as usize)
    }
}

pub enum Direction {
    ToMemory = 0,
    FromMemory,
}

pub enum Step {
    Forward = 0,
    Backward,
}

pub struct Chop {
    dma: u32,
    cpu: u32,
}

pub enum SyncMode {
    Immediate = 0,
    Request,
    LinkedList,
}

pub trait BaseAddress: Read + Write {
    /// Gets the memory address where this DMA channel will start reading
    /// from/writing to.
    fn get(&self) -> u32 {
        unsafe { self.read() }
    }

    /// Sets the memory address where this DMA channel will start reading
    /// from/writing to.
    fn set(&mut self, address: *const u32) {
        let address = address as u32;
        if cfg!(debug_assertions) {
            assert_eq!(address >> 24, 0);
        }
        unsafe { self.write(address) }
    }
}
pub trait BlockControl: Read + Write {
    fn get(&self, sync_mode: SyncMode) -> Option<BlockSize> {
        let value = unsafe { self.read() };
        match sync_mode {
            SyncMode::Immediate => match value {
                0 => Some(0x1_0000u32.into()),
                1..=0xFFFF => Some(value.into()),
                _ => None,
            },
            SyncMode::Request => Some(BlockSize::Multi {
                words: value as u16,
                blocks: (value >> 16) as u16,
            }),
            SyncMode::LinkedList => Some(BlockSize::LinkedList),
        }
    }
    fn set<T>(&mut self, block_size: T)
    where BlockSize: From<T> {
        let block_size = BlockSize::from(block_size);
        let words = match block_size {
            BlockSize::Single(words) => match words {
                0..=0xFFFF => words as u32,
                0x1_0000 => 0,
                _ => {
                    if cfg!(debug_assertions) {
                        panic!("Number of words can't exceed 0x1_0000");
                    };
                    0
                },
            },
            BlockSize::Multi { words, blocks } => words as u32 | ((blocks as u32) << 16),
            BlockSize::LinkedList => 0,
        };
        unsafe {
            self.write(words);
        }
    }
}
pub trait ChannelControl: Update {
    fn set_direction(&mut self, direction: Direction) -> &mut Self {
        unsafe {
            self.update(|val| val & !1 | (direction as u32));
        }
        self
    }
    fn set_step(&mut self, step: Step) -> &mut Self {
        unsafe {
            self.update(|val| val & !0b10 | ((step as u32) << 1));
        }
        self
    }
    fn set_chop(&mut self, chop: Option<Chop>) -> &mut Self {
        unsafe {
            self.update(|val| match chop {
                Some(chop) => {
                    if cfg!(debug_assertions) {
                        if chop.dma > 0b111 || chop.cpu > 0b111 {
                            panic!("DMA chopping windows are limited to 3 bits");
                        }
                    }
                    val | (1 << 8) | (chop.dma << 16) | (chop.cpu << 20)
                },
                None => val & !(1 << 8),
            })
        }
        self
    }
    fn set_sync_mode(&mut self, sync_mode: SyncMode) -> &mut Self {
        unsafe {
            self.update(|val| (val & !(0b11 << 9)) | ((sync_mode as u32) << 9));
        }
        self
    }
    fn sync_mode(&self) -> Option<SyncMode> {
        let bits = unsafe { self.read() };
        match (bits >> 9) & 0b11 {
            0 => Some(SyncMode::Immediate),
            1 => Some(SyncMode::Request),
            2 => Some(SyncMode::LinkedList),
            _ => None,
        }
    }
    fn start<T: Copy>(&mut self, result: T) -> Transfer<Self, T> {
        unsafe {
            match self.sync_mode() {
                Some(SyncMode::Immediate) => self.update(|val| val | (1 << 24) | (1 << 28)),
                _ => self.update(|val| val | (1 << 24)),
            }
        }
        Transfer {
            channel_control: self,
            result,
        }
    }
    fn busy(&self) -> bool {
        unsafe { self.read() & (1 << 24) != 0 }
    }
}

#[must_use]
pub struct Transfer<'a, C: ChannelControl + ?Sized, T: Copy> {
    channel_control: &'a C,
    result: T,
}

impl<C: ChannelControl, T: Copy> Transfer<'_, C, T> {
    pub fn busy(&self) -> bool {
        self.channel_control.busy()
    }

    pub fn wait(self) -> T {
        while self.busy() {}
        self.result
    }

    pub fn if_done(&self) -> Option<T> {
        if !self.busy() {
            Some(self.result)
        } else {
            None
        }
    }
}

macro_rules! enable_fn {
    ($name:ident, $bit:expr) => {
        pub fn $name(&mut self, enable: bool) {
            unsafe {
                self.update(|val| {
                    if enable {
                        val | (1 << $bit)
                    } else {
                        val & !(1 << $bit)
                    }
                })
            }
        }
    };
}
impl dma::Control {
    enable_fn!(gpu, 11);

    enable_fn!(otc, 27);
}

impl dma::gpu::Channel {
    pub fn prepare_ot(&mut self, gp1: &mut gpu::GP1) -> &mut Self {
        gp1.dma_direction(2);
        self.block_control.set(BlockSize::LinkedList);
        self.channel_control
            .set_direction(Direction::FromMemory)
            .set_sync_mode(SyncMode::LinkedList);
        self
    }

    pub fn send<const N: usize>(&mut self, ot: &OT<N>) -> Transfer<dma::gpu::ChannelControl, ()> {
        self.send_offset(ot, N - 1)
    }

    pub fn send_offset<const N: usize>(
        &mut self, ot: &OT<N>, n: usize,
    ) -> Transfer<dma::gpu::ChannelControl, ()> {
        self.base_address.set(ot.entry(n));
        self.channel_control.start(())
    }

    pub fn load_tim(&mut self, tim: &TIM, gp0: &mut gpu::GP0) -> (TexPage, Option<Clut>) {
        self.channel_control
            .set_direction(Direction::FromMemory)
            .set_step(Step::Forward)
            .set_chop(None)
            .set_sync_mode(SyncMode::Immediate);

        let texpage = tim.texpage();
        let clut = tim.clut();

        unsafe { gp0.write(0xA0 << 24) };
        let bmp = tim.bitmap().data();
        self.base_address.set(bmp.as_ptr());
        self.block_control.set(bmp.len());
        self.channel_control.start(()).wait();

        tim.clut_bitmap().map(|clut| {
            unsafe { gp0.write(0xA0 << 24) };
            self.base_address.set(clut.data().as_ptr());
            self.block_control.set(clut.data().len());
            self.channel_control.start(()).wait();
        });

        (texpage, clut)
    }
}
impl dma::otc::Channel {
    pub fn clear<const N: usize>(&mut self, ot: &OT<N>) -> Transfer<dma::otc::ChannelControl, ()> {
        self.base_address.set(ot.start());
        self.block_control.set(N as u32);
        self.channel_control
            .set_sync_mode(SyncMode::Immediate)
            .set_step(Step::Backward)
            .start(())
    }
}
