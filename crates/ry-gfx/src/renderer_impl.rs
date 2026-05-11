use crate::renderer::Renderer;
use raylib::ffi::*; 

pub struct Sdl2Renderer;

impl Renderer for Sdl2Renderer {
    fn clear(&mut self, r: u8, g: u8, b: u8) {
        unsafe {
            rlClearColor(r, g, b, 255);
            rlClearScreenBuffers();
        }
    }

    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8, fill: bool) {
        unsafe {
            let mode = if fill { 7 } else { 1 }; // RL_QUADS=7, RL_LINES=1
            rlBegin(mode as i32);
            rlColor4ub(r, g, b, 255);
            rlVertex2i(x, y);
            rlVertex2i(x + w, y);
            rlVertex2i(x + w, y + h);
            rlVertex2i(x, y + h);
            rlEnd();
        }
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, r: u8, g: u8, b: u8) {
        unsafe {
            rlBegin(1); // RL_LINES=1
            rlColor4ub(r, g, b, 255);
            rlVertex2i(x1, y1);
            rlVertex2i(x2, y2);
            rlEnd();
        }
    }

    fn present(&mut self) {
        // Nada que hacer aquí, SDL2 ya hace el swap en el backend principal
    }
}
