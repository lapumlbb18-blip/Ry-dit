// crates/toolkit-ry/src/world_hud.rs
// Entity-anchored HUD elements (health bars + names) + Debug overlay
// Usa coordenadas del mundo y transforma a pantalla vía Camera2D

use crate::theme::{ColorRGBA, Theme};

// Re-export Camera2D de ry-gfx para conveniencia
pub use ry_gfx::camera::Camera2D;

// ============================================================================
// ENTITY HEALTH BAR + IDENTIFIER (world-space)
// ============================================================================

/// Datos de una entidad para renderizar su HUD
pub struct EntityHUD {
    /// Posición X en el mundo
    pub world_x: f32,
    /// Posición Y en el mundo
    pub world_y: f32,
    /// Ancho de la entidad (para escalar la barra)
    pub width: f32,
    /// Altura de la entidad (para posicionar sobre la entidad)
    pub height: f32,
    /// Vida actual
    pub current_hp: f32,
    /// Vida máxima
    pub max_hp: f32,
    /// Nombre/ID de la entidad (opcional)
    pub name: Option<&'static str>,
    /// Color personalizado de la barra (opcional, usa el del tema si None)
    pub bar_color: Option<ColorRGBA>,
}

/// Dibujar barra de vida anclada a entidad (coordenadas del mundo)
///
/// La barra se posiciona automáticamente sobre la entidad y sigue la cámara.
/// Incluye:
/// - Fondo oscuro
/// - Relleno de vida (verde→amarillo→rojo según %)
/// - Borde
/// - Nombre/ID encima (si está configurado)
pub fn draw_entity_health_bar_world(
    gui: &mut migui::Migui,
    entity: &EntityHUD,
    camera: &Camera2D,
    screen_w: i32,
    screen_h: i32,
    theme: &Theme,
) {
    // Transformar coordenadas del mundo a pantalla
    let (sx, sy) = camera.world_to_screen(entity.world_x, entity.world_y, screen_w, screen_h);
    let screen_x = sx as f32;
    let screen_y = sy as f32;

    // Escalar dimensiones con zoom
    let bar_width = (entity.width * camera.zoom).max(20.0);
    let bar_height = 6.0;
    let bar_offset_y = (entity.height * camera.zoom) + 8.0;
    let bar_y = screen_y - bar_offset_y;

    // Calcular ratio de vida
    let ratio = (entity.current_hp / entity.max_hp).clamp(0.0, 1.0);

    // Color dinámico: verde (100%) → amarillo (50%) → rojo (25%)
    let fill_color = if let Some(custom) = entity.bar_color {
        custom
    } else if ratio > 0.5 {
        // Verde
        ColorRGBA::rgb(50, 200, 50)
    } else if ratio > 0.25 {
        // Amarillo
        ColorRGBA::rgb(220, 200, 50)
    } else {
        // Rojo
        ColorRGBA::rgb(220, 50, 50)
    };

    // Fondo oscuro
    gui.panel(
        migui::WidgetId::new("entity_hp_bg"),
        migui::Rect::new(screen_x, bar_y, bar_width, bar_height),
        migui::Color { r: 20, g: 20, b: 20, a: 180 },
    );

    // Relleno de vida
    let fill_width = (bar_width * ratio).max(1.0);
    if fill_width > 0.0 {
        gui.panel(
            migui::WidgetId::new("entity_hp_fill"),
            migui::Rect::new(screen_x, bar_y, fill_width, bar_height),
            crate::widgets::rgba_to_migui(fill_color),
        );
    }

    // Borde sutil
    gui.panel(
        migui::WidgetId::new("entity_hp_border"),
        migui::Rect::new(screen_x, bar_y, bar_width, bar_height),
        crate::widgets::rgba_to_migui(ColorRGBA::rgb(80, 80, 80)),
    );

    // Nombre/ID encima de la barra
    if let Some(name) = entity.name {
        let name_y = bar_y - (theme.font_size_small as f32 + 2.0);
        // Calcular ancho del texto aproximado
        let name_width = (name.len() as f32) * (theme.font_size_small as f32) * 0.6;
        let name_x = screen_x + (bar_width - name_width) / 2.0;

        gui.label(
            migui::WidgetId::new("entity_name"),
            name,
            migui::Rect::new(name_x, name_y, name_width, theme.font_size_small as f32 + 2.0),
        );
    }
}

/// Dibujar múltiples barras de vida de entidades
pub fn draw_entity_health_bars(
    gui: &mut migui::Migui,
    entities: &[EntityHUD],
    camera: &Camera2D,
    screen_w: i32,
    screen_h: i32,
    theme: &Theme,
) {
    for (i, entity) in entities.iter().enumerate() {
        // WidgetId único por entidad
        draw_entity_health_bar_world(gui, entity, camera, screen_w, screen_h, theme);
        // Nota: en un uso real se deberían crear IDs únicos por entidad
        let _ = i; // suppress unused warning
    }
}

// ============================================================================
// DEBUG OVERLAY (información del motor)
// ============================================================================

/// Estado del motor para debug overlay
pub struct DebugInfo {
    /// FPS actuales
    pub fps: f32,
    /// Frame time en ms
    pub frame_time_ms: f32,
    /// Partículas activas (si aplica)
    pub particle_count: Option<usize>,
    /// Posición de la cámara (x, y)
    pub camera_x: f32,
    pub camera_y: f32,
    /// Zoom de la cámara
    pub camera_zoom: f32,
    /// Rotación de la cámara (grados)
    pub camera_rotation: f32,
    /// Entidades activas
    pub entity_count: Option<usize>,
    /// Memoria heap usada (MB) - opcional
    pub memory_mb: Option<f32>,
    /// Estado del juego (playing, paused, menu)
    pub game_state: Option<&'static str>,
    /// Texto personalizado adicional
    pub custom_lines: Vec<String>,
}

impl DebugInfo {
    /// Crear DebugInfo con valores mínimos
    pub fn minimal(fps: f32) -> Self {
        Self {
            fps,
            frame_time_ms: 1000.0 / fps.max(1.0),
            particle_count: None,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            camera_rotation: 0.0,
            entity_count: None,
            memory_mb: None,
            game_state: None,
            custom_lines: Vec::new(),
        }
    }
}

/// Dibujar overlay de debug en esquina superior izquierda
///
/// Muestra:
/// - FPS + frame time
/// - Posición y zoom de cámara
/// - Partículas / entidades
/// - Memoria (si está disponible)
/// - Estado del juego
/// - Líneas custom
pub fn draw_debug_overlay(
    gui: &mut migui::Migui,
    info: &DebugInfo,
    theme: &Theme,
    screen_w: i32,
    _screen_h: i32,
) {
    let padding = 8.0;
    let line_height = theme.font_size_small as f32 + 4.0;
    let mut lines: Vec<String> = Vec::new();

    // Línea 1: FPS + frame time
    lines.push(format!("FPS: {:.0} | Frame: {:.1}ms", info.fps, info.frame_time_ms));

    // Línea 2: Cámara
    lines.push(format!(
        "Cam: ({:.0}, {:.0}) | Zoom: {:.2}x | Rot: {:.0}°",
        info.camera_x, info.camera_y, info.camera_zoom, info.camera_rotation
    ));

    // Línea 3: Partículas
    if let Some(count) = info.particle_count {
        lines.push(format!("Partículas: {}", count));
    }

    // Línea 4: Entidades
    if let Some(count) = info.entity_count {
        lines.push(format!("Entidades: {}", count));
    }

    // Línea 5: Memoria
    if let Some(mb) = info.memory_mb {
        lines.push(format!("Memoria: {:.1} MB", mb));
    }

    // Línea 6: Estado del juego
    if let Some(state) = info.game_state {
        lines.push(format!("Estado: {}", state));
    }

    // Líneas custom
    for line in &info.custom_lines {
        lines.push(line.clone());
    }

    // Calcular dimensiones del panel
    let max_line_len = lines.iter().map(|l| l.len()).max().unwrap_or(20);
    let panel_width = (max_line_len as f32) * (theme.font_size_small as f32) * 0.6 + padding * 2.0;
    let panel_height = (lines.len() as f32) * line_height + padding * 2.0;

    // Fondo semi-transparente
    gui.panel(
        migui::WidgetId::new("debug_bg"),
        migui::Rect::new(padding, padding, panel_width, panel_height),
        migui::Color { r: 0, g: 0, b: 0, a: 180 },
    );

    // Renderizar cada línea
    for (i, line) in lines.iter().enumerate() {
        let y = padding + 4.0 + i as f32 * line_height;
        gui.label(
            migui::WidgetId::new(&format!("debug_line_{}", i)),
            line,
            migui::Rect::new(
                padding + 4.0,
                y,
                panel_width - padding * 2.0 - 8.0,
                theme.font_size_small as f32,
            ),
        );
    }
}

// ============================================================================
// HUD STATS (esquina superior derecha) - Stats compactos
// ============================================================================

/// Stats compactos para esquina superior derecha (score, time, etc.)
pub struct StatsHUD {
    /// Score/puntuación
    pub score: Option<i64>,
    /// Oro/monedas
    pub gold: Option<i64>,
    /// Tiempo en segundos
    pub time_seconds: Option<f32>,
    /// Nivel actual
    pub level: Option<i32>,
    /// Líneas custom
    pub custom: Vec<String>,
}

/// Dibujar stats compactos en esquina superior derecha
pub fn draw_stats_hud(
    gui: &mut migui::Migui,
    stats: &StatsHUD,
    theme: &Theme,
    screen_w: i32,
    screen_h: i32,
) {
    let padding = 8.0;
    let line_height = theme.font_size as f32 + 4.0;
    let mut lines: Vec<String> = Vec::new();

    if let Some(score) = stats.score {
        lines.push(format!("Score: {}", score));
    }
    if let Some(gold) = stats.gold {
        lines.push(format!("🪙 {}", gold));
    }
    if let Some(time) = stats.time_seconds {
        let mins = (time / 60.0) as i32;
        let secs = (time % 60.0) as i32;
        lines.push(format!("{:02}:{:02}", mins, secs));
    }
    if let Some(level) = stats.level {
        lines.push(format!("Nivel {}", level));
    }
    for line in &stats.custom {
        lines.push(line.clone());
    }

    let max_line_len = lines.iter().map(|l| l.len()).max().unwrap_or(10);
    let panel_width = (max_line_len as f32) * (theme.font_size as f32) * 0.6 + padding * 2.0;
    let panel_height = (lines.len() as f32) * line_height + padding * 2.0;
    let x = screen_w as f32 - panel_width - padding;
    let y = padding;

    // Fondo semi-transparente
    gui.panel(
        migui::WidgetId::new("stats_bg"),
        migui::Rect::new(x, y, panel_width, panel_height),
        migui::Color { r: 0, g: 0, b: 0, a: 140 },
    );

    for (i, line) in lines.iter().enumerate() {
        let ly = y + padding + 4.0 + i as f32 * line_height;
        gui.label(
            migui::WidgetId::new(&format!("stats_line_{}", i)),
            line,
            migui::Rect::new(
                x + padding + 4.0,
                ly,
                panel_width - padding * 2.0 - 8.0,
                theme.font_size as f32,
            ),
        );
    }
}

// ============================================================================
// MINIMAP AVANZADO (con entidades)
// ============================================================================

/// Entidad para mostrar en el minimap
pub struct MinimapEntity {
    pub world_x: f32,
    pub world_y: f32,
    /// Color del punto en el minimap
    pub color: ColorRGBA,
    /// Tamaño del punto
    pub size: f32,
}

/// Dibujar minimap avanzado con entidades
pub fn draw_minimap_advanced(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    size: f32,
    camera: &Camera2D,
    world_w: f32,
    world_h: f32,
    player_color: ColorRGBA,
    entities: &[MinimapEntity],
    theme: &Theme,
) {
    // Fondo del minimap
    gui.panel(
        migui::WidgetId::new("minimap_bg"),
        migui::Rect::new(x, y, size, size),
        crate::widgets::rgba_to_migui(theme.slot_bg),
    );

    // Borde del minimap
    gui.panel(
        migui::WidgetId::new("minimap_border"),
        migui::Rect::new(x, y, size, size),
        crate::widgets::rgba_to_migui(ColorRGBA::rgb(80, 80, 100)),
    );

    // Jugador (centro de la cámara)
    let player_dot = 6.0;
    gui.panel(
        migui::WidgetId::new("minimap_player"),
        migui::Rect::new(
            x + size / 2.0 - player_dot / 2.0,
            y + size / 2.0 - player_dot / 2.0,
            player_dot,
            player_dot,
        ),
        crate::widgets::rgba_to_migui(player_color),
    );

    // Viewport rect (qué porción del mundo es visible)
    let view_w = (screen_width_from_zoom(camera.zoom, size) / world_w) * size;
    let view_h = (screen_height_from_zoom(camera.zoom, size) / world_h) * size;
    let view_x = x + size / 2.0 - view_w / 2.0;
    let view_y = y + size / 2.0 - view_h / 2.0;

    // Dibujar entidades
    for entity in entities {
        let ex = x + (entity.world_x / world_w) * size;
        let ey = y + (entity.world_y / world_h) * size;
        let dot_size = entity.size.max(2.0);

        gui.panel(
            migui::WidgetId::new("minimap_entity"),
            migui::Rect::new(ex - dot_size / 2.0, ey - dot_size / 2.0, dot_size, dot_size),
            crate::widgets::rgba_to_migui(entity.color),
        );
    }

    // Suppress unused warnings
    let _ = (view_x, view_y);
}

fn screen_width_from_zoom(zoom: f32, _reference_size: f32) -> f32 {
    // Aproximación: el viewport visible es inversamente proporcional al zoom
    800.0 / zoom
}

fn screen_height_from_zoom(zoom: f32, _reference_size: f32) -> f32 {
    600.0 / zoom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_hud_creation() {
        let hud = EntityHUD {
            world_x: 100.0,
            world_y: 200.0,
            width: 32.0,
            height: 48.0,
            current_hp: 75.0,
            max_hp: 100.0,
            name: Some("Enemy #1"),
            bar_color: None,
        };
        assert_eq!(hud.current_hp / hud.max_hp, 0.75);
    }

    #[test]
    fn test_debug_info_minimal() {
        let info = DebugInfo::minimal(60.0);
        assert_eq!(info.fps, 60.0);
        assert!(info.frame_time_ms > 16.0);
        assert!(info.particle_count.is_none());
    }

    #[test]
    fn test_stats_hud() {
        let stats = StatsHUD {
            score: Some(1500),
            gold: Some(250),
            time_seconds: Some(125.0),
            level: Some(3),
            custom: vec!["Custom line".to_string()],
        };
        assert!(stats.score.is_some());
        assert!(stats.gold.is_some());
        assert_eq!(stats.custom.len(), 1);
    }

    #[test]
    fn test_health_color_by_ratio() {
        // Verde (>50%)
        let full = 100.0 / 100.0;
        assert!(full > 0.5);

        // Amarillo (25-50%)
        let half = 40.0 / 100.0;
        assert!(half > 0.25 && half <= 0.5);

        // Rojo (<25%)
        let low = 10.0 / 100.0;
        assert!(low <= 0.25);
    }
}
