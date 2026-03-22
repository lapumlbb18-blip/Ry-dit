//! # migui - Immediate Mode GUI puro en Rust
//!
//! **Sistema de GUI independiente sin dependencias gráficas**
//!
//! ## Filosofía
//! - **Immediate Mode**: Cada frame se evalúa desde cero
//! - **Sin dependencias**: Funciona en cualquier plataforma
//! - **Backend agnóstico**: Se conecta a raylib, terminal, web, etc.
//!
//! ## Ejemplo
//! ```rust
//! use migui::{Migui, Event, WidgetId};
//!
//! let mut gui = Migui::new();
//! let mut contador = 0;
//!
//! // En tu game loop:
//! // gui.begin_frame();
//! // if gui.button(WidgetId::new("btn"), rect(10, 10, 100, 30)) {
//! //     contador += 1;
//! // }
//! // gui.end_frame();
//! ```

// ============================================================================
// TIPOS BÁSICOS
// ============================================================================

/// Identificador único para widgets
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetId(pub String);

impl WidgetId {
    pub fn new(s: &str) -> Self { Self(s.to_string()) }
}

/// Rectángulo para layout
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32, pub y: f32, pub w: f32, pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self { Self { x, y, w, h } }
    
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
}

/// Colores básicos
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const RED: Color = Color::new(230, 41, 55, 255);
    pub const GREEN: Color = Color::new(117, 203, 100, 255);
    pub const BLUE: Color = Color::new(51, 122, 206, 255);
    pub const YELLOW: Color = Color::new(253, 249, 0, 255);
    pub const GRAY: Color = Color::new(128, 128, 128, 255);
    pub const BG: Color = Color::new(30, 30, 30, 255);
    pub const PANEL: Color = Color::new(50, 50, 50, 255);
    pub const BUTTON: Color = Color::new(70, 70, 70, 255);
    pub const BUTTON_HOVER: Color = Color::new(90, 90, 90, 255);
    pub const BUTTON_ACTIVE: Color = Color::new(110, 110, 110, 255);
    pub const BORDER: Color = Color::new(100, 100, 100, 255);
    pub const TEXT: Color = Color::new(240, 240, 240, 255);
    pub const ACCENT: Color = Color::new(51, 122, 206, 255);
    
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rojo" | "red" => Color::RED,
            "verde" | "green" => Color::GREEN,
            "azul" | "blue" => Color::BLUE,
            "amarillo" | "yellow" => Color::YELLOW,
            "blanco" | "white" => Color::WHITE,
            "negro" | "black" => Color::BLACK,
            "gris" | "gray" => Color::GRAY,
            "panel" => Color::PANEL,
            "boton" | "button" => Color::BUTTON,
            "borde" | "border" => Color::BORDER,
            "texto" | "text" => Color::TEXT,
            "acento" | "accent" => Color::ACCENT,
            _ => Color::WHITE,
        }
    }
}

// ============================================================================
// EVENTOS
// ============================================================================

/// Eventos de entrada
#[derive(Debug, Clone)]
pub enum Event {
    MouseMove { x: f32, y: f32 },
    MouseDown { button: MouseButton, x: f32, y: f32 },
    MouseUp { button: MouseButton, x: f32, y: f32 },
    KeyDown { key: Key },
    KeyUp { key: Key },
    CharTyped { ch: char },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton { Left, Right, Middle }
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Escape, Enter, Backspace, ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
}

// ============================================================================
// ESTADO DE WIDGETS
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct WidgetState {
    pub hovered: bool,
    pub active: bool,
    pub clicked: bool,
}

#[derive(Debug, Clone, Default)]
pub struct WindowState {
    pub x: f32, pub y: f32,
    pub dragging: bool,
    pub drag_offset_x: f32, pub drag_offset_y: f32,
    pub open: bool,
}

#[derive(Debug, Clone)]
pub struct TextboxState {
    pub text: String,
    pub cursor_pos: usize,
    pub selected: bool,
}

impl Default for TextboxState {
    fn default() -> Self { Self { text: String::new(), cursor_pos: 0, selected: false } }
}

// ============================================================================
// COMANDOS DE DIBUJO (para el backend)
// ============================================================================

/// Comandos que el backend debe ejecutar
#[derive(Debug, Clone)]
pub enum DrawCommand {
    Clear { color: Color },
    DrawRect { rect: Rect, color: Color },
    DrawText { text: String, x: f32, y: f32, size: u32, color: Color },
    DrawLine { x1: f32, y1: f32, x2: f32, y2: f32, color: Color, thickness: f32 },
}

// ============================================================================
// MIGUI - MAIN STRUCT
// ============================================================================

pub struct Migui {
    mouse_x: f32,
    mouse_y: f32,
    mouse_down: bool,
    mouse_pressed: bool,
    mouse_released: bool,
    
    pub widget_states: std::collections::HashMap<String, WidgetState>,
    pub window_states: std::collections::HashMap<String, WindowState>,
    pub textbox_states: std::collections::HashMap<String, TextboxState>,
    
    draw_commands: Vec<DrawCommand>,
    frame_count: u64,
}

impl Migui {
    pub fn new() -> Self {
        Self {
            mouse_x: 0.0, mouse_y: 0.0,
            mouse_down: false, mouse_pressed: false, mouse_released: false,
            widget_states: std::collections::HashMap::new(),
            window_states: std::collections::HashMap::new(),
            textbox_states: std::collections::HashMap::new(),
            draw_commands: Vec::new(),
            frame_count: 0,
        }
    }
    
    // ========================================================================
    // FRAME MANAGEMENT
    // ========================================================================
    
    pub fn begin_frame(&mut self) {
        self.draw_commands.clear();
        self.mouse_pressed = false;
        self.mouse_released = false;
        self.frame_count += 1;
    }
    
    pub fn end_frame(&mut self) {
        // Los comandos están listos para el backend
    }
    
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::MouseMove { x, y } => {
                self.mouse_x = x;
                self.mouse_y = y;
            }
            Event::MouseDown { button: MouseButton::Left, x, y } => {
                self.mouse_x = x;
                self.mouse_y = y;
                self.mouse_down = true;
                self.mouse_pressed = true;
            }
            Event::MouseUp { button: MouseButton::Left, x, y } => {
                self.mouse_x = x;
                self.mouse_y = y;
                self.mouse_down = false;
                self.mouse_released = true;
            }
            Event::KeyDown { key } => {
                // Manejo de teclado para textbox
                if let Some(ts) = self.textbox_states.values_mut().find(|t| t.selected) {
                    if key == Key::Backspace && ts.cursor_pos > 0 {
                        ts.cursor_pos -= 1;
                        ts.text.remove(ts.cursor_pos);
                    }
                }
            }
            Event::CharTyped { ch } => {
                if let Some(ts) = self.textbox_states.values_mut().find(|t| t.selected) {
                    ts.text.insert(ts.cursor_pos, ch);
                    ts.cursor_pos += 1;
                }
            }
            _ => {}
        }
    }
    
    // ========================================================================
    // QUERY METHODS
    // ========================================================================
    
    pub fn mouse_x(&self) -> f32 { self.mouse_x }
    pub fn mouse_y(&self) -> f32 { self.mouse_y }
    pub fn mouse_position(&self) -> (f32, f32) { (self.mouse_x, self.mouse_y) }
    pub fn is_mouse_pressed(&self) -> bool { self.mouse_pressed }
    pub fn is_mouse_down(&self) -> bool { self.mouse_down }
    
    pub fn draw_commands(&self) -> &[DrawCommand] { &self.draw_commands }
    
    // ========================================================================
    // WIDGETS
    // ========================================================================
    
    /// Button - retorna true si fue clickeado en este frame
    pub fn button(&mut self, id: WidgetId, rect: Rect, label: &str) -> bool {
        let state = self.widget_states.entry(id.0).or_insert_with(WidgetState::default);
        
        state.hovered = rect.contains(self.mouse_x, self.mouse_y);
        if state.hovered && self.mouse_pressed { state.active = true; }
        let clicked = state.hovered && state.active && self.mouse_released;
        if !self.mouse_down { state.active = false; }
        
        // Comando de dibujo
        let color = if state.active { Color::BUTTON_ACTIVE }
            else if state.hovered { Color::BUTTON_HOVER }
            else { Color::BUTTON };
        
        self.draw_commands.push(DrawCommand::DrawRect { rect, color });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x, y1: rect.y, x2: rect.x + rect.w, y2: rect.y,
            color: Color::BORDER, thickness: 2.0,
        });
        // ... más líneas del borde
        
        // Texto centrado
        self.draw_commands.push(DrawCommand::DrawText {
            text: label.to_string(),
            x: rect.x + (rect.w - label.len() as f32 * 8.0) / 2.0,
            y: rect.y + rect.h / 2.0 - 8.0,
            size: 16,
            color: Color::TEXT,
        });
        
        clicked
    }
    
    /// Label - texto estático
    pub fn label(&mut self, _id: WidgetId, text: &str, rect: Rect) {
        self.draw_commands.push(DrawCommand::DrawText {
            text: text.to_string(),
            x: rect.x,
            y: rect.y + rect.h / 4.0,
            size: 16,
            color: Color::TEXT,
        });
    }
    
    /// Checkbox - retorna true si cambió el estado
    pub fn checkbox(&mut self, id: WidgetId, label: &str, checked: &mut bool, rect: Rect) -> bool {
        let state = self.widget_states.entry(id.0.clone()).or_insert_with(WidgetState::default);
        
        let cb_rect = Rect::new(rect.x + 4.0, rect.y + 4.0, rect.h - 8.0, rect.h - 8.0);
        state.hovered = cb_rect.contains(self.mouse_x, self.mouse_y);
        
        let clicked = state.hovered && self.mouse_pressed && !state.active;
        if self.mouse_pressed { state.active = true; }
        if self.mouse_released { state.active = false; }
        
        // Dibujar checkbox
        let bg = if state.hovered { Color::BUTTON_HOVER } else { Color::BUTTON };
        self.draw_commands.push(DrawCommand::DrawRect { rect: cb_rect, color: bg });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: cb_rect.x, y1: cb_rect.y,
            x2: cb_rect.x + cb_rect.w, y2: cb_rect.y,
            color: Color::BORDER, thickness: 2.0,
        });
        
        if *checked {
            let margin = 4.0;
            self.draw_commands.push(DrawCommand::DrawRect {
                rect: Rect::new(cb_rect.x + margin, cb_rect.y + margin,
                               cb_rect.w - margin * 2.0, cb_rect.h - margin * 2.0),
                color: Color::ACCENT,
            });
        }
        
        // Label
        self.draw_commands.push(DrawCommand::DrawText {
            text: label.to_string(),
            x: rect.x + cb_rect.w + 8.0,
            y: rect.y + rect.h / 4.0,
            size: 16,
            color: Color::TEXT,
        });
        
        if clicked { *checked = !*checked; }
        clicked
    }
    
    /// Slider - retorna el valor actual
    pub fn slider(&mut self, id: WidgetId, value: f32, min: f32, max: f32, rect: Rect) -> f32 {
        let state = self.widget_states.entry(id.0).or_insert_with(WidgetState::default);
        
        let track_h = 8.0f32;
        let track_y = rect.y + (rect.h - track_h) / 2.0;
        let range = max - min;
        let norm = if range > 0.0 { (value - min) / range } else { 0.0 };
        let knob_w = track_h;
        let knob_x = rect.x + norm * (rect.w - knob_w);
        
        state.hovered = rect.contains(self.mouse_x, self.mouse_y);
        let knob_hovered = Rect::new(knob_x, track_y, knob_w, track_h)
            .contains(self.mouse_x, self.mouse_y);
        
        if (knob_hovered || state.active) && self.mouse_pressed { state.active = true; }
        
        let mut new_value = value;
        if state.active && self.mouse_x >= rect.x && self.mouse_x <= rect.x + rect.w {
            new_value = min + ((self.mouse_x - rect.x) / rect.w) * range;
            new_value = new_value.clamp(min, max);
        }
        if !self.mouse_down { state.active = false; }
        
        // Dibujar track
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: Rect::new(rect.x, track_y, rect.w, track_h),
            color: Color::BUTTON,
        });
        
        // Dibujar knob
        let knob_color = if knob_hovered || state.active { Color::BUTTON_HOVER } else { Color::BUTTON };
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: Rect::new(knob_x, track_y, knob_w, track_h),
            color: knob_color,
        });
        
        // Valor
        self.draw_commands.push(DrawCommand::DrawText {
            text: format!("{:.1}", new_value),
            x: rect.x + rect.w - 50.0,
            y: rect.y + rect.h / 4.0,
            size: 14,
            color: Color::TEXT,
        });
        
        new_value
    }
    
    /// Panel - contenedor visual
    pub fn panel(&mut self, _id: WidgetId, rect: Rect, color: Color) {
        self.draw_commands.push(DrawCommand::DrawRect { rect, color });
        self.draw_commands.push(DrawCommand::DrawLine {
            x1: rect.x, y1: rect.y, x2: rect.x + rect.w, y2: rect.y,
            color: Color::BORDER, thickness: 2.0,
        });
    }
    
    /// Textbox - retorna referencia al texto
    pub fn textbox(&mut self, id: WidgetId, rect: Rect) -> &str {
        let state = self.widget_states.entry(id.0.clone()).or_insert_with(WidgetState::default);
        let ts = self.textbox_states.entry(id.0).or_insert_with(TextboxState::default);
        
        state.hovered = rect.contains(self.mouse_x, self.mouse_y);
        if state.hovered && self.mouse_pressed { ts.selected = true; }
        else if self.mouse_pressed { ts.selected = false; }
        
        let bg = if ts.selected { Color::ACCENT }
            else if state.hovered { Color::BUTTON_HOVER }
            else { Color::BUTTON };
        
        self.draw_commands.push(DrawCommand::DrawRect { rect, color: bg });
        
        let display = if ts.selected { format!("{}_", ts.text) } else { ts.text.clone() };
        self.draw_commands.push(DrawCommand::DrawText {
            text: display,
            x: rect.x + 5.0,
            y: rect.y + rect.h / 4.0,
            size: 16,
            color: Color::TEXT,
        });
        
        &ts.text
    }
    
    pub fn set_textbox_text(&mut self, id: &str, text: String) {
        if let Some(ts) = self.textbox_states.get_mut(id) {
            ts.text = text;
        }
    }
    
    /// Window - ventana arrastrable, retorna true si está abierta
    pub fn window(&mut self, id: WidgetId, title: &str, rect: Rect, open: &mut bool) -> bool {
        if !*open { return false; }
        
        let ws = self.window_states.entry(id.0.clone())
            .or_insert_with(|| WindowState { x: rect.x, y: rect.y, ..Default::default() });
        
        if ws.dragging {
            if self.mouse_down {
                ws.x = self.mouse_x - ws.drag_offset_x;
                ws.y = self.mouse_y - ws.drag_offset_y;
            } else { ws.dragging = false; }
        }
        
        let header_h = 30.0f32;
        let header_rect = Rect::new(ws.x, ws.y, rect.w, header_h);
        let header_hovered = header_rect.contains(self.mouse_x, self.mouse_y);
        
        if header_hovered && self.mouse_pressed {
            ws.dragging = true;
            ws.drag_offset_x = self.mouse_x - ws.x;
            ws.drag_offset_y = self.mouse_y - ws.y;
        }
        
        // Cuerpo de ventana
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: Rect::new(ws.x, ws.y, rect.w, rect.h),
            color: Color::PANEL,
        });
        
        // Header
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: header_rect,
            color: Color::ACCENT,
        });
        
        // Título
        self.draw_commands.push(DrawCommand::DrawText {
            text: title.to_string(),
            x: ws.x + 10.0,
            y: ws.y + 7.0,
            size: 18,
            color: Color::WHITE,
        });
        
        // Botón cerrar
        let close_x = ws.x + rect.w - 25.0;
        let close_rect = Rect::new(close_x, ws.y + 5.0, 20.0, 20.0);
        let close_hovered = close_rect.contains(self.mouse_x, self.mouse_y);
        
        self.draw_commands.push(DrawCommand::DrawRect {
            rect: close_rect,
            color: if close_hovered { Color::RED } else { Color::new(128, 0, 0, 255) },
        });
        
        self.draw_commands.push(DrawCommand::DrawText {
            text: "X".to_string(),
            x: close_x + 5.0,
            y: ws.y + 3.0,
            size: 18,
            color: Color::WHITE,
        });
        
        if close_hovered && self.mouse_pressed { *open = false; }
        true
    }
    
    /// Message box - retorna índice del botón presionado
    pub fn message_box(&mut self, title: &str, message: &str, buttons: &[&str], rect: Rect) -> i32 {
        // Fondo
        self.draw_commands.push(DrawCommand::DrawRect { rect, color: Color::PANEL });
        
        // Título
        self.draw_commands.push(DrawCommand::DrawText {
            text: title.to_string(),
            x: rect.x + 10.0,
            y: rect.y + 10.0,
            size: 18,
            color: Color::ACCENT,
        });
        
        // Mensaje
        self.draw_commands.push(DrawCommand::DrawText {
            text: message.to_string(),
            x: rect.x + 10.0,
            y: rect.y + 35.0,
            size: 16,
            color: Color::TEXT,
        });
        
        // Botones
        let btn_w = 80.0f32;
        let btn_h = 35.0f32;
        let btn_y = rect.y + rect.h - btn_h - 10.0;
        let total_w = buttons.len() as f32 * (btn_w + 10.0) - 10.0;
        let mut btn_x = rect.x + (rect.w - total_w) / 2.0;
        
        for (i, btn_text) in buttons.iter().enumerate() {
            if self.button(WidgetId::new(&format!("msgbox_{}_{}", title, i)),
                          Rect::new(btn_x, btn_y, btn_w, btn_h), btn_text) {
                return i as i32;
            }
            btn_x += btn_w + 10.0;
        }
        
        -1
    }
}

impl Default for Migui {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rect_contains() {
        let r = Rect::new(0.0, 0.0, 100.0, 50.0);
        assert!(r.contains(50.0, 25.0));
        assert!(!r.contains(150.0, 25.0));
    }
    
    #[test]
    fn test_button_click() {
        let mut gui = Migui::new();
        gui.begin_frame();
        gui.handle_event(Event::MouseMove { x: 50.0, y: 50.0 });
        gui.handle_event(Event::MouseDown { button: MouseButton::Left, x: 50.0, y: 50.0 });
        gui.handle_event(Event::MouseUp { button: MouseButton::Left, x: 50.0, y: 50.0 });
        
        let clicked = gui.button(WidgetId::new("btn"), Rect::new(0.0, 0.0, 100.0, 100.0), "Click");
        assert!(clicked);
    }
    
    #[test]
    fn test_slider_value() {
        let mut gui = Migui::new();
        gui.begin_frame();
        gui.handle_event(Event::MouseMove { x: 150.0, y: 50.0 });
        gui.handle_event(Event::MouseDown { button: MouseButton::Left, x: 150.0, y: 50.0 });
        
        let value = gui.slider(WidgetId::new("sld"), 0.5, 0.0, 1.0, Rect::new(100.0, 40.0, 200.0, 20.0));
        assert!(value >= 0.0 && value <= 1.0);
    }
}
