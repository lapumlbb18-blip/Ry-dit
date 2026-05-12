use crate::renderer::Renderer;
use gl;

pub struct Sdl2Renderer;

impl Renderer for Sdl2Renderer {
    fn clear(&mut self, r: u8, g: u8, b: u8) {
        unsafe {
            gl::ClearColor(
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                1.0
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn draw_rect(&mut self, _x: i32, _y: i32, _w: i32, _h: i32, _r: u8, _g: u8, _b: u8, _fill: bool) {
        // En OpenGL Core no hay glBegin/glEnd.
        // Se requiere un Shader Program y VAOs para dibujar rectángulos.
        // Por ahora lo dejamos vacío para validar que el ClearColor funciona sin crash.
    }

    fn draw_line(&mut self, _x1: i32, _y1: i32, _x2: i32, _y2: i32, _r: u8, _g: u8, _b: u8) {
    }

    fn present(&mut self) {
    }
}
