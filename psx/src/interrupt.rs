use crate::registers::{Update, Write};
use crate::rw_register;

rw_register!(Mask, 0x1F80_1074);

pub enum Interrupts<'a> {
    All,
    Subset(&'a mut dyn Iterator<Item = IRQ>),
}

pub enum IRQ {
    Vblank = 0,
    GPU,
    CDROM,
    DMA,
    Timer0,
    Timer1,
    Timer2,
    Controller,
    SIO,
    SPU,
    Controller2,
}

impl Mask {
    pub fn enable(&mut self, interrupts: Interrupts) {
        match interrupts {
            Interrupts::All => self.write(0x0000_07FF),
            Interrupts::Subset(it) => {
                self.update(|mut val| {
                    for irq in it.into_iter() {
                        val |= 1 << irq as u32;
                    }
                    val
                });
            },
        }
    }

    pub fn disable(&mut self, interrupts: Interrupts) {
        match interrupts {
            Interrupts::All => self.write(0),
            Interrupts::Subset(it) => {
                self.update(|mut val| {
                    for irq in it.into_iter() {
                        val &= !(1 << irq as u32);
                    }
                    val
                });
            },
        }
    }

    pub fn free<F, R>(&mut self, f: F) -> R
    where F: FnOnce() -> R {
        self.disable(Interrupts::All);
        let ret = f();
        self.enable(Interrupts::All);
        ret
    }
}