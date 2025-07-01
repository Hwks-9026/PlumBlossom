use raylib::prelude::*;
use std::collections::HashSet;
pub(crate) fn init() -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .size(640, 480)
        .title("Em-x32 Display")
        .build()
}

pub(crate) fn rasterize(window: &mut (RaylibHandle, RaylibThread), memory: Vec<u8>) {
    let mut d = window.0.begin_drawing(&window.1);
    let mut i = 0;
    while i < 307200 {
        let ci = (i * 4) as usize;
        d.draw_pixel(
            i % 640,
            i / 640,
            Color::new(memory[ci], memory[ci + 1], memory[ci + 2], memory[ci + 3]),
        );
        i += 1;
    }
}
