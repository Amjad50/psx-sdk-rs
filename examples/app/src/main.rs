#![no_std]
#![no_main]
#![feature(array_map)]

libpsx::exe!();

use libpsx::gpu::Ctxt;
use libpsx::gpu::color::{Color, Palette};
use libpsx::gpu::draw;
use libpsx::gpu::position::Position;

use libpsx::util::{delay, ArrayUtils};

fn main() {
    let mut theta = 0.0;
    let delta = 1.0;
    let size = 256;
    let ctxt = Ctxt::new();
    ctxt.reset_buffer();
    ctxt.display_env.on();
    loop {
        theta += delta;
        if theta > 360.0 {
            theta -= 360.0;
        };
        draw::square(&Position::zero(), size, &Color::black(), None);
        draw(theta);
        blink();
    }
}

fn draw(theta: f32) {
    let size = 128;
    let center = Position::new(128, 128);
    let offset = Position::new(64, 64);
    let rect = Position::rect(offset, size, size);
    let pos1 = rect.map(|p| rotate_point(p, theta, center));
    let pos2: [Position; 8] = pos1.intercalate(&pos1.map(|p| rotate_point(p, 45.0, center)));
    let pos3: [Position; 16] = pos2.intercalate(&pos2.map(|p| rotate_point(p, 22.5, center)));
    let pos: [Position; 32] = pos3.intercalate(&pos3.map(|p| rotate_point(p, 11.25, center)));

    let col1 = [
        Color::aqua(),
        Color::mint(),
        Color::orange(),
        Color::indigo(),
    ];
    let col2: [Color; 8] = col1.intercalate(&col1);
    let col3 = col2.intercalate::<16>(&col2);
    let col = col3.intercalate::<32>(&col3);
    let pal = Palette::Shaded(col);
    draw::frame(&pos, &pal, None);
}

fn blink() {
    delay(100000);
}

// Does the GTE expose trig functions directly?
fn sin(mut x: f32) -> f32 {
    fn approx_sin(z: f32) -> f32 {
        4.0 * z * (180.0 - z) / (40500.0 - (z * (180.0 - z)))
    }
    while x < 0.0 {
        x += 360.0;
    }
    while x > 360.0 {
        x -= 360.0;
    }
    if x <= 180.0 {
        approx_sin(x)
    } else {
        -approx_sin(x - 180.0)
    }
}

fn cos(x: f32) -> f32 {
    let y = 90.0 - x;
    sin(y)
}

// Rotation is better handled by the GTE but this'll do for a demo
fn rotate_point(p: Position, theta: f32, c: Position) -> Position {
    let dx = p.x() as f32 - c.x() as f32;
    let dy = p.y() as f32 - c.y() as f32;
    let xp = dx * cos(theta) - dy * sin(theta);
    let yp = dy * cos(theta) + dx * sin(theta);
    let xf = xp + c.x() as f32;
    let yf = yp + c.y() as f32;
    Position::new(xf as u32, yf as u32)
}