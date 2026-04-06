//! Módulo Raylib - Drawing 2D/3D
//!
//! Raylib es excelente para:
//! - Drawing 2D: rectángulos, círculos, líneas, triángulos
//! - Drawing 3D: cubos, esferas, cilindros, modelos
//! - Colors: paleta completa
//! - Camera 3D: perspectiva, ortográfica
//!
//! NOTA: El texto TTF se deja a SDL2 (más profesional)

#[cfg(feature = "raylib-backend")]
use raylib::prelude::*;

/// Color ry-dit (compatible con raylib y SDL2)
#[derive(Debug, Clone, Copy)]
pub struct RyColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RyColor {
    // Colores predefinidos
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Self = Self { r: 220, g: 60, b: 60, a: 255 };
    pub const GREEN: Self = Self { r: 60, g: 200, b: 60, a: 255 };
    pub const BLUE: Self = Self { r: 60, g: 60, b: 220, a: 255 };
    pub const YELLOW: Self = Self { r: 220, g: 220, b: 60, a: 255 };
    pub const CYAN: Self = Self { r: 60, g: 200, b: 200, a: 255 };
    pub const MAGENTA: Self = Self { r: 200, g: 60, b: 200, a: 255 };
    pub const ORANGE: Self = Self { r: 220, g: 160, b: 60, a: 255 };
    pub const GRAY: Self = Self { r: 120, g: 120, b: 120, a: 255 };
    pub const DARK_GRAY: Self = Self { r: 50, g: 50, b: 60, a: 255 };

    #[cfg(feature = "raylib-backend")]
    pub fn to_raylib(&self) -> raylib::prelude::Color {
        raylib::prelude::Color::new(self.r, self.g, self.b, self.a)
    }
}

/// Contexto de dibujo Raylib
#[cfg(feature = "raylib-backend")]
pub struct RaylibDraw {
    handle: raylib::RaylibHandle,
    thread: raylib::RaylibThread,
    width: i32,
    height: i32,
}

#[cfg(feature = "raylib-backend")]
impl RaylibDraw {
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        let (handle, thread) = raylib::init(width as u32, height as u32, title, 60);
        Ok(Self { handle, thread, width, height })
    }

    pub fn begin_draw(&mut self) {
        self.handle.begin_drawing(&self.thread);
    }

    pub fn end_draw(&mut self) {
        self.handle.end_drawing(&self.thread);
    }

    pub fn clear(&mut self, color: RyColor) {
        self.handle.clear_background(color.to_raylib());
    }

    // 2D Drawing
    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: RyColor) {
        self.handle.draw_rectangle(x, y, w, h, color.to_raylib());
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: RyColor) {
        self.handle.draw_circle(x, y, radius as f32, color.to_raylib());
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: RyColor) {
        self.handle.draw_line(x1, y1, x2, y2, color.to_raylib());
    }

    pub fn draw_triangle(&mut self, v1: (i32,i32), v2: (i32,i32), v3: (i32,i32), color: RyColor) {
        let p1 = Vector2::new(v1.0 as f32, v1.1 as f32);
        let p2 = Vector2::new(v2.0 as f32, v2.1 as f32);
        let p3 = Vector2::new(v3.0 as f32, v3.1 as f32);
        self.handle.draw_triangle(p1, p2, p3, color.to_raylib());
    }

    // 3D Drawing (básico)
    pub fn draw_cube(&mut self, pos: (f32,f32,f32), size: f32, color: RyColor) {
        let p = Vector3::new(pos.0, pos.1, pos.2);
        self.handle.draw_cube(p, size, size, size, color.to_raylib());
    }

    pub fn draw_sphere(&mut self, pos: (f32,f32,f32), radius: f32, color: RyColor) {
        let p = Vector3::new(pos.0, pos.1, pos.2);
        self.handle.draw_sphere(p, radius, color.to_raylib());
    }

    // Texto (gaming style - usar SDL2 TTF para texto profesional)
    pub fn draw_text_gaming(&mut self, text: &str, x: i32, y: i32, size: i32, color: RyColor) {
        self.handle.draw_text(text, x, y, size as f32, color.to_raylib());
    }

    // Control
    pub fn should_close(&self) -> bool {
        self.handle.window_should_close()
    }

    pub fn is_key_pressed(&self, key: raylib::consts::KeyboardKey) -> bool {
        self.handle.is_key_pressed(key)
    }
}
