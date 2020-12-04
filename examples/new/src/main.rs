#![no_std]
#![no_main]
#![feature(array_map, min_const_generics)]

use psx::framebuffer::Framebuffer;
use psx::gpu::prim;
use psx::gpu::prim::polyf::PolyF4;
use psx::gpu::prim::polyft::PolyFT4;
use psx::gpu::{Color, Vertex};
use psx::interrupt::IRQ;
use psx::printer::Printer;
use psx::tim::TIM;
use psx::{include_u32, unzip_now};

psx::exe!();

fn main(mut mmio: MMIO) {
    mmio.dma_control.gpu(true).otc(true);
    let mut fb = Framebuffer::new((0, 0), (0, 240), (320, 240), &mut mmio.gp0, &mut mmio.gp1);

    let buffer = prim::Buffer::<100>::new();
    let mut ot = prim::OT::<8>::new();

    mmio.otc_dma.clear(&ot).wait();

    draw_scene2(&buffer, &mut ot);

    draw_scene(&buffer, &mut ot);
    mmio.gpu_dma.prepare_ot(&mut mmio.gp1).send(&ot).wait();

    loop {
        mmio.gpu_dma.send(&ot).wait();
        mmio.gpu_stat.sync();

        mmio.int_stat.ack(IRQ::Vblank);

        fb.swap(&mut mmio.gp0, &mut mmio.gp1);
        psx::delay(10000000);
        panic!("panicked");
        mmio.gpu_stat.sync();
    }
}

fn draw_scene<const N: usize, const M: usize>(
    buffer: &prim::Buffer<N>, ot: &mut prim::OT<M>,
) {
    #[repr(C)]
    struct Composite {
        pub a: prim::polyf::PolyF3,
        pub b: prim::polyf::PolyF4,
    }
    impl prim::Init for Composite {
        fn init(&mut self) {
            self.a.cmd();
            self.b.cmd();
        }
    }
    let composite = buffer.alloc::<Composite>().unwrap();
    composite
        .packet()
        .a
        .vertices([(0, 0), (100, 0), (0, 100)])
        .color(Color::BLUE);
    composite
        .packet()
        .b
        .vertices([(100, 100), (50, 100), (100, 50), (25, 25)])
        .color(Color::YELLOW);
    ot.add_prim(4, composite);
}

fn draw_scene2<const N: usize, const M: usize>(
    buffer: &prim::Buffer<N>, ot: &mut prim::OT<M>,
) {
    let prim0 = buffer
        .PolyF3()
        .unwrap()
        .vertices([(25, 25), (75, 0), (75, 100)])
        .color(Color::RED);
    let buffer2 = prim::Buffer::<20>::new();
    let prim1 = buffer2
        .PolyF4()
        .unwrap()
        .vertices([(200, 130), (200, 260), (275, 230), (275, 260)])
        .color(Color::ORANGE);
    // Uncommenting the latter part should not produce a valid program
    // Review Embedonomicon DMA chapter to try and fix this
    ot.add_prim(4, prim0); //.add_prim(4, prim1);
}
