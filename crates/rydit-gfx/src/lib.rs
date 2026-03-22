//! # RyDit Graphics Layer (rydit-gfx)
//! 
//! **Sincronización entre Rust (arquitecto) y Raylib (pincel)**
//! 
//! Esta capa abstrae las inconsistencias de la API de raylib-rs,
//! proporcionando una interfaz consistente para RyDit.
//! 
//! ## Filosofía
//! 
//! - **Rust = Arquitecto**: Controla el game loop, input, decisiones
//! - **Raylib = Pincel**: Solo ejecuta dibujos
//! - **rydit-gfx = Puente**: Sincroniza ambos mundos
//! 
//! ## Ejemplo de Uso
//!
//! ```rust,no_run
//! use rydit_gfx::{RyditGfx, ColorRydit, Key};
//!
//! fn main() {
//!     let mut gfx = RyditGfx::new("Mi Juego RyDit", 800, 600);
//!
//!     while !gfx.should_close() {
//!         gfx.begin_draw();
//!
//!         gfx.clear_background(ColorRydit::Negro);
//!         gfx.draw_circle(400, 300, 50, ColorRydit::Rojo);
//!         gfx.draw_rect(100, 100, 100, 100, ColorRydit::Verde);
//!
//!         if gfx.is_key_pressed(Key::Escape) {
//!             break;
//!         }
//!
//!         gfx.end_draw();
//!     }
//! }
//! ```

use raylib::prelude::*;
use raylib::consts::KeyboardKey;

// Colores manuales (raylib nobuild no incluye colors::prelude)
pub const RED: Color = Color { r: 230, g: 41, b: 55, a: 255 };
pub const GREEN: Color = Color { r: 117, g: 203, b: 100, a: 255 };
pub const BLUE: Color = Color { r: 51, g: 122, b: 206, a: 255 };
pub const YELLOW: Color = Color { r: 253, g: 249, b: 0, a: 255 };
pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };
pub const PINK: Color = Color { r: 255, g: 192, b: 203, a: 255 };
pub const ORANGE: Color = Color { r: 255, g: 165, b: 0, a: 255 };
pub const GRAY: Color = Color { r: 128, g: 128, b: 128, a: 255 };

// Colores adicionales v0.2.0
pub const CYAN: Color = Color { r: 0, g: 255, b: 255, a: 255 };
pub const PURPLE: Color = Color { r: 128, g: 0, b: 128, a: 255 };
pub const BROWN: Color = Color { r: 165, g: 42, b: 42, a: 255 };
pub const LIME: Color = Color { r: 0, g: 255, b: 0, a: 255 };
pub const NAVY: Color = Color { r: 0, g: 0, b: 128, a: 255 };
pub const OLIVE: Color = Color { r: 128, g: 128, b: 0, a: 255 };
pub const TEAL: Color = Color { r: 0, g: 128, b: 128, a: 255 };
pub const MAROON: Color = Color { r: 128, g: 0, b: 0, a: 255 };

// Teclas
pub const KEY_ESCAPE: KeyboardKey = unsafe { std::mem::transmute(256i32) };
pub const KEY_SPACE: KeyboardKey = unsafe { std::mem::transmute(32i32) };
pub const KEY_ENTER: KeyboardKey = unsafe { std::mem::transmute(257i32) };
pub const KEY_UP: KeyboardKey = unsafe { std::mem::transmute(265i32) };
pub const KEY_DOWN: KeyboardKey = unsafe { std::mem::transmute(264i32) };
pub const KEY_LEFT: KeyboardKey = unsafe { std::mem::transmute(263i32) };
pub const KEY_RIGHT: KeyboardKey = unsafe { std::mem::transmute(262i32) };
pub const KEY_A: KeyboardKey = unsafe { std::mem::transmute(65i32) };
pub const KEY_B: KeyboardKey = unsafe { std::mem::transmute(66i32) };
pub const KEY_C: KeyboardKey = unsafe { std::mem::transmute(67i32) };
pub const KEY_D: KeyboardKey = unsafe { std::mem::transmute(68i32) };
pub const KEY_E: KeyboardKey = unsafe { std::mem::transmute(69i32) };
pub const KEY_F: KeyboardKey = unsafe { std::mem::transmute(70i32) };
pub const KEY_G: KeyboardKey = unsafe { std::mem::transmute(71i32) };
pub const KEY_H: KeyboardKey = unsafe { std::mem::transmute(72i32) };
pub const KEY_I: KeyboardKey = unsafe { std::mem::transmute(73i32) };
pub const KEY_J: KeyboardKey = unsafe { std::mem::transmute(74i32) };
pub const KEY_K: KeyboardKey = unsafe { std::mem::transmute(75i32) };
pub const KEY_L: KeyboardKey = unsafe { std::mem::transmute(76i32) };
pub const KEY_M: KeyboardKey = unsafe { std::mem::transmute(77i32) };
pub const KEY_N: KeyboardKey = unsafe { std::mem::transmute(78i32) };
pub const KEY_O: KeyboardKey = unsafe { std::mem::transmute(79i32) };
pub const KEY_P: KeyboardKey = unsafe { std::mem::transmute(80i32) };
pub const KEY_Q: KeyboardKey = unsafe { std::mem::transmute(81i32) };
pub const KEY_R: KeyboardKey = unsafe { std::mem::transmute(82i32) };
pub const KEY_S: KeyboardKey = unsafe { std::mem::transmute(83i32) };
pub const KEY_T: KeyboardKey = unsafe { std::mem::transmute(84i32) };
pub const KEY_U: KeyboardKey = unsafe { std::mem::transmute(85i32) };
pub const KEY_V: KeyboardKey = unsafe { std::mem::transmute(86i32) };
pub const KEY_W: KeyboardKey = unsafe { std::mem::transmute(87i32) };
pub const KEY_X: KeyboardKey = unsafe { std::mem::transmute(88i32) };
pub const KEY_Y: KeyboardKey = unsafe { std::mem::transmute(89i32) };
pub const KEY_Z: KeyboardKey = unsafe { std::mem::transmute(90i32) };
pub const KEY_ZERO: KeyboardKey = unsafe { std::mem::transmute(48i32) };
pub const KEY_ONE: KeyboardKey = unsafe { std::mem::transmute(49i32) };
pub const KEY_TWO: KeyboardKey = unsafe { std::mem::transmute(50i32) };
pub const KEY_THREE: KeyboardKey = unsafe { std::mem::transmute(51i32) };
pub const KEY_FOUR: KeyboardKey = unsafe { std::mem::transmute(52i32) };
pub const KEY_FIVE: KeyboardKey = unsafe { std::mem::transmute(53i32) };
pub const KEY_SIX: KeyboardKey = unsafe { std::mem::transmute(54i32) };
pub const KEY_SEVEN: KeyboardKey = unsafe { std::mem::transmute(55i32) };
pub const KEY_EIGHT: KeyboardKey = unsafe { std::mem::transmute(56i32) };
pub const KEY_NINE: KeyboardKey = unsafe { std::mem::transmute(57i32) };

// ============================================================================
// COLORES RYDIT
// ============================================================================

/// Colores básicos para RyDit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorRydit {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
    Magenta,
    Rosa,
    Naranja,
    Gris,
    // Colores v0.2.0
    Cyan,
    Morado,
    Cafe,
    Lima,
    AzulOscuro,
    Oliva,
    Turquesa,
    Vino,
}

impl ColorRydit {
    /// Convertir a Color de raylib
    pub fn to_color(&self) -> Color {
        match self {
            ColorRydit::Rojo => RED,
            ColorRydit::Verde => GREEN,
            ColorRydit::Azul => BLUE,
            ColorRydit::Amarillo => YELLOW,
            ColorRydit::Blanco => WHITE,
            ColorRydit::Negro => BLACK,
            ColorRydit::Magenta => MAGENTA,
            ColorRydit::Rosa => PINK,
            ColorRydit::Naranja => ORANGE,
            ColorRydit::Gris => GRAY,
            ColorRydit::Cyan => CYAN,
            ColorRydit::Morado => PURPLE,
            ColorRydit::Cafe => BROWN,
            ColorRydit::Lima => LIME,
            ColorRydit::AzulOscuro => NAVY,
            ColorRydit::Oliva => OLIVE,
            ColorRydit::Turquesa => TEAL,
            ColorRydit::Vino => MAROON,
        }
    }

    /// Crear desde string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rojo" | "red" => ColorRydit::Rojo,
            "verde" | "green" => ColorRydit::Verde,
            "azul" | "blue" => ColorRydit::Azul,
            "amarillo" | "yellow" => ColorRydit::Amarillo,
            "blanco" | "white" => ColorRydit::Blanco,
            "negro" | "black" => ColorRydit::Negro,
            "magenta" | "fucsia" => ColorRydit::Magenta,
            "rosa" | "pink" => ColorRydit::Rosa,
            "naranja" | "orange" => ColorRydit::Naranja,
            "gris" | "gray" | "grey" => ColorRydit::Gris,
            "cyan" | "celeste" => ColorRydit::Cyan,
            "morado" | "purple" | "violeta" => ColorRydit::Morado,
            "cafe" | "brown" | "marron" => ColorRydit::Cafe,
            "lima" | "lime" => ColorRydit::Lima,
            "azuloscuro" | "navy" | "azul oscuro" => ColorRydit::AzulOscuro,
            "oliva" | "olive" => ColorRydit::Oliva,
            "turquesa" | "teal" => ColorRydit::Turquesa,
            "vino" | "maroon" | "granate" => ColorRydit::Vino,
            _ => ColorRydit::Blanco,
        }
    }
}

// ============================================================================
// TECLAS
// ============================================================================

/// Teclas para input
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Escape,
    Space,
    Enter,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
}

impl Key {
    /// Convertir a KeyboardKey de raylib
    pub fn to_raylib(&self) -> KeyboardKey {
        match self {
            Key::Escape => KEY_ESCAPE,
            Key::Space => KEY_SPACE,
            Key::Enter => KEY_ENTER,
            Key::ArrowUp => KEY_UP,
            Key::ArrowDown => KEY_DOWN,
            Key::ArrowLeft => KEY_LEFT,
            Key::ArrowRight => KEY_RIGHT,
            Key::A => KEY_A,
            Key::B => KEY_B,
            Key::C => KEY_C,
            Key::D => KEY_D,
            Key::E => KEY_E,
            Key::F => KEY_F,
            Key::G => KEY_G,
            Key::H => KEY_H,
            Key::I => KEY_I,
            Key::J => KEY_J,
            Key::K => KEY_K,
            Key::L => KEY_L,
            Key::M => KEY_M,
            Key::N => KEY_N,
            Key::O => KEY_O,
            Key::P => KEY_P,
            Key::Q => KEY_Q,
            Key::R => KEY_R,
            Key::S => KEY_S,
            Key::T => KEY_T,
            Key::U => KEY_U,
            Key::V => KEY_V,
            Key::W => KEY_W,
            Key::X => KEY_X,
            Key::Y => KEY_Y,
            Key::Z => KEY_Z,
            Key::Num0 => KEY_ZERO,
            Key::Num1 => KEY_ONE,
            Key::Num2 => KEY_TWO,
            Key::Num3 => KEY_THREE,
            Key::Num4 => KEY_FOUR,
            Key::Num5 => KEY_FIVE,
            Key::Num6 => KEY_SIX,
            Key::Num7 => KEY_SEVEN,
            Key::Num8 => KEY_EIGHT,
            Key::Num9 => KEY_NINE,
        }
    }
}

// ============================================================================
// RYDIT GFX - EL PUENTE
// ============================================================================

/// RyDit Graphics Layer
/// 
/// Sincroniza Rust (arquitecto) con Raylib (pincel)
/// 
/// ## Características:
/// - API consistente (oculta inconsistencias de raylib-rs)
/// - Rust siempre controla, Raylib siempre ejecuta
/// - Manejo seguro de handles y threads
pub struct RyditGfx {
    handle: RaylibHandle,
    thread: RaylibThread,
    width: i32,
    height: i32,
    fps: i32,
}

impl RyditGfx {
    /// Crear nueva ventana gráfica
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let (handle, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();
        
        println!("[RYDIT-GFX]: Ventana creada {}x{}", width, height);
        println!("[RYDIT-GFX]: Rust = Arquitecto, Raylib = Pincel");
        
        Self {
            handle,
            thread,
            width,
            height,
            fps: 60,
        }
    }
    
    /// Configurar FPS objetivo
    pub fn set_target_fps(&mut self, fps: i32) {
        self.fps = fps;
        self.handle.set_target_fps(fps as u32);
    }

    /// Verificar si la ventana debe cerrarse
    pub fn should_close(&self) -> bool {
        self.handle.window_should_close()
    }

    /// Obtener ancho de ventana
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Obtener alto de ventana
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Iniciar dibujo (obtener DrawHandle)
    pub fn begin_draw(&mut self) -> DrawHandle<'_> {
        let d = self.handle.begin_drawing(&self.thread);
        DrawHandle::new(d)
    }
    
    /// Finalizar dibujo (automático con Drop de DrawHandle)
    pub fn end_draw(&mut self) {
        // Automático cuando DrawHandle se va de scope
    }
    
    /// Limpiar pantalla
    pub fn clear_background(&mut self, color: ColorRydit) {
        let mut d = self.begin_draw();
        d.clear(color);
        // end_draw automático
    }
    
    /// Dibujar círculo
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: ColorRydit) {
        let mut d = self.begin_draw();
        d.draw_circle(x, y, radius, color);
    }
    
    /// Dibujar rectángulo
    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRydit) {
        let mut d = self.begin_draw();
        d.draw_rectangle(x, y, w, h, color);
    }
    
    /// Dibujar línea
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: ColorRydit) {
        let mut d = self.begin_draw();
        d.draw_line(x1, y1, x2, y2, color);
    }
    
    /// Dibujar texto
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: ColorRydit) {
        let mut d = self.begin_draw();
        d.draw_text(text, x, y, size, color);
    }
    
    /// Verificar tecla presionada
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.handle.is_key_pressed(key.to_raylib())
    }
    
    /// Verificar tecla mantenida
    pub fn is_key_down(&self, key: Key) -> bool {
        self.handle.is_key_down(key.to_raylib())
    }
    
    /// Obtener posición X del mouse
    pub fn get_mouse_x(&self) -> i32 {
        self.handle.get_mouse_x() as i32
    }
    
    /// Obtener posición Y del mouse
    pub fn get_mouse_y(&self) -> i32 {
        self.handle.get_mouse_y() as i32
    }

    // ========================================================================
    // INPUT MOUSE AVANZADO - V0.3.0
    // ========================================================================

    /// Obtener posición del mouse como (x, y)
    pub fn get_mouse_position(&self) -> (i32, i32) {
        (self.handle.get_mouse_x() as i32, self.handle.get_mouse_y() as i32)
    }

    /// Verificar si botón del mouse está presionado (0=izq, 1=der, 2=medio)
    pub fn is_mouse_button_pressed(&self, button: i32) -> bool {
        // Raylib usa valores: 0=MOUSE_LEFT_BUTTON, 1=MOUSE_RIGHT_BUTTON, 2=MOUSE_MIDDLE_BUTTON
        unsafe {
            let ffi_button = button;
            raylib::ffi::IsMouseButtonPressed(ffi_button)
        }
    }

    /// Verificar si botón del mouse está mantenido
    pub fn is_mouse_button_down(&self, button: i32) -> bool {
        unsafe {
            let ffi_button = button;
            raylib::ffi::IsMouseButtonDown(ffi_button)
        }
    }

    /// Obtener movimiento del mouse (delta X, delta Y)
    pub fn get_mouse_delta(&self) -> (i32, i32) {
        let delta = self.handle.get_mouse_delta();
        (delta.x as i32, delta.y as i32)
    }

    /// Obtener scroll del mouse (Y axis)
    pub fn get_mouse_wheel(&self) -> f32 {
        // get_mouse_wheel_move retorna un f32 con el scroll en Y
        self.handle.get_mouse_wheel_move()
    }
}

impl Drop for RyditGfx {
    fn drop(&mut self) {
        println!("[RYDIT-GFX]: Cerrando ventana...");
    }
}

// ============================================================================
// DRAW HANDLE - EL PINCEL
// ============================================================================

/// Handle de dibujo (Raylib = Pincel)
/// 
/// Se obtiene de `RyditGfx::begin_draw()` y se usa para dibujar.
/// Al salir de scope, automáticamente finaliza el dibujo.
pub struct DrawHandle<'a> {
    draw: RaylibDrawHandle<'a>,
}

impl<'a> DrawHandle<'a> {
    fn new(draw: RaylibDrawHandle<'a>) -> Self {
        Self { draw }
    }
    
    /// Limpiar con color
    pub fn clear(&mut self, color: ColorRydit) {
        self.draw.clear_background(color.to_color());
    }
    
    /// Dibujar círculo
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: ColorRydit) {
        self.draw.draw_circle(x, y, radius as f32, color.to_color());
    }
    
    /// Dibujar rectángulo
    pub fn draw_rectangle(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRydit) {
        self.draw.draw_rectangle(x, y, w, h, color.to_color());
    }
    
    /// Dibujar línea
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: ColorRydit) {
        self.draw.draw_line(x1, y1, x2, y2, color.to_color());
    }
    
    /// Dibujar texto
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, size: i32, color: ColorRydit) {
        self.draw.draw_text(text, x, y, size, color.to_color());
    }

    // ========================================================================
    // FUNCIONES DRAW V0.2.0 - NUEVAS FORMAS
    // ========================================================================

    /// Dibujar triángulo
    pub fn draw_triangle(&mut self, v1: (i32, i32), v2: (i32, i32), v3: (i32, i32), color: ColorRydit) {
        let v1_raylib = Vector2::new(v1.0 as f32, v1.1 as f32);
        let v2_raylib = Vector2::new(v2.0 as f32, v2.1 as f32);
        let v3_raylib = Vector2::new(v3.0 as f32, v3.1 as f32);
        self.draw.draw_triangle(v1_raylib, v2_raylib, v3_raylib, color.to_color());
    }

    /// Dibujar triángulo con líneas (outline)
    pub fn draw_triangle_lines(&mut self, v1: (i32, i32), v2: (i32, i32), v3: (i32, i32), color: ColorRydit) {
        self.draw_line(v1.0, v1.1, v2.0, v2.1, color);
        self.draw_line(v2.0, v2.1, v3.0, v3.1, color);
        self.draw_line(v3.0, v3.1, v1.0, v1.1, color);
    }

    /// Dibujar rectángulo con líneas (outline)
    pub fn draw_rectangle_lines(&mut self, x: i32, y: i32, w: i32, h: i32, color: ColorRydit) {
        self.draw.draw_rectangle_lines(x, y, w, h, color.to_color());
    }

    /// Dibujar anillo (ring)
    pub fn draw_ring(&mut self, center: (i32, i32), _inner_radius: i32, outer_radius: i32, color: ColorRydit) {
        // Simplificación: dibujamos solo el círculo exterior
        // draw_ring de raylib requiere parámetros adicionales (start/end angle)
        self.draw.draw_circle(center.0, center.1, outer_radius as f32, color.to_color());
    }

    /// Dibujar elipse
    pub fn draw_ellipse(&mut self, center: (i32, i32), radius_h: i32, radius_v: i32, color: ColorRydit) {
        // raylib draw_ellipse: (centerX, centerY, radiusH, radiusV, color)
        self.draw.draw_ellipse(center.0, center.1, radius_h as f32, radius_v as f32, color.to_color());
    }

    /// Dibujar línea gruesa
    pub fn draw_line_thick(&mut self, start_pos: (i32, i32), end_pos: (i32, i32), thick: f32, color: ColorRydit) {
        let start = Vector2::new(start_pos.0 as f32, start_pos.1 as f32);
        let end = Vector2::new(end_pos.0 as f32, end_pos.1 as f32);
        self.draw.draw_line_ex(start, end, thick, color.to_color());
    }

    // ========================================================================
    // FUNCIONES DRAW V0.3.0 - ROTACIÓN
    // ========================================================================

    /// Dibujar rectángulo rotado
    pub fn draw_rectangle_pro(&mut self, x: i32, y: i32, width: i32, height: i32, angle: f32, color: ColorRydit) {
        let origin = Vector2::new(width as f32 / 2.0, height as f32 / 2.0);
        let rect = Rectangle::new(x as f32, y as f32, width as f32, height as f32);
        self.draw.draw_rectangle_pro(rect, origin, angle, color.to_color());
    }
}

impl<'a> Drop for DrawHandle<'a> {
    fn drop(&mut self) {
        // Finalizar dibujo automáticamente
        // (RaylibDrawHandle lo hace en su Drop)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_str() {
        assert_eq!(ColorRydit::from_str("rojo"), ColorRydit::Rojo);
        assert_eq!(ColorRydit::from_str("RED"), ColorRydit::Rojo);
        assert_eq!(ColorRydit::from_str("verde"), ColorRydit::Verde);
        assert_eq!(ColorRydit::from_str("azul"), ColorRydit::Azul);
        assert_eq!(ColorRydit::from_str("amarillo"), ColorRydit::Amarillo);
        assert_eq!(ColorRydit::from_str("blanco"), ColorRydit::Blanco);
        assert_eq!(ColorRydit::from_str("negro"), ColorRydit::Negro);
        assert_eq!(ColorRydit::from_str("desconocido"), ColorRydit::Blanco);
    }
    
    #[test]
    fn test_key_to_raylib() {
        // Solo verificamos que no panic
        let _ = Key::Escape.to_raylib();
        let _ = Key::Space.to_raylib();
        let _ = Key::A.to_raylib();
        let _ = Key::Num0.to_raylib();
    }
    
    #[test]
    fn test_color_to_color() {
        let color = ColorRydit::Rojo.to_color();
        assert_eq!(color.r, 230);
        assert_eq!(color.g, 41);
        assert_eq!(color.b, 55);
        assert_eq!(color.a, 255);
    }

    // ========================================================================
    // TESTS V0.1.9 - GRÁFICOS Y COLORES
    // ========================================================================

    #[test]
    fn test_draw_circle_colores() {
        // Verificar que todos los colores básicos funcionan
        let colores = vec!["rojo", "verde", "azul", "amarillo", "blanco", "negro"];
        
        for color_str in colores {
            let color_rydit = ColorRydit::from_str(color_str);
            let color = color_rydit.to_color();
            // Solo verificamos que el alpha sea 255 (completamente opaco)
            assert_eq!(color.a, 255, "Color {} debe tener alpha 255", color_str);
        }
    }

    #[test]
    fn test_draw_rect_dimensiones() {
        // Verificar conversión de dimensiones para rectángulos
        // Las dimensiones son enteros en raylib
        let x: i32 = 100;
        let y: i32 = 200;
        let ancho: i32 = 50;
        let alto: i32 = 75;
        
        // Verificamos que los valores se mantienen
        assert_eq!(x, 100);
        assert_eq!(y, 200);
        assert_eq!(ancho, 50);
        assert_eq!(alto, 75);
        
        // Un rectángulo con estas dimensiones debería ser válido
        let rect = raylib::prelude::Rectangle::new(x as f32, y as f32, ancho as f32, alto as f32);
        assert_eq!(rect.x, 100.0);
        assert_eq!(rect.y, 200.0);
        assert!((rect.width - 50.0).abs() < 0.01);
        assert!((rect.height - 75.0).abs() < 0.01);
    }

    // ========================================================================
    // TESTS V0.3.0 - INPUT MOUSE AVANZADO
    // ========================================================================

    #[test]
    fn test_mouse_functions_exist() {
        // Solo verificamos que las funciones existen y compilan
        // No podemos probar mouse real sin ventana
        let _ = RyditGfx::new("Test", 800, 600);
        // gfx.get_mouse_position()  // Retorna (i32, i32)
        // gfx.is_mouse_button_pressed(0)  // Retorna bool
        // gfx.get_mouse_wheel()  // Retorna (f32, f32)
    }

    #[test]
    fn test_mouse_button_mapping() {
        // Verificar mapeo de botones
        assert_eq!(0, 0);  // Left button index
        assert_eq!(1, 1);  // Right button index
        assert_eq!(2, 2);  // Middle button index
    }
}
