// Fichero temporal para shim
use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl crate::backend_sdl2::Sdl2Backend {
    pub fn set_draw_color(&mut self, color: Color) {
        // Guardamos el color para usarlo en la siguiente llamada a draw
        self.last_color = Some(color);
    }
    
    pub fn clear(&mut self) {
        if let Some(c) = self.last_color {
            unsafe {
                raylib::ffi::rlViewport(0, 0, self.width, self.height);
                raylib::ffi::rlMatrixMode(raylib::ffi::RL_PROJECTION as i32);
                raylib::ffi::rlLoadIdentity();
                raylib::ffi::rlOrtho(0.0, self.width as f64, self.height as f64, 0.0, -1.0, 1.0);
                raylib::ffi::rlMatrixMode(raylib::ffi::RL_MODELVIEW as i32);
                raylib::ffi::rlLoadIdentity();

                raylib::ffi::ClearBackground(raylib::ffi::Color { r: c.r, g: c.g, b: c.b, a: 255 });
            }
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
        self.renderer.present();
        unsafe {
            gl::Finish();
        }
        self.window.gl_swap_window();
    }
}
