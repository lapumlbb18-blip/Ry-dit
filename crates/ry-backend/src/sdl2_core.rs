//! Módulo SDL2 Core - Input + Audio + Assets + Texto TTF profesional
//!
//! SDL2 es excelente para:
//! - Texto TTF profesional: anti-alias, fuentes reales, kerning
//! - Input completo: teclado, mouse, touch, gamepad
//! - Audio: SDL2_mixer (WAV, OGG, MP3)
//! - Assets: SDL2_image (PNG, JPG, BMP)

use sdl2::event::Event;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::image::LoadSurface;

// ============================================================================
// EVENTOS DE MOUSE COMPLETOS
// ============================================================================

#[derive(Debug, Clone)]
pub enum MouseEvent {
    Moved { x: i32, y: i32, dx: i32, dy: i32 },
    LeftClick { x: i32, y: i32 },
    LeftDoubleClick { x: i32, y: i32 },
    RightClick { x: i32, y: i32 },
    RightDoubleClick { x: i32, y: i32 },
    MiddleClick { x: i32, y: i32 },
    ButtonDown { x: i32, y: i32, button: sdl2::mouse::MouseButton },
    ButtonUp { x: i32, y: i32, button: sdl2::mouse::MouseButton },
    Wheel { x: i32, y: i32, direction: MouseWheelDirection },
    EnterWindow,
    LeaveWindow,
}

#[derive(Debug, Clone)]
pub enum MouseWheelDirection { Normal, Flipped }

pub struct MouseState {
    pub x: i32, pub y: i32,
    pub left_down: bool, pub right_down: bool, pub middle_down: bool,
    pub scroll_x: i32, pub scroll_y: i32,
    last_click_time: std::time::Instant,
    last_click_pos: (i32, i32),
}

impl MouseState {
    pub fn new() -> Self {
        Self { x: 0, y: 0, left_down: false, right_down: false, middle_down: false,
               scroll_x: 0, scroll_y: 0,
               last_click_time: std::time::Instant::now(), last_click_pos: (0, 0) }
    }

    pub fn process_event(&mut self, event: &Event) -> Option<MouseEvent> {
        match event {
            Event::MouseMotion { x, y, xrel, yrel, .. } => {
                self.x = *x; self.y = *y;
                Some(MouseEvent::Moved { x: *x, y: *y, dx: *xrel, dy: *yrel })
            }
            Event::MouseButtonDown { mouse_btn, x, y, clicks, .. } => {
                self.x = *x; self.y = *y;
                let now = std::time::Instant::now();
                let is_double = now.duration_since(self.last_click_time) < std::time::Duration::from_millis(500)
                    && *x == self.last_click_pos.0 && *y == self.last_click_pos.1 && *clicks == 2;
                self.last_click_time = now; self.last_click_pos = (*x, *y);
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => {
                        self.left_down = true;
                        Some(if is_double { MouseEvent::LeftDoubleClick { x: *x, y: *y } }
                             else { MouseEvent::LeftClick { x: *x, y: *y } })
                    }
                    sdl2::mouse::MouseButton::Right => {
                        self.right_down = true;
                        Some(if is_double { MouseEvent::RightDoubleClick { x: *x, y: *y } }
                             else { MouseEvent::RightClick { x: *x, y: *y } })
                    }
                    sdl2::mouse::MouseButton::Middle => {
                        self.middle_down = true;
                        Some(MouseEvent::MiddleClick { x: *x, y: *y })
                    }
                    _ => Some(MouseEvent::ButtonDown { x: *x, y: *y, button: *mouse_btn }),
                }
            }
            Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                self.x = *x; self.y = *y;
                match mouse_btn {
                    sdl2::mouse::MouseButton::Left => self.left_down = false,
                    sdl2::mouse::MouseButton::Right => self.right_down = false,
                    sdl2::mouse::MouseButton::Middle => self.middle_down = false,
                    _ => {}
                }
                Some(MouseEvent::ButtonUp { x: *x, y: *y, button: *mouse_btn })
            }
            Event::MouseWheel { x, y, direction, .. } => {
                self.scroll_x = *x; self.scroll_y = *y;
                let dir = match direction {
                    sdl2::mouse::MouseWheelDirection::Normal => MouseWheelDirection::Normal,
                    sdl2::mouse::MouseWheelDirection::Flipped => MouseWheelDirection::Flipped,
                    _ => MouseWheelDirection::Normal,
                };
                Some(MouseEvent::Wheel { x: *x, y: *y, direction: dir })
            }
            Event::Window { win_event: sdl2::event::WindowEvent::Leave, .. } => Some(MouseEvent::LeaveWindow),
            Event::Window { win_event: sdl2::event::WindowEvent::Enter, .. } => Some(MouseEvent::EnterWindow),
            _ => None,
        }
    }
}

impl Default for MouseState { fn default() -> Self { Self::new() } }

// ============================================================================
// TEXTO TTF PROFESIONAL
// ============================================================================

pub struct TextTexture {
    pub texture: Texture<'static>,
    pub width: u32,
    pub height: u32,
}

pub struct TtfFont {
    texture_creator: std::rc::Rc<TextureCreator<sdl2::video::WindowContext>>,
    font_path: String,
    font_size: u16,
}

impl TtfFont {
    pub fn new(path: String, size: u16, tc: std::rc::Rc<TextureCreator<sdl2::video::WindowContext>>) -> Self {
        Self { texture_creator: tc, font_path: path, font_size: size }
    }

    pub fn render_text(&self, text: &str, color: SdlColor) -> Option<TextTexture> {
        let ttf = sdl2::ttf::init().ok()?;
        let font = ttf.load_font(&self.font_path, self.font_size).ok()?;
        let surface = font.render(text).blended(color).ok()?;
        let (w, h) = surface.size();
        let texture = self.texture_creator.create_texture_from_surface(&surface).ok()?;
        let st: Texture<'static> = unsafe { std::mem::transmute(texture) };
        Some(TextTexture { texture: st, width: w, height: h })
    }

    pub fn measure_text(&self, text: &str) -> (u32, u32) {
        if let Ok(ttf) = sdl2::ttf::init() {
            if let Ok(font) = ttf.load_font(&self.font_path, self.font_size) {
                return font.size_of(text).unwrap_or((0, 0));
            }
        }
        (text.len() as u32 * 8, self.font_size as u32)
    }
}

// ============================================================================
// SDL2 CORE
// ============================================================================

pub struct Sdl2Core {
    pub canvas: Canvas<Window>,
    pub event_pump: sdl2::EventPump,
    pub mouse: MouseState,
    pub texture_creator: std::rc::Rc<TextureCreator<sdl2::video::WindowContext>>,
    pub font: Option<TtfFont>,
    pub width: i32, pub height: i32,
}

impl Sdl2Core {
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        sdl2::hint::set("SDL_VIDEODRIVER", "x11");
        sdl2::hint::set("SDL_RENDER_DRIVER", "opengles2");
        sdl2::hint::set("SDL_VIDEO_X11_FORCE_EGL", "1");
        sdl2::hint::set("SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH", "1");
        sdl2::hint::set("SDL_HINT_TOUCH_MOUSE_EVENTS", "1");
        sdl2::hint::set("SDL_HINT_ENABLE_SCREEN_KEYBOARD", "1");
        sdl2::hint::set("SDL_HINT_IME_SHOW_UI", "1");

        let sdl = sdl2::init().map_err(|e| e.to_string())?;
        let video = sdl.video().map_err(|e| e.to_string())?;
        sdl2::ttf::init().map_err(|e| e.to_string())?;
        unsafe { sdl2::sys::SDL_StartTextInput(); }

        let window = video.window(title, width as u32, height as u32)
            .position_centered().opengl().build().map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().accelerated().present_vsync()
            .build().map_err(|e| e.to_string())?;
        let texture_creator = std::rc::Rc::new(canvas.texture_creator());
        let event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

        let font = Self::load_system_font(std::rc::Rc::clone(&texture_creator));

        Ok(Self { canvas, event_pump, mouse: MouseState::new(), texture_creator, font, width, height })
    }

    fn load_system_font(tc: std::rc::Rc<TextureCreator<sdl2::video::WindowContext>>) -> Option<TtfFont> {
        for path in &["/system/fonts/Roboto-Regular.ttf", "/system/fonts/DroidSans.ttf",
                       "/data/data/com.termux/files/usr/share/fonts/DejaVuSans.ttf"] {
            if std::path::Path::new(path).exists() {
                println!("[SDL2-TTF] Fuente: {}", path);
                return Some(TtfFont::new((*path).into(), 16, tc));
            }
        }
        eprintln!("[SDL2-TTF] Sin fuente del sistema");
        None
    }

    pub fn poll_events(&mut self) -> Vec<MouseEvent> {
        let mut evts = Vec::new();
        for event in self.event_pump.poll_iter() {
            if let Some(me) = self.mouse.process_event(&event) { evts.push(me); }
        }
        evts
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, color: SdlColor) {
        if let Some(ref font) = self.font {
            if let Some(tex) = font.render_text(text, color) {
                let _ = self.canvas.copy(&tex.texture, None, Rect::new(x, y, tex.width, tex.height));
            }
        }
    }

    pub fn load_sprite(&self, path: &str) -> Option<Sprite> {
        Sprite::load(&self.canvas, path).ok()
    }
}

// ============================================================================
// SPRITE
// ============================================================================

pub struct Sprite {
    texture: Texture<'static>,
    width: u32, height: u32,
}

impl Sprite {
    pub fn load(canvas: &Canvas<Window>, path: &str) -> Result<Self, String> {
        let tc = canvas.texture_creator();
        let surface: Surface<'_> = Surface::from_file(path).map_err(|e| e.to_string())?;
        let (w, h) = surface.size();
        let texture = tc.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        let st: Texture<'static> = unsafe { std::mem::transmute(texture) };
        Ok(Self { texture: st, width: w, height: h })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, scale: u32) {
        let _ = canvas.copy(&self.texture, None, Rect::new(x, y, self.width * scale, self.height * scale));
    }

    pub fn size(&self) -> (u32, u32) { (self.width, self.height) }
}
