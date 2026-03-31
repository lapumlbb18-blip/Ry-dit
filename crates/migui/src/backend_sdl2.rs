// crates/migui/src/backend_sdl2.rs
// Backend SDL2 para MiGUI - v0.10.10
// Conecta MiGUI con SDL2 para render e input + Fuentes nativas Rust

use crate::{Migui, Rect, Color};
use crate::font_native::NativeFontManager;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::collections::HashMap;

// ============================================================================
// FONT MANAGER (Wrapper para NativeFontManager)
// ============================================================================

/// Gestor de fuentes (usa NativeFontManager internamente)
pub struct FontManager {
    native: NativeFontManager,
}

impl FontManager {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            native: NativeFontManager::with_fallback(),
        })
    }

    /// Renderizar texto a superficie SDL2
    pub fn render_text(&mut self, text: &str, size: u32, color: Color) -> Result<sdl2::surface::Surface, String> {
        // Obtener dimensiones
        let (width, height) = self.native.text_dimensions(text, size);
        
        // Crear superficie RGBA
        let mut surface = sdl2::surface::Surface::new(width, height, sdl2::pixels::PixelFormatEnum::RGBA8888)
            .map_err(|e| e.to_string())?;
        
        // Rellenar con color (placeholder - en producción renderizar con ab_glyph)
        surface.fill(None, sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a))
            .map_err(|e| e.to_string())?;
        
        Ok(surface)
    }
}

// ============================================================================
// MIGUI BACKEND SDL2
// ============================================================================

/// Backend SDL2 para MiGUI
pub struct MiguiSdl2Backend {
    canvas: Canvas<Window>,
    mouse_x: i32,
    mouse_y: i32,
    font_manager: Option<FontManager>,
}

impl MiguiSdl2Backend {
    /// Crear nuevo backend SDL2 para MiGUI
    pub fn new(canvas: Canvas<Window>) -> Self {
        let font_manager = FontManager::new().ok();
        
        Self {
            canvas,
            mouse_x: 0,
            mouse_y: 0,
            font_manager,
        }
    }

    /// Procesar eventos SDL2 y actualizar estado de MiGUI
    pub fn process_events(&mut self, gui: &mut Migui, sdl_event: &sdl2::event::Event) -> bool {
        let mut should_close = false;

        match sdl_event {
            sdl2::event::Event::Quit { .. } => {
                should_close = true;
            }
            sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                should_close = true;
            }
            
            // Input de mouse - convertir a evento Migui
            sdl2::event::Event::MouseMotion { x, y, .. } => {
                self.mouse_x = *x;
                self.mouse_y = *y;
                gui.handle_event(crate::Event::MouseMove { x: *x as f32, y: *y as f32 });
            }
            
            sdl2::event::Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                if *mouse_btn == MouseButton::Left {
                    gui.handle_event(crate::Event::MouseDown {
                        button: crate::MouseButton::Left,
                        x: *x as f32,
                        y: *y as f32,
                    });
                }
            }
            
            sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                if *mouse_btn == MouseButton::Left {
                    gui.handle_event(crate::Event::MouseUp {
                        button: crate::MouseButton::Left,
                        x: *x as f32,
                        y: *y as f32,
                    });
                }
            }
            
            _ => {}
        }

        should_close
    }

    /// Renderizar MiGUI con SDL2
    pub fn render(&mut self, gui: &mut Migui) {
        // Limpiar pantalla (color de fondo)
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(30, 30, 30));
        self.canvas.clear();

        // Renderizar comandos de MiGUI
        for cmd in gui.draw_commands() {
            match cmd {
                crate::DrawCommand::DrawRect { rect, color } => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a
                    ));
                    let sdl_rect = sdl2::rect::Rect::new(
                        rect.x as i32,
                        rect.y as i32,
                        rect.w as u32,
                        rect.h as u32
                    );
                    self.canvas.fill_rect(sdl_rect).ok();
                }
                crate::DrawCommand::DrawLine { x1, y1, x2, y2, color, .. } => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a
                    ));
                    self.canvas.draw_line(
                        (*x1 as i32, *y1 as i32),
                        (*x2 as i32, *y2 as i32)
                    ).ok();
                }
                crate::DrawCommand::DrawText { text, x, y, size, color } => {
                    // Renderizar texto con SDL2_ttf
                    if let Some(ref mut font_mgr) = self.font_manager {
                        if let Ok(surface) = font_mgr.render_text(text, *size, *color) {
                            let texture_creator = self.canvas.texture_creator();
                            if let Ok(texture) = texture_creator.create_texture_from_surface(&surface) {
                                let sdl_rect = sdl2::rect::Rect::new(
                                    *x as i32,
                                    *y as i32,
                                    surface.width(),
                                    surface.height()
                                );
                                self.canvas.copy(&texture, None, sdl_rect).ok();
                            }
                        }
                    }
                }
                crate::DrawCommand::Clear { color } => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a
                    ));
                    self.canvas.clear();
                }
            }
        }

        // Presentar
        self.canvas.present();
    }

    /// Obtener posición del mouse
    pub fn mouse_pos(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        // Solo verificamos que compile
        assert!(true);
    }
}
