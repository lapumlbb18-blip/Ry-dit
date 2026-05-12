use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::backend_sdl2::Sdl2Backend;
use crate::renderer::Renderer;

impl Sdl2Backend {
    pub fn set_draw_color(&mut self, color: Color) {
        self.last_color = Some(color);
    }
    
    pub fn clear(&mut self) {
        if let Some(c) = self.last_color {
            self.renderer.clear(c.r, c.g, c.b);
        }
    }

    pub fn fill_rect(&mut self, rect: Rect) {
        if let Some(c) = self.last_color {
            self.renderer.draw_rect(rect.x(), rect.y(), rect.width() as i32, rect.height() as i32, c.r, c.g, c.b, true);
        }
    }

    pub fn draw_rect(&mut self, rect: Rect) {
        if let Some(c) = self.last_color {
            self.renderer.draw_rect(rect.x(), rect.y(), rect.width() as i32, rect.height() as i32, c.r, c.g, c.b, false);
        }
    }

    pub fn draw_line(&mut self, p1: (i32, i32), p2: (i32, i32)) {
        if let Some(c) = self.last_color {
            self.renderer.draw_line(p1.0, p1.1, p2.0, p2.1, c.r, c.g, c.b);
        }
    }

    pub fn present(&mut self) {
        // self.renderer.present(); // Ya no es necesario
        self.window.gl_swap_window();
    }
}
