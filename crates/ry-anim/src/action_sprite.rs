//! Action Sprite System — Conecta action_assets + ry-gfx rendering
//!
//! Este módulo transforma las funciones matemáticas de `action_assets`
//! en structs de alto nivel listas para renderizar sprites animados.
//!
//! ## Arquitectura
//!
//! ```text
//! action_assets.rs (matemáticas puras)
//!     ↓ envuelve
//! action_sprite.rs (structs con estado + render)
//!     ↓ usa
//! ry-gfx draw_texture_rec (renderizado real)
//! ```
//!
//! ## Ejemplo
//!
//! ```rust,ignore
//! // Crear sprite sheet
//! let sheet = SpriteSheet::new("hero.png", 64, 64, 8, 4);
//!
//! // Crear clips de animación
//! let mut sprite = AnimatedSprite::new(sheet);
//! sprite.add_clip("idle", 0..4, 0.15, LoopMode::Loop);
//! sprite.add_clip("run", 4..12, 0.1, LoopMode::Loop);
//! sprite.add_clip("jump", 12..16, 0.1, LoopMode::Once);
//!
//! // En el game loop:
//! sprite.update(0.016); // dt
//! sprite.play("run");
//! sprite.flip_horizontal(true); // mirroring según dirección
//! sprite.render(x, y, SpriteColor::rgb(255, 255, 255));
//! ```

use crate::action_assets::{
    animation_blend, animation_state_machine, frame_animation, sprite_flip, sprite_sheet_parse,
};
use std::collections::HashMap;
use std::ops::Range;

// ============================================================================
// COLOR — Simple RGBA (independiente de ry-gfx)
// ============================================================================

/// Color RGBA para sprites
#[derive(Debug, Clone, Copy)]
pub struct SpriteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl SpriteColor {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub const fn blanco() -> Self { Self::rgb(255, 255, 255) }
    pub const fn rojo() -> Self { Self::rgb(255, 0, 0) }
    pub const fn verde() -> Self { Self::rgb(0, 255, 0) }
    pub const fn azul() -> Self { Self::rgb(0, 0, 255) }
}

// ============================================================================
// LOOP MODE — Control de repetición
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum LoopMode {
    Loop,
    Once,
    PingPong,
}

// ============================================================================
// SPRITE SHEET — Textura + grid de frames
// ============================================================================

/// Hoja de sprites — referencia a textura + dimensiones de frames
///
/// # Ejemplo
/// ```rust,ignore
/// let sheet = SpriteSheet::new("hero.png", 64, 64, 8, 4);
/// // 32 frames totales (8 cols × 4 rows), cada frame 64x64
/// let rect = sheet.get_frame_rect(5);
/// // rect = { x: 320, y: 64, w: 64, h: 64 }
/// ```
#[derive(Debug, Clone)]
pub struct SpriteSheet {
    /// ID de textura en AssetsManager
    pub texture_id: String,
    /// Ancho de cada frame
    pub frame_width: f32,
    /// Alto de cada frame
    pub frame_height: f32,
    /// Columnas en el sprite sheet
    pub columns: usize,
    /// Filas en el sprite sheet
    pub rows: usize,
    /// Total de frames
    pub total_frames: usize,
}

impl SpriteSheet {
    pub fn new(texture_id: &str, frame_width: f32, frame_height: f32, columns: usize, rows: usize) -> Self {
        Self {
            texture_id: texture_id.to_string(),
            frame_width,
            frame_height,
            columns,
            rows,
            total_frames: columns * rows,
        }
    }

    /// Obtener rect del frame (usa action_assets::sprite_sheet_parse internamente)
    pub fn get_frame_rect(&self, frame_index: usize) -> FrameRect {
        let result = sprite_sheet_parse(
            self.columns as f64 * self.frame_width as f64,
            self.rows as f64 * self.frame_height as f64,
            self.frame_width as f64,
            self.frame_height as f64,
            frame_index.min(self.total_frames - 1),
            self.columns,
        );

        FrameRect {
            x: result["x"].as_f64().unwrap_or(0.0) as f32,
            y: result["y"].as_f64().unwrap_or(0.0) as f32,
            w: result["w"].as_f64().unwrap_or(self.frame_width as f64) as f32,
            h: result["h"].as_f64().unwrap_or(self.frame_height as f64) as f32,
        }
    }
}

/// Rect de un frame individual
#[derive(Debug, Clone, Copy)]
pub struct FrameRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

// ============================================================================
/// ANIMATION CLIP — Clip nombrado con frame range
// ============================================================================

/// Clip de animación — rango de frames con duración y loop
#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub name: String,
    pub frames: Range<usize>,
    pub frame_duration: f64,
    pub loop_mode: LoopMode,
}

impl AnimationClip {
    pub fn new(name: &str, frames: Range<usize>, frame_duration: f64, loop_mode: LoopMode) -> Self {
        Self {
            name: name.to_string(),
            frames,
            frame_duration,
            loop_mode,
        }
    }

    /// Duración total del clip
    pub fn total_duration(&self) -> f64 {
        self.frames.len() as f64 * self.frame_duration
    }

    /// Frame actual basado en tiempo t (usa action_assets::frame_animation)
    pub fn get_frame(&self, t: f64) -> usize {
        let frame_count = self.frames.len();
        let loop_str = match self.loop_mode {
            LoopMode::Loop => "loop",
            LoopMode::Once => "once",
            LoopMode::PingPong => "ping_pong",
        };

        let result = frame_animation(frame_count, self.frame_duration, t, loop_str);
        let local_frame = result["current_frame"].as_u64().unwrap_or(0) as usize;
        self.frames.start + local_frame
    }

    /// Progreso actual (0.0–1.0)
    pub fn get_progress(&self, t: f64) -> f64 {
        let frame_count = self.frames.len();
        let loop_str = match self.loop_mode {
            LoopMode::Loop => "loop",
            LoopMode::Once => "once",
            LoopMode::PingPong => "ping_pong",
        };

        let result = frame_animation(frame_count, self.frame_duration, t, loop_str);
        result["progress"].as_f64().unwrap_or(0.0)
    }

    /// ¿Terminó la animación (solo para Once)?
    pub fn is_finished(&self, t: f64) -> bool {
        if self.loop_mode != LoopMode::Once { return false; }
        t >= self.total_duration()
    }

    /// ¿Está flipped (para PingPong)?
    pub fn is_flipped(&self, t: f64) -> bool {
        if self.loop_mode != LoopMode::PingPong { return false; }
        let result = frame_animation(self.frames.len(), self.frame_duration, t, "ping_pong");
        result["flipped"].as_bool().unwrap_or(false)
    }
}

// ============================================================================
/// ANIMATED SPRITE — Sprite animado completo
// ============================================================================

/// Sprite animado — combina sheet + clips + state machine + flip
///
/// # Ejemplo
/// ```rust,ignore
/// let sheet = SpriteSheet::new("hero.png", 64, 64, 8, 4);
/// let mut sprite = AnimatedSprite::new(sheet);
/// sprite.add_clip("idle", 0..4, 0.15, LoopMode::Loop);
/// sprite.add_clip("run", 4..12, 0.1, LoopMode::Loop);
/// sprite.add_clip("jump", 12..16, 0.1, LoopMode::Once);
///
/// sprite.play("run");
/// sprite.update(0.016);
/// sprite.flip_horizontal(true);
/// sprite.render(draw_handle, x, y, SpriteColor::blanco());
/// ```
pub struct AnimatedSprite {
    sheet: SpriteSheet,
    clips: HashMap<String, AnimationClip>,
    current_clip: Option<String>,
    time_in_clip: f64,
    flip_h: bool,
    flip_v: bool,
    origin_x: f64,
    origin_y: f64,
}

impl AnimatedSprite {
    pub fn new(sheet: SpriteSheet) -> Self {
        Self {
            sheet,
            clips: HashMap::new(),
            current_clip: None,
            time_in_clip: 0.0,
            flip_h: false,
            flip_v: false,
            origin_x: 0.5,
            origin_y: 0.5,
        }
    }

    /// Agregar clip de animación
    pub fn add_clip(&mut self, name: &str, frames: Range<usize>, frame_duration: f64, loop_mode: LoopMode) {
        self.clips.insert(name.to_string(), AnimationClip::new(name, frames, frame_duration, loop_mode));
    }

    /// Reproducir un clip
    pub fn play(&mut self, name: &str) {
        if self.clips.contains_key(name) {
            if self.current_clip.as_deref() != Some(name) {
                self.time_in_clip = 0.0;
            }
            self.current_clip = Some(name.to_string());
        }
    }

    /// Actualizar tiempo (llamar cada frame con dt)
    pub fn update(&mut self, dt: f64) {
        self.time_in_clip += dt;
    }

    /// Frame actual a renderizar
    pub fn current_frame(&self) -> usize {
        if let Some(ref name) = self.current_clip {
            if let Some(clip) = self.clips.get(name) {
                return clip.get_frame(self.time_in_clip);
            }
        }
        0
    }

    /// Rect del frame actual (source_rect para draw_texture_rec)
    pub fn current_frame_rect(&self) -> FrameRect {
        self.sheet.get_frame_rect(self.current_frame())
    }

    /// Flip horizontal
    pub fn flip_horizontal(&mut self, val: bool) { self.flip_h = val; }

    /// Flip vertical
    pub fn flip_vertical(&mut self, val: bool) { self.flip_v = val; }

    /// Origen para flip (0.0–1.0, default 0.5 = centro)
    pub fn set_origin(&mut self, x: f64, y: f64) { self.origin_x = x; self.origin_y = y; }

    /// Info de flip (usa action_assets::sprite_flip internamente)
    pub fn flip_info(&self) -> FlipInfo {
        let result = sprite_flip(self.flip_h, self.flip_v, self.origin_x, self.origin_y);
        FlipInfo {
            scale_x: result["scale_x"].as_f64().unwrap_or(1.0) as f32,
            scale_y: result["scale_y"].as_f64().unwrap_or(1.0) as f32,
            is_flipped: result["is_flipped"].as_bool().unwrap_or(false),
        }
    }

    /// State machine (usa action_assets::animation_state_machine)
    pub fn state_info(&self) -> StateInfo {
        let clip_names: Vec<String> = self.clips.keys().cloned().collect();
        let durations: Vec<f64> = clip_names.iter()
            .filter_map(|n| self.clips.get(n).map(|c| c.total_duration()))
            .collect();

        let current = self.current_clip.clone().unwrap_or_default();
        let result = animation_state_machine(&current, &clip_names, &durations, self.time_in_clip, "");

        StateInfo {
            state: result["state"].as_str().unwrap_or("").to_string(),
            progress: result["progress"].as_f64().unwrap_or(0.0),
            time_in_state: result["time_in_state"].as_f64().unwrap_or(0.0),
            duration: result["duration"].as_f64().unwrap_or(1.0),
            transitioning: result["transitioning"].as_bool().unwrap_or(false),
            next_state: result["next_state"].as_str().unwrap_or("").to_string(),
        }
    }

    /// ¿Animación terminada?
    pub fn is_finished(&self) -> bool {
        if let Some(ref name) = self.current_clip {
            if let Some(clip) = self.clips.get(name) {
                return clip.is_finished(self.time_in_clip);
            }
        }
        false
    }

    /// Renderizar sprite actual
    ///
    /// Usa `draw_texture_rec` de ry-gfx con source_rect del frame actual
    /// y flip transform.
    ///
    /// # Nota
    /// `draw_handle` es una referencia genérica al draw handle de ry-gfx.
    /// En implementación real se usa ry_gfx::DrawHandle o equivalente.
    pub fn render(
        &self,
        texture_id: &str,
        x: f32,
        y: f32,
        color: SpriteColor,
    ) -> RenderCommand {
        let rect = self.current_frame_rect();
        let flip = self.flip_info();

        let scale_x = flip.scale_x;
        let scale_y = flip.scale_y;

        RenderCommand {
            texture_id: texture_id.to_string(),
            source_x: rect.x,
            source_y: rect.y,
            source_w: rect.w,
            source_h: rect.h,
            dest_x: x,
            dest_y: y,
            dest_w: rect.w * scale_x.abs(),
            dest_h: rect.h * scale_y.abs(),
            color,
        }
    }
}

/// Info de flip calculada
#[derive(Debug, Clone)]
pub struct FlipInfo {
    pub scale_x: f32,
    pub scale_y: f32,
    pub is_flipped: bool,
}

/// Info de state machine
#[derive(Debug, Clone)]
pub struct StateInfo {
    pub state: String,
    pub progress: f64,
    pub time_in_state: f64,
    pub duration: f64,
    pub transitioning: bool,
    pub next_state: String,
}

/// Comando de renderizado — listo para enviar al backend
#[derive(Debug, Clone)]
pub struct RenderCommand {
    pub texture_id: String,
    pub source_x: f32,
    pub source_y: f32,
    pub source_w: f32,
    pub source_h: f32,
    pub dest_x: f32,
    pub dest_y: f32,
    pub dest_w: f32,
    pub dest_h: f32,
    pub color: SpriteColor,
}

// ============================================================================
// BLENDING — Transición suave entre clips
// ============================================================================

/// Blend entre dos clips de animación
///
/// # Ejemplo
/// ```rust,ignore
/// // Transición de idle a run
/// let blend = blend_clips(
///     idle_progress,  // 0.8 (80% del ciclo idle)
///     run_progress,   // 0.1 (recién empezó a correr)
///     0.5,            // blend_factor: 50% de transición
///     0.3,            // blend_duration: 300ms
///     0.15,           // t: a mitad de la transición
/// );
/// ```
pub fn blend_clips(
    state_a_progress: f64,
    state_b_progress: f64,
    blend_factor: f64,
    blend_duration: f64,
    t: f64,
) -> BlendResult {
    let result = animation_blend(state_a_progress, state_b_progress, blend_factor, blend_duration, t);
    BlendResult {
        blended_progress: result["blended_progress"].as_f64().unwrap_or(0.0),
        blend_factor: result["blend_factor"].as_f64().unwrap_or(0.0),
        is_complete: result["is_complete"].as_bool().unwrap_or(false),
        progress: result["progress"].as_f64().unwrap_or(0.0),
    }
}

#[derive(Debug, Clone)]
pub struct BlendResult {
    pub blended_progress: f64,
    pub blend_factor: f64,
    pub is_complete: bool,
    pub progress: f64,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_sheet() {
        let sheet = SpriteSheet::new("hero.png", 64.0, 64.0, 8, 4);
        assert_eq!(sheet.total_frames, 32);

        let rect = sheet.get_frame_rect(5);
        // frame 5: row = 5/8 = 0, col = 5%8 = 5 → x=320, y=0
        assert_eq!(rect.x, 320.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.w, 64.0);
        assert_eq!(rect.h, 64.0);

        // frame 13: row = 13/8 = 1, col = 13%8 = 5 → x=320, y=64
        let rect2 = sheet.get_frame_rect(13);
        assert_eq!(rect2.x, 320.0);
        assert_eq!(rect2.y, 64.0);
    }

    #[test]
    fn test_animation_clip_loop() {
        let clip = AnimationClip::new("idle", 0..4, 0.15, LoopMode::Loop);
        assert!((clip.total_duration() - 0.6).abs() < 0.01); // 4 * 0.15

        let frame = clip.get_frame(0.35);
        assert!(frame >= 0 && frame < 4);
    }

    #[test]
    fn test_animation_clip_once() {
        let clip = AnimationClip::new("jump", 0..4, 0.1, LoopMode::Once);
        assert!(clip.is_finished(0.5));
        assert!(!clip.is_finished(0.2));
    }

    #[test]
    fn test_animated_sprite() {
        let sheet = SpriteSheet::new("hero.png", 64.0, 64.0, 8, 4);
        let mut sprite = AnimatedSprite::new(sheet);

        sprite.add_clip("idle", 0..4, 0.15, LoopMode::Loop);
        sprite.add_clip("run", 4..12, 0.1, LoopMode::Loop);

        sprite.play("idle");
        sprite.update(0.2); // 0.2s = frame 1 (0.2 / 0.15 = 1.33)

        assert_eq!(sprite.current_clip, Some("idle".to_string()));
        assert!(sprite.current_frame() >= 1);
    }

    #[test]
    fn test_blend_clips() {
        let result = blend_clips(0.5, 0.0, 1.0, 0.5, 0.25);
        assert!(result.blended_progress > 0.0);
        assert!(!result.is_complete);
    }

    #[test]
    fn test_flip_info() {
        let sheet = SpriteSheet::new("hero.png", 64.0, 64.0, 8, 4);
        let mut sprite = AnimatedSprite::new(sheet);
        sprite.flip_horizontal(true);

        let flip = sprite.flip_info();
        assert!(flip.is_flipped);
        assert!((flip.scale_x - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn test_render_command() {
        let sheet = SpriteSheet::new("hero.png", 64.0, 64.0, 8, 4);
        let mut sprite = AnimatedSprite::new(sheet);
        sprite.add_clip("idle", 0..4, 0.15, LoopMode::Loop);
        sprite.play("idle");
        sprite.update(0.1);

        let cmd = sprite.render("hero.png", 100.0, 200.0, SpriteColor::blanco());
        assert_eq!(cmd.texture_id, "hero.png");
        assert_eq!(cmd.dest_x, 100.0);
        assert_eq!(cmd.dest_y, 200.0);
        assert!(cmd.source_w > 0.0);
    }
}
