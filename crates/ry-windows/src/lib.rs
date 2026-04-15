//! # Ry-Windows (ry-windows)
//!
//! Ventana unificada configurable por plataforma para **Ry-Dit**.
//!
//! ## Filosofía
//! - **Unificar SDL2 + Raylib** bajo una API común
//! - **Configurable por plataforma**: Termux/openbox, Windows, Linux, Mac, Android, iOS
//! - **Menús/submenus/controles preconfigurables** en el buildeo
//! - El binario no manda herramientas por defecto — se configura qué generar
//!
//! ## Uso básico
//!
//! ```rust,ignore
//! use ry_windows::{WindowConfig, WindowBuilder, Backend, Platform};
//!
//! let config = WindowConfig::new("Mi Juego", 1280, 720)
//!     .platform(Platform::Termux)
//!     .backend(Backend::Sdl2)
//!     .show_fps(true)
//!     .fullscreen(false);
//!
//! let mut win = WindowBuilder::build(&config)?;
//!
//! while !win.should_close() {
//!     win.poll_events();
//!     win.begin_frame();
//!     // ... dibujar ...
//!     win.end_frame();
//! }
//! ```
//!
//! ## Presets de plataforma
//!
//! | Plataforma | Backend por defecto | Controles | Notas |
//! |------------|-------------------|-----------|-------|
//! | **Termux** | SDL2 (X11) | Teclado físico | openbox, zink opcional |
//! | **Desktop Linux** | SDL2 o Raylib | Teclado + mouse | X11/Wayland |
//! | **Windows** | SDL2 | Teclado + mouse + gamepad | DirectX fallback |
//! | **macOS** | SDL2 | Teclado + mouse + trackpad | Metal fallback |
//! | **Android** | Raylib | Touch controls | On-screen joysticks |
//! | **iOS** | Raylib | Touch controls | On-screen joysticks |

#![allow(clippy::too_many_arguments)]

// ============================================================================
// PLATFORM PRESETS
// ============================================================================

/// Plataforma destino
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// Termux con X11 (openbox)
    Termux,
    /// Desktop Linux (X11/Wayland)
    Linux,
    /// Windows
    Windows,
    /// macOS
    MacOS,
    /// Android
    Android,
    /// iOS
    Ios,
    /// Web/WASM
    Web,
}

impl Platform {
    /// Backend recomendado por plataforma
    pub fn default_backend(&self) -> Backend {
        match self {
            Platform::Termux | Platform::Linux | Platform::Windows | Platform::MacOS => Backend::Sdl2,
            Platform::Android | Platform::Ios => Backend::Raylib,
            Platform::Web => Backend::Raylib,
        }
    }

    /// ¿Soporta controles táctiles nativos?
    pub fn has_native_touch(&self) -> bool {
        matches!(self, Platform::Android | Platform::Ios)
    }

    /// ¿Tiene teclado físico?
    pub fn has_physical_keyboard(&self) -> bool {
        !matches!(self, Platform::Android | Platform::Ios)
    }

    /// ¿Soporta gamepad nativo?
    pub fn has_native_gamepad(&self) -> bool {
        matches!(self, Platform::Windows | Platform::Linux | Platform::MacOS)
    }

    /// SDL hints para plataforma
    pub fn sdl_hints(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            Platform::Termux => vec![
                ("SDL_VIDEODRIVER", "x11"),
                ("SDL_RENDER_DRIVER", "opengles2"),
                ("SDL_VIDEO_X11_FORCE_EGL", "1"),
            ],
            Platform::Android => vec![
                ("SDL_ANDROID_SEPARATE_MOUSE_AND_TOUCH", "1"),
                ("SDL_HINT_TOUCH_MOUSE_EVENTS", "1"),
            ],
            Platform::Linux => vec![
                ("SDL_VIDEODRIVER", "x11"),
            ],
            _ => vec![],
        }
    }
}

// ============================================================================
// BACKEND
// ============================================================================

/// Backend de renderizado/input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// SDL2 — input completo, TTF profesional, audio
    Sdl2,
    /// Raylib — drawing 2D/3D simple, touch controls
    Raylib,
}

impl Backend {
    /// ¿Soporta TTF nativo?
    pub fn has_ttf(&self) -> bool {
        matches!(self, Backend::Sdl2)
    }

    /// ¿Soporta touch controls nativos?
    pub fn has_touch(&self) -> bool {
        matches!(self, Backend::Raylib)
    }
}

// ============================================================================
// WINDOW CONFIG
// ============================================================================

/// Configuración de ventana
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Título de la ventana
    pub title: String,
    /// Ancho en píxeles
    pub width: u32,
    /// Alto en píxeles
    pub height: u32,
    /// Plataforma destino
    pub platform: Platform,
    /// Backend de renderizado
    pub backend: Backend,
    /// ¿Mostrar FPS en pantalla?
    pub show_fps: bool,
    /// ¿Pantalla completa?
    pub fullscreen: bool,
    /// ¿VSync activado?
    pub vsync: bool,
    /// ¿Resizable?
    pub resizable: bool,
    /// FPS target (0 = ilimitado)
    pub target_fps: u32,
    /// ¿Mostrar controles en pantalla? (Android/iOS)
    pub show_touch_controls: bool,
    /// Ruta de fuente TTF personalizada (None = sistema)
    pub font_path: Option<String>,
    /// Tamaño de fuente por defecto
    pub font_size: u16,
    /// Icono de ventana (ruta)
    pub icon_path: Option<String>,
    /// Posición X de la ventana (None = centrada)
    pub position_x: Option<i32>,
    /// Posición Y de la ventana (None = centrada)
    pub position_y: Option<i32>,
}

impl WindowConfig {
    /// Crear config con valores por defecto
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            platform: Platform::Termux,
            backend: Backend::Sdl2,
            show_fps: false,
            fullscreen: false,
            vsync: true,
            resizable: false,
            target_fps: 60,
            show_touch_controls: false,
            font_path: None,
            font_size: 16,
            icon_path: None,
            position_x: None,
            position_y: None,
        }
    }

    /// Aplicar preset de plataforma
    pub fn platform(mut self, platform: Platform) -> Self {
        self.platform = platform;
        self.backend = platform.default_backend();
        self.show_touch_controls = platform.has_native_touch() && !platform.has_physical_keyboard();
        self
    }

    /// Cambiar backend
    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    /// Mostrar FPS
    pub fn show_fps(mut self, show: bool) -> Self {
        self.show_fps = show;
        self
    }

    /// Pantalla completa
    pub fn fullscreen(mut self, fs: bool) -> Self {
        self.fullscreen = fs;
        self
    }

    /// VSync
    pub fn vsync(mut self, vs: bool) -> Self {
        self.vsync = vs;
        self
    }

    /// FPS target
    pub fn target_fps(mut self, fps: u32) -> Self {
        self.target_fps = fps;
        self
    }

    /// Controles táctiles
    pub fn touch_controls(mut self, show: bool) -> Self {
        self.show_touch_controls = show;
        self
    }

    /// Fuente personalizada
    pub fn font(mut self, path: &str, size: u16) -> Self {
        self.font_path = Some(path.to_string());
        self.font_size = size;
        self
    }

    /// Posición de ventana
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position_x = Some(x);
        self.position_y = Some(y);
        self
    }

    /// Preset: juego 2D desktop
    pub fn preset_game_2d(&mut self) {
        self.vsync = true;
        self.show_fps = true;
        self.resizable = false;
        self.target_fps = 60;
    }

    /// Preset: editor visual
    pub fn preset_editor(&mut self) {
        self.vsync = false;
        self.show_fps = true;
        self.resizable = true;
        self.target_fps = 0; // ilimitado
    }

    /// Preset: demo rápida
    pub fn preset_demo(&mut self) {
        self.vsync = true;
        self.show_fps = true;
        self.resizable = false;
        self.target_fps = 30;
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self::new("Ry-Dit", 1280, 720)
    }
}

// ============================================================================
// WINDOW EVENT
// ============================================================================

/// Evento de ventana unificado
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Tecla presionada
    KeyDown { key: String, repeat: bool },
    /// Tecla liberada
    KeyUp { key: String },
    /// Texto ingresido (UTF-8)
    TextInput { text: String },
    /// Mouse movido
    MouseMoved { x: f32, y: f32, dx: f32, dy: f32 },
    /// Click mouse
    MouseDown { x: f32, y: f32, button: u8 },
    /// Liberar mouse
    MouseUp { x: f32, y: f32, button: u8 },
    /// Scroll
    MouseWheel { x: f32, y: f32 },
    /// Touch begin
    TouchDown { x: f32, y: f32, id: u64 },
    /// Touch move
    TouchMove { x: f32, y: f32, id: u64 },
    /// Touch end
    TouchUp { x: f32, y: f32, id: u64 },
    /// Gamepad botón
    GamepadDown { button: u8, id: u32 },
    /// Gamepad eje
    GamepadAxis { axis: u8, value: f32, id: u32 },
    /// Ventana redimensionada
    Resized { width: u32, height: u32 },
    /// Ventana cerrada
    CloseRequested,
    /// Sin eventos
    None,
}

// ============================================================================
// INPUT STATE (agregado simple)
// ============================================================================

/// Estado de teclas presionadas
#[derive(Debug, Default, Clone)]
pub struct InputState {
    keys_down: std::collections::HashSet<String>,
    mouse_x: f32,
    mouse_y: f32,
    mouse_down: [bool; 5],
}

impl InputState {
    pub fn new() -> Self { Self::default() }

    pub fn is_key_down(&self, key: &str) -> bool {
        self.keys_down.contains(key)
    }

    pub fn mouse_pos(&self) -> (f32, f32) {
        (self.mouse_x, self.mouse_y)
    }

    pub fn is_mouse_down(&self, button: u8) -> bool {
        button < 5 && self.mouse_down[button as usize]
    }

    pub fn process(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyDown { key, .. } => { self.keys_down.insert(key.clone()); }
            WindowEvent::KeyUp { key } => { self.keys_down.remove(key); }
            WindowEvent::MouseMoved { x, y, .. } => { self.mouse_x = *x; self.mouse_y = *y; }
            WindowEvent::MouseDown { button, .. } => {
                if *button < 5 { self.mouse_down[*button as usize] = true; }
            }
            WindowEvent::MouseUp { button, .. } => {
                if *button < 5 { self.mouse_down[*button as usize] = false; }
            }
            _ => {}
        }
    }

    pub fn clear(&mut self) {
        self.keys_down.clear();
        self.mouse_down = [false; 5];
    }
}

// ============================================================================
// WINDOW (trait abstraction)
// ============================================================================

/// Trait de ventana — implementado por SDL2 y Raylib
pub trait WindowTrait {
    fn should_close(&self) -> bool;
    fn poll_events(&mut self) -> Vec<WindowEvent>;
    fn begin_frame(&mut self);
    fn end_frame(&mut self);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn title(&self) -> &str;
    fn set_title(&mut self, title: &str);
}

// ============================================================================
// WINDOW BUILDER
// ============================================================================

/// Builder de ventana — crea la ventana según backend configurado
pub struct WindowBuilder;

impl WindowBuilder {
    /// Construir ventana desde configuración
    #[cfg(feature = "sdl2-backend")]
    pub fn build(config: &WindowConfig) -> Result<Box<dyn WindowTrait>, String> {
        match config.backend {
            Backend::Sdl2 => {
                // Apply SDL hints
                for (key, value) in config.platform.sdl_hints() {
                    sdl2::hint::set(key, value);
                }

                let sdl = sdl2::init().map_err(|e| e.to_string())?;
                let video = sdl.video().map_err(|e| e.to_string())?;

                let mut window_builder = video.window(&config.title, config.width, config.height);

                if config.position_x.is_some() && config.position_y.is_some() {
                    window_builder.position(
                        config.position_x.unwrap(),
                        config.position_y.unwrap(),
                    );
                } else {
                    window_builder.position_centered();
                }

                if config.resizable {
                    window_builder.resizable();
                }

                window_builder.opengl();

                if config.fullscreen {
                    window_builder.fullscreen();
                }

                let window = window_builder.build().map_err(|e| e.to_string())?;
                let canvas = window.into_canvas()
                    .accelerated()
                    .present_vsync()
                    .build()
                    .map_err(|e| e.to_string())?;

                // vsync already set via present_vsync()
                let _ = config.vsync;

                Ok(Box::new(Sdl2Window {
                    canvas,
                    event_pump: sdl.event_pump().map_err(|e| e.to_string())?,
                    config: config.clone(),
                    should_close: false,
                }))
            }
            Backend::Raylib => {
                Self::build_raylib(config)
            }
        }
    }

    #[cfg(not(feature = "sdl2-backend"))]
    pub fn build(config: &WindowConfig) -> Result<Box<dyn WindowTrait>, String> {
        match config.backend {
            Backend::Raylib => Self::build_raylib(config),
            Backend::Sdl2 => Err("sdl2-backend feature not enabled".into()),
        }
    }

    #[cfg(feature = "raylib-backend")]
    fn build_raylib(config: &WindowConfig) -> Result<Box<dyn WindowTrait>, String> {
        use raylib::prelude::*;

        let mut builder = RaylibVideoThread::init()
            .size(config.width, config.height)
            .title(&config.title);

        if config.fullscreen {
            builder = builder.fullscreen();
        }

        let (rl, thread) = builder.build().map_err(|e| format!("raylib init: {:?}", e))?;

        Ok(Box::new(RaylibWindow {
            rl,
            thread,
            config: config.clone(),
            should_close: false,
        }))
    }

    #[cfg(not(feature = "raylib-backend"))]
    fn build_raylib(_config: &WindowConfig) -> Result<Box<dyn WindowTrait>, String> {
        Err("raylib-backend feature not enabled".into())
    }
}

// ============================================================================
// SDL2 WINDOW IMPLEMENTATION
// ============================================================================

#[cfg(feature = "sdl2-backend")]
pub struct Sdl2Window {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    config: WindowConfig,
    should_close: bool,
}

#[cfg(feature = "sdl2-backend")]
impl WindowTrait for Sdl2Window {
    fn should_close(&self) -> bool { self.should_close }

    fn poll_events(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();
        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } | Event::Window { win_event: sdl2::event::WindowEvent::Close, .. } => {
                    self.should_close = true;
                    events.push(WindowEvent::CloseRequested);
                }
                Event::KeyDown { keycode, repeat, .. } => {
                    if let Some(kc) = keycode {
                        events.push(WindowEvent::KeyDown {
                            key: format!("{:?}", kc),
                            repeat: repeat,
                        });
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(kc) = keycode {
                        events.push(WindowEvent::KeyUp { key: format!("{:?}", kc) });
                    }
                }
                Event::TextInput { text, .. } => {
                    events.push(WindowEvent::TextInput { text });
                }
                Event::MouseMotion { x, y, xrel, yrel, .. } => {
                    events.push(WindowEvent::MouseMoved {
                        x: x as f32, y: y as f32,
                        dx: xrel as f32, dy: yrel as f32,
                    });
                }
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    events.push(WindowEvent::MouseDown {
                        x: x as f32, y: y as f32,
                        button: mouse_btn as u8,
                    });
                }
                Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    events.push(WindowEvent::MouseUp {
                        x: x as f32, y: y as f32,
                        button: mouse_btn as u8,
                    });
                }
                Event::MouseWheel { x, y, .. } => {
                    events.push(WindowEvent::MouseWheel {
                        x: x as f32, y: y as f32,
                    });
                }
                Event::FingerDown { x, y, finger_id, .. } => {
                    events.push(WindowEvent::TouchDown {
                        x, y, id: finger_id as u64,
                    });
                }
                Event::FingerMotion { x, y, finger_id, .. } => {
                    events.push(WindowEvent::TouchMove {
                        x, y, id: finger_id as u64,
                    });
                }
                Event::FingerUp { x, y, finger_id, .. } => {
                    events.push(WindowEvent::TouchUp {
                        x, y, id: finger_id as u64,
                    });
                }
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    events.push(WindowEvent::Resized {
                        width: w as u32, height: h as u32,
                    });
                }
                _ => {}
            }
        }
        events
    }

    fn begin_frame(&mut self) {
        self.canvas.clear();
    }

    fn end_frame(&mut self) {
        self.canvas.present();
    }

    fn width(&self) -> u32 {
        self.canvas.output_size().unwrap_or((self.config.width, self.config.height)).0
    }

    fn height(&self) -> u32 {
        self.canvas.output_size().unwrap_or((self.config.width, self.config.height)).1
    }

    fn title(&self) -> &str { &self.config.title }

    fn set_title(&mut self, _title: &str) {
        // SDL2 window title set via window mut
    }
}

// ============================================================================
// RAYLIB WINDOW IMPLEMENTATION
// ============================================================================

#[cfg(feature = "raylib-backend")]
pub struct RaylibWindow {
    rl: raylib::prelude::RaylibHandle,
    thread: raylib::prelude::RaylibThread,
    config: WindowConfig,
    should_close: bool,
}

#[cfg(feature = "raylib-backend")]
impl WindowTrait for RaylibWindow {
    fn should_close(&self) -> bool {
        self.should_close || self.rl.window_should_close()
    }

    fn poll_events(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();

        if self.rl.window_should_close() {
            self.should_close = true;
            events.push(WindowEvent::CloseRequested);
        }

        // Keyboard
        for key in 0..256u32 {
            let rl_key = unsafe { std::mem::transmute(key) };
            if self.rl.is_key_pressed(rl_key) {
                events.push(WindowEvent::KeyDown {
                    key: format!("key_{}", key),
                    repeat: false,
                });
            }
            if self.rl.is_key_released(rl_key) {
                events.push(WindowEvent::KeyUp { key: format!("key_{}", key) });
            }
        }

        // Mouse
        let mouse_pos = self.rl.get_mouse_position();
        if self.rl.is_mouse_button_pressed(raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT) {
            events.push(WindowEvent::MouseDown {
                x: mouse_pos.x, y: mouse_pos.y, button: 0,
            });
        }
        if self.rl.is_mouse_button_released(raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT) {
            events.push(WindowEvent::MouseUp {
                x: mouse_pos.x, y: mouse_pos.y, button: 0,
            });
        }

        // Touch
        let touches = self.rl.get_touch_points();
        for touch in touches {
            if touch.id >= 0 {
                events.push(WindowEvent::TouchDown {
                    x: touch.position.x, y: touch.position.y, id: touch.id as u64,
                });
            }
        }

        events
    }

    fn begin_frame(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(raylib::prelude::Color::BLACK);
    }

    fn end_frame(&mut self) {
        // begin_drawing handle dropped automatically
    }

    fn width(&self) -> u32 {
        self.rl.get_screen_width() as u32
    }

    fn height(&self) -> u32 {
        self.rl.get_screen_height() as u32
    }

    fn title(&self) -> &str { &self.config.title }

    fn set_title(&mut self, title: &str) {
        // raylib doesn't support runtime title change easily
        let _ = title;
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_config_default() {
        let c = WindowConfig::default();
        assert_eq!(c.title, "Ry-Dit");
        assert_eq!(c.width, 1280);
        assert_eq!(c.height, 720);
        assert_eq!(c.platform, Platform::Termux);
    }

    #[test]
    fn test_window_config_new() {
        let c = WindowConfig::new("Test", 800, 600);
        assert_eq!(c.title, "Test");
        assert_eq!(c.width, 800);
        assert_eq!(c.height, 600);
    }

    #[test]
    fn test_platform_defaults() {
        assert_eq!(Platform::Termux.default_backend(), Backend::Sdl2);
        assert_eq!(Platform::Android.default_backend(), Backend::Raylib);
        assert_eq!(Platform::Ios.default_backend(), Backend::Raylib);
    }

    #[test]
    fn test_platform_capabilities() {
        assert!(Platform::Android.has_native_touch());
        assert!(!Platform::Termux.has_native_touch());
        assert!(Platform::Termux.has_physical_keyboard());
        assert!(!Platform::Android.has_physical_keyboard());
        assert!(Platform::Windows.has_native_gamepad());
    }

    #[test]
    fn test_platform_sdl_hints() {
        let hints = Platform::Termux.sdl_hints();
        assert!(!hints.is_empty());
        assert_eq!(hints[0].0, "SDL_VIDEODRIVER");

        let hints_android = Platform::Android.sdl_hints();
        assert!(!hints_android.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let c = WindowConfig::new("Game", 1920, 1080)
            .platform(Platform::Linux)
            .show_fps(true)
            .fullscreen(false)
            .vsync(false)
            .target_fps(144);

        assert_eq!(c.platform, Platform::Linux);
        assert!(c.show_fps);
        assert!(!c.fullscreen);
        assert!(!c.vsync);
        assert_eq!(c.target_fps, 144);
    }

    #[test]
    fn test_presets() {
        let mut c = WindowConfig::default();
        c.preset_game_2d();
        assert!(c.show_fps);
        assert!(c.vsync);
        assert_eq!(c.target_fps, 60);

        c.preset_editor();
        assert!(c.resizable);
        assert_eq!(c.target_fps, 0);

        c.preset_demo();
        assert_eq!(c.target_fps, 30);
    }

    #[test]
    fn test_input_state() {
        let mut state = InputState::new();
        state.process(&WindowEvent::KeyDown { key: "A".into(), repeat: false });
        assert!(state.is_key_down("A"));

        state.process(&WindowEvent::KeyUp { key: "A".into() });
        assert!(!state.is_key_down("A"));

        state.process(&WindowEvent::MouseMoved { x: 100.0, y: 200.0, dx: 0.0, dy: 0.0 });
        let (x, y) = state.mouse_pos();
        assert_eq!(x, 100.0);
        assert_eq!(y, 200.0);
    }

    #[test]
    fn test_backend_capabilities() {
        assert!(Backend::Sdl2.has_ttf());
        assert!(!Backend::Raylib.has_ttf());
        assert!(Backend::Raylib.has_touch());
        assert!(!Backend::Sdl2.has_touch());
    }
}
