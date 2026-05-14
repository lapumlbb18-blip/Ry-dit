use crate::renderer::Renderer;
use raylib::ffi;

pub struct Sdl2Renderer;

impl Renderer for Sdl2Renderer {
    fn clear(&mut self, r: u8, g: u8, b: u8) {
        unsafe {
            ffi::ClearBackground(ffi::Color { r, g, b, a: 255 });
        }
    }

    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8, fill: bool) {
        unsafe {
            let color = ffi::Color { r, g, b, a: 255 };
            if fill {
                ffi::DrawRectangle(x, y, w, h, color);
            } else {
                ffi::DrawRectangleLines(x, y, w, h, color);
            }
        }
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, r: u8, g: u8, b: u8) {
        unsafe {
            let color = ffi::Color { r, g, b, a: 255 };
            ffi::DrawLine(x1, y1, x2, y2, color);
        }
    }

    fn present(&mut self) {
        // En el modo híbrido, el swap lo maneja SDL2 a través de window.gl_swap_window()
        // Este método del trait se mantiene para compatibilidad pero está vacío aquí.
    }
}
