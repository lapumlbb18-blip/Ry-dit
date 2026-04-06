//! ry-system-ry - Sistema Universal Ry
//!
//! Integra ry-backend + migui para crear aplicaciones GUI completas.
//!
//! ## Arquitectura
//! ```text
//! ry-system-ry
//!     ├── ry-backend (raylib drawing + SDL2 TTF/input)
//!     └── migui (Dear ImGui style UI)
//! ```

pub use ry_backend;
pub use migui;

use ry_backend::sdl2_core::Sdl2Core;
use migui::{Migui, Event, Key, MouseButton, DrawCommand};

/// Sistema Ry completo - backend + UI
pub struct RySystem {
    /// Backend SDL2 con TTF, input, mouse, sprites
    pub core: Sdl2Core,
    /// UI inmediata estilo Dear ImGui
    pub gui: Migui,
    /// Debería cerrar?
    pub should_close: bool,
}

impl RySystem {
    /// Crear nuevo sistema Ry
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        let core = Sdl2Core::new(title, width, height)?;
        let gui = Migui::new();
        
        println!("[RY-SYSTEM] Sistema iniciado: {}x{}", width, height);
        println!("[RY-SYSTEM] TTF: {}", if core.font.is_some() { "✅" } else { "❌" });
        println!("[RY-SYSTEM] Mouse: click, doble-click, derecho, scroll");
        println!("[RY-SYSTEM] Touch Android: FingerDown/Motion/Up");
        
        Ok(Self {
            core,
            gui,
            should_close: false,
        })
    }

    /// Procesar eventos del frame
    pub fn poll_events(&mut self) {
        for event in self.core.event_pump.poll_iter() {
            match &event {
                ry_backend::sdl2::event::Event::Quit { .. } => {
                    self.should_close = true;
                }
                ry_backend::sdl2::event::Event::KeyDown {
                    keycode: Some(ry_backend::sdl2::keyboard::Keycode::Escape),
                    repeat: false,
                    ..
                } => {
                    self.should_close = true;
                }
                ry_backend::sdl2::event::Event::KeyDown {
                    keycode: Some(kc),
                    repeat: false,
                    ..
                } => {
                    if let Some(key) = keycode_to_migui(*kc) {
                        self.gui.handle_event(Event::KeyDown { key });
                    }
                }
                ry_backend::sdl2::event::Event::TextInput { text, .. } => {
                    for ch in text.chars() {
                        self.gui.handle_event(Event::CharTyped { ch });
                    }
                }
                ry_backend::sdl2::event::Event::MouseMotion { x, y, .. } => {
                    self.gui.handle_event(Event::MouseMove { x: *x as f32, y: *y as f32 });
                }
                ry_backend::sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    self.gui.handle_event(Event::MouseDown {
                        button: MouseButton::Left,
                        x: *x as f32, y: *y as f32,
                    });
                }
                ry_backend::sdl2::event::Event::MouseButtonUp { x, y, .. } => {
                    self.gui.handle_event(Event::MouseUp {
                        button: MouseButton::Left,
                        x: *x as f32, y: *y as f32,
                    });
                }
                ry_backend::sdl2::event::Event::FingerDown { x, y, .. } => {
                    self.gui.handle_event(Event::MouseMove { x: *x * 800.0, y: *y * 600.0 });
                    self.gui.handle_event(Event::MouseDown {
                        button: MouseButton::Left, x: *x * 800.0, y: *y * 600.0,
                    });
                }
                ry_backend::sdl2::event::Event::FingerMotion { x, y, .. } => {
                    self.gui.handle_event(Event::MouseMove { x: *x * 800.0, y: *y * 600.0 });
                }
                ry_backend::sdl2::event::Event::FingerUp { x, y, .. } => {
                    self.gui.handle_event(Event::MouseUp {
                        button: MouseButton::Left, x: *x * 800.0, y: *y * 600.0,
                    });
                }
                _ => {}
            }

            // Mouse events del Sdl2Core
            if let Some(me) = self.core.mouse.process_event(&event) {
                use ry_backend::sdl2_core::MouseEvent;
                match me {
                    MouseEvent::LeftDoubleClick { x, y } => {
                        self.gui.handle_event(Event::MouseDown {
                            button: MouseButton::Left, x: x as f32, y: y as f32,
                        });
                        self.gui.handle_event(Event::MouseUp {
                            button: MouseButton::Left, x: x as f32, y: y as f32,
                        });
                    }
                    MouseEvent::RightClick { x, y } => {
                        self.gui.handle_event(Event::MouseDown {
                            button: MouseButton::Right, x: x as f32, y: y as f32,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    /// Renderizar frame completo
    pub fn render(&mut self) {
        self.gui.begin_frame();

        // Limpiar
        self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGB(25, 25, 35));
        let _ = self.core.canvas.clear();

        // Renderizar comandos migui
        for cmd in self.gui.draw_commands() {
            match cmd {
                DrawCommand::DrawRect { rect, color } => {
                    self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    let _ = self.core.canvas.fill_rect(ry_backend::sdl2::rect::Rect::new(
                        rect.x as i32, rect.y as i32,
                        rect.w.max(1.0) as u32, rect.h.max(1.0) as u32,
                    ));
                }
                DrawCommand::DrawLine { x1, y1, x2, y2, color, thickness: _ } => {
                    self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    let _ = self.core.canvas.draw_line(
                        (*x1 as i32, *y1 as i32), (*x2 as i32, *y2 as i32),
                    );
                }
                DrawCommand::DrawText { text, x, y, size, color } => {
                    if let Some(ref font) = self.core.font {
                        let sdl_color = ry_backend::sdl2::pixels::Color::RGBA(
                            color.r, color.g, color.b, color.a,
                        );
                        if let Some(tex) = font.render_text(text, sdl_color) {
                            let _ = self.core.canvas.copy(
                                &tex.texture, None,
                                ry_backend::sdl2::rect::Rect::new(*x as i32, *y as i32, tex.width, tex.height),
                            );
                        }
                    }
                }
                DrawCommand::Clear { color } => {
                    self.core.canvas.set_draw_color(ry_backend::sdl2::pixels::Color::RGBA(
                        color.r, color.g, color.b, color.a,
                    ));
                    let _ = self.core.canvas.clear();
                }
            }
        }

        self.core.canvas.present();
    }

    /// Frame completo: poll + render
    pub fn frame(&mut self) {
        self.poll_events();
        self.render();
    }
}

fn keycode_to_migui(kc: ry_backend::sdl2::keyboard::Keycode) -> Option<Key> {
    use ry_backend::sdl2::keyboard::Keycode;
    match kc {
        Keycode::Escape => Some(Key::Escape),
        Keycode::Return | Keycode::Return2 => Some(Key::Enter),
        Keycode::Backspace => Some(Key::Backspace),
        Keycode::Up => Some(Key::ArrowUp),
        Keycode::Down => Some(Key::ArrowDown),
        Keycode::Left => Some(Key::ArrowLeft),
        Keycode::Right => Some(Key::ArrowRight),
        Keycode::A => Some(Key::A), Keycode::B => Some(Key::B),
        Keycode::C => Some(Key::C), Keycode::D => Some(Key::D),
        Keycode::E => Some(Key::E), Keycode::F => Some(Key::F),
        Keycode::G => Some(Key::G), Keycode::H => Some(Key::H),
        Keycode::I => Some(Key::I), Keycode::J => Some(Key::J),
        Keycode::K => Some(Key::K), Keycode::L => Some(Key::L),
        Keycode::M => Some(Key::M), Keycode::N => Some(Key::N),
        Keycode::O => Some(Key::O), Keycode::P => Some(Key::P),
        Keycode::Q => Some(Key::Q), Keycode::R => Some(Key::R),
        Keycode::S => Some(Key::S), Keycode::T => Some(Key::T),
        Keycode::U => Some(Key::U), Keycode::V => Some(Key::V),
        Keycode::W => Some(Key::W), Keycode::X => Some(Key::X),
        Keycode::Y => Some(Key::Y), Keycode::Z => Some(Key::Z),
        _ => None,
    }
}
