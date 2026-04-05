// crates/toolkit-ry/src/widgets.rs
// Widgets de juego para Ry-Dit — HUD, Menús, Diálogos, Inventario
// Construido sobre migui (immediate mode) + Theme system

use crate::theme::{ColorRGBA, Theme};

// ============================================================================
// UTILIDADES DE COLOR
// ============================================================================

/// Convertir ColorRGBA a migui Color
pub fn rgba_to_migui(c: ColorRGBA) -> migui::Color {
    migui::Color {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
    }
}

/// Convertir ColorRGBA a SDL2 Color
pub fn rgba_to_sdl2(c: ColorRGBA) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGBA(c.r, c.g, c.b, c.a)
}

// ============================================================================
// HUD WIDGETS
// ============================================================================

/// Barra de vida (HP)
/// Retorna true si se renderizó correctamente
pub fn draw_health_bar(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    current: f32,
    max: f32,
    theme: &Theme,
) {
    let ratio = (current / max).clamp(0.0, 1.0);

    // Fondo de la barra
    gui.panel(
        migui::WidgetId::new("hp_bg"),
        migui::Rect::new(x, y, width, height),
        rgba_to_migui(theme.health_bar_bg),
    );

    // Relleno de vida
    let fill_width = width * ratio;
    if fill_width > 0.0 {
        gui.panel(
            migui::WidgetId::new("hp_fill"),
            migui::Rect::new(x, y, fill_width, height),
            rgba_to_migui(theme.health_bar_fill),
        );
    }

    // Borde
    if theme.border_width > 0.0 {
        gui.panel(
            migui::WidgetId::new("hp_border"),
            migui::Rect::new(x, y, width, height),
            rgba_to_migui(ColorRGBA::transparent()),
        );
    }

    // Texto "HP: 75/100"
    let label = format!("HP: {}/{}", current as i32, max as i32);
    gui.label(
        migui::WidgetId::new("hp_label"),
        &label,
        migui::Rect::new(x + 4.0, y + 2.0, width - 8.0, height),
    );
}

/// Barra de maná (MP)
pub fn draw_mana_bar(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    current: f32,
    max: f32,
    theme: &Theme,
) {
    let ratio = (current / max).clamp(0.0, 1.0);

    gui.panel(
        migui::WidgetId::new("mp_bg"),
        migui::Rect::new(x, y, width, height),
        rgba_to_migui(theme.mana_bar_bg),
    );

    let fill_width = width * ratio;
    if fill_width > 0.0 {
        gui.panel(
            migui::WidgetId::new("mp_fill"),
            migui::Rect::new(x, y, fill_width, height),
            rgba_to_migui(theme.mana_bar_fill),
        );
    }

    let label = format!("MP: {}/{}", current as i32, max as i32);
    gui.label(
        migui::WidgetId::new("mp_label"),
        &label,
        migui::Rect::new(x + 4.0, y + 2.0, width - 8.0, height),
    );
}

/// Barra de experiencia (XP)
pub fn draw_xp_bar(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    current: f32,
    max: f32,
    theme: &Theme,
) {
    let ratio = (current / max).clamp(0.0, 1.0);

    gui.panel(
        migui::WidgetId::new("xp_bg"),
        migui::Rect::new(x, y, width, height),
        rgba_to_migui(theme.xp_bar_bg),
    );

    let fill_width = width * ratio;
    if fill_width > 0.0 {
        gui.panel(
            migui::WidgetId::new("xp_fill"),
            migui::Rect::new(x, y, fill_width, height),
            rgba_to_migui(theme.xp_bar_fill),
        );
    }

    let label = format!("XP: {}/{}", current as i32, max as i32);
    gui.label(
        migui::WidgetId::new("xp_label"),
        &label,
        migui::Rect::new(x + 4.0, y + 2.0, width - 8.0, height),
    );
}

/// Display de puntuación
pub fn draw_score(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    score: i64,
    theme: &Theme,
) {
    let label = format!("Score: {}", score);
    gui.label(
        migui::WidgetId::new("score"),
        &label,
        migui::Rect::new(x, y, 200.0, theme.font_size as f32 + 8.0),
    );
}

/// Display de monedas/oro
pub fn draw_gold(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    gold: i64,
    theme: &Theme,
) {
    let label = format!("🪙 {}", gold);
    gui.label(
        migui::WidgetId::new("gold"),
        &label,
        migui::Rect::new(x, y, 150.0, theme.font_size as f32 + 8.0),
    );
}

/// Display de tiempo
pub fn draw_timer(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    seconds: f32,
    theme: &Theme,
) {
    let mins = (seconds / 60.0) as i32;
    let secs = (seconds % 60.0) as i32;
    let label = format!("{:02}:{:02}", mins, secs);
    gui.label(
        migui::WidgetId::new("timer"),
        &label,
        migui::Rect::new(x, y, 80.0, theme.font_size as f32 + 8.0),
    );
}

/// HUD completo (vida + maná + score)
pub fn draw_full_hud(
    gui: &mut migui::Migui,
    hp: f32,
    max_hp: f32,
    mp: f32,
    max_mp: f32,
    xp: f32,
    max_xp: f32,
    score: i64,
    theme: &Theme,
) {
    let bar_w = 200.0;
    let bar_h = 18.0;
    let x = 20.0;
    let mut y = 20.0;

    draw_health_bar(gui, x, y, bar_w, bar_h, hp, max_hp, theme);
    y += bar_h + 4.0;

    draw_mana_bar(gui, x, y, bar_w, bar_h, mp, max_mp, theme);
    y += bar_h + 4.0;

    draw_xp_bar(gui, x, y, bar_w, bar_h, xp, max_xp, theme);

    draw_score(gui, 600.0, 20.0, score, theme);
}

// ============================================================================
// MENU WIDGETS
// ============================================================================

/// Menú principal — retorna índice de opción seleccionada (-1 = ninguna)
pub fn draw_main_menu(
    gui: &mut migui::Migui,
    title: &str,
    options: &[&str],
    theme: &Theme,
    hover_state: &mut i32,
) -> i32 {
    let screen_w = 800.0;
    let screen_h = 600.0;

    // Fondo semi-transparente
    gui.panel(
        migui::WidgetId::new("menu_bg"),
        migui::Rect::new(0.0, 0.0, screen_w, screen_h),
        rgba_to_migui(theme.menu_bg),
    );

    // Título
    gui.label(
        migui::WidgetId::new("menu_title"),
        title,
        migui::Rect::new(
            screen_w / 2.0 - 150.0,
            80.0,
            300.0,
            theme.font_size_title as f32 + 10.0,
        ),
    );

    // Opciones
    let item_h = 40.0;
    let total_h = options.len() as f32 * (item_h + theme.spacing);
    let mut y = (screen_h - total_h) / 2.0;

    for (i, option) in options.iter().enumerate() {
        let id = format!("menu_opt_{}", i);
        let is_hovered = *hover_state == i as i32;

        let bg = if is_hovered {
            rgba_to_migui(theme.menu_item_hover)
        } else {
            rgba_to_migui(theme.menu_item_bg)
        };

        gui.panel(
            migui::WidgetId::new(&format!("{}_bg", id)),
            migui::Rect::new(screen_w / 2.0 - 120.0, y, 240.0, item_h),
            bg,
        );

        if gui.button(
            migui::WidgetId::new(&id),
            migui::Rect::new(screen_w / 2.0 - 110.0, y + 4.0, 220.0, item_h - 8.0),
            option,
        ) {
            return i as i32;
        }

        y += item_h + theme.spacing;
    }

    -1
}

/// Menú de pausa — retorna opción seleccionada
pub fn draw_pause_menu(
    gui: &mut migui::Migui,
    theme: &Theme,
    hover_state: &mut i32,
) -> i32 {
    draw_main_menu(
        gui,
        "PAUSA",
        &["Continuar", "Opciones", "Guardar", "Salir"],
        theme,
        hover_state,
    )
}

/// Game Over screen
pub fn draw_game_over(
    gui: &mut migui::Migui,
    score: i64,
    theme: &Theme,
    hover_state: &mut i32,
) -> i32 {
    draw_main_menu(
        gui,
        &format!("GAME OVER\nScore: {}", score),
        &["Reiniciar", "Menú Principal"],
        theme,
        hover_state,
    )
}

/// Menú de opciones
pub fn draw_options_menu(
    gui: &mut migui::Migui,
    theme: &Theme,
    volume: &mut f32,
    fullscreen: &mut bool,
    hover_state: &mut i32,
) {
    let screen_w = 800.0;
    let screen_h = 600.0;

    gui.panel(
        migui::WidgetId::new("options_bg"),
        migui::Rect::new(0.0, 0.0, screen_w, screen_h),
        rgba_to_migui(theme.menu_bg),
    );

    gui.label(
        migui::WidgetId::new("options_title"),
        "OPCIONES",
        migui::Rect::new(
            screen_w / 2.0 - 100.0,
            40.0,
            200.0,
            theme.font_size_title as f32 + 10.0,
        ),
    );

    // Volumen
    gui.label(
        migui::WidgetId::new("vol_label"),
        &format!("Volumen: {:.0}%", *volume * 100.0),
        migui::Rect::new(200.0, 150.0, 200.0, 30.0),
    );

    *volume = gui.slider(
        migui::WidgetId::new("vol_slider"),
        *volume,
        0.0,
        1.0,
        migui::Rect::new(420.0, 150.0, 200.0, 30.0),
    );

    // Fullscreen
    gui.checkbox(
        migui::WidgetId::new("fs_check"),
        "Pantalla Completa",
        fullscreen,
        migui::Rect::new(200.0, 200.0, 300.0, 30.0),
    );

    // Botón volver
    if gui.button(
        migui::WidgetId::new("options_back"),
        migui::Rect::new(screen_w / 2.0 - 60.0, 400.0, 120.0, 40.0),
        "Volver",
    ) {
        *hover_state = 99;
    }
}

// ============================================================================
/// Diálogo de NPC
pub fn draw_dialog(
    gui: &mut migui::Migui,
    npc_name: &str,
    text: &str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    theme: &Theme,
) {
    // Fondo del diálogo
    gui.panel(
        migui::WidgetId::new("dialog_bg"),
        migui::Rect::new(x, y, width, height),
        rgba_to_migui(theme.dialog_bg),
    );

    // Borde
    if theme.border_width > 0.0 {
        gui.panel(
            migui::WidgetId::new("dialog_border"),
            migui::Rect::new(x, y, width, height),
            rgba_to_migui(ColorRGBA::transparent()),
        );
    }

    // Nombre del NPC
    gui.label(
        migui::WidgetId::new("dialog_name"),
        npc_name,
        migui::Rect::new(x + 10.0, y + 4.0, width - 20.0, theme.font_size_small as f32 + 6.0),
    );

    // Texto del diálogo
    gui.label(
        migui::WidgetId::new("dialog_text"),
        text,
        migui::Rect::new(
            x + 10.0,
            y + theme.font_size_small as f32 + 12.0,
            width - 20.0,
            height - theme.font_size_small as f32 - 20.0,
        ),
    );
}

/// Message box (confirmación)
pub fn draw_message_box(
    gui: &mut migui::Migui,
    title: &str,
    message: &str,
    buttons: &[&str],
    theme: &Theme,
) -> i32 {
    let screen_w = 800.0;
    let screen_h = 600.0;
    let box_w = 400.0;
    let box_h = 200.0;
    let x = (screen_w - box_w) / 2.0;
    let y = (screen_h - box_h) / 2.0;

    // Fondo
    gui.panel(
        migui::WidgetId::new("msg_bg"),
        migui::Rect::new(0.0, 0.0, screen_w, screen_h),
        rgba_to_migui(ColorRGBA { r: 0, g: 0, b: 0, a: 128 }),
    );

    gui.panel(
        migui::WidgetId::new("msg_box"),
        migui::Rect::new(x, y, box_w, box_h),
        rgba_to_migui(theme.dialog_bg),
    );

    // Título
    gui.label(
        migui::WidgetId::new("msg_title"),
        title,
        migui::Rect::new(x + 10.0, y + 8.0, box_w - 20.0, theme.font_size as f32 + 6.0),
    );

    // Mensaje
    gui.label(
        migui::WidgetId::new("msg_text"),
        message,
        migui::Rect::new(x + 10.0, y + 40.0, box_w - 20.0, 80.0),
    );

    // Botones
    let btn_w = 100.0;
    let btn_h = 35.0;
    let total_w = buttons.len() as f32 * (btn_w + 10.0);
    let mut btn_x = x + (box_w - total_w) / 2.0;
    let btn_y = y + box_h - 50.0;

    for (i, label) in buttons.iter().enumerate() {
        if gui.button(
            migui::WidgetId::new(&format!("msg_btn_{}", i)),
            migui::Rect::new(btn_x, btn_y, btn_w, btn_h),
            label,
        ) {
            return i as i32;
        }
        btn_x += btn_w + 10.0;
    }

    -1
}

// ============================================================================
// INVENTARIO
// ============================================================================

/// Slot de inventario individual
pub fn draw_inventory_slot(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    size: f32,
    item_name: Option<&str>,
    count: i32,
    is_selected: bool,
    theme: &Theme,
) {
    let bg = if is_selected {
        rgba_to_migui(theme.slot_hover)
    } else {
        rgba_to_migui(theme.slot_bg)
    };

    gui.panel(
        migui::WidgetId::new(&format!("slot_{}_{}", x as i32, y as i32)),
        migui::Rect::new(x, y, size, size),
        bg,
    );

    if let Some(name) = item_name {
        gui.label(
            migui::WidgetId::new(&format!("slot_label_{}_{}", x as i32, y as i32)),
            name,
            migui::Rect::new(x + 2.0, y + 2.0, size - 4.0, size - 14.0),
        );

        if count > 1 {
            gui.label(
                migui::WidgetId::new(&format!("slot_count_{}_{}", x as i32, y as i32)),
                &format!("x{}", count),
                migui::Rect::new(
                    x + size - 30.0,
                    y + size - 14.0,
                    28.0,
                    theme.font_size_small as f32,
                ),
            );
        }
    }
}

/// Grid de inventario completo
pub fn draw_inventory_grid(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    cols: usize,
    rows: usize,
    slot_size: f32,
    spacing: f32,
    items: &[Option<(String, i32)>],
    selected_slot: &mut usize,
    theme: &Theme,
) -> Option<usize> {
    // Fondo del panel de inventario
    let panel_w = cols as f32 * (slot_size + spacing) + theme.padding * 2.0;
    let panel_h = rows as f32 * (slot_size + spacing) + theme.padding * 2.0 + 30.0;

    gui.panel(
        migui::WidgetId::new("inv_panel"),
        migui::Rect::new(x, y, panel_w, panel_h),
        rgba_to_migui(theme.dialog_bg),
    );

    gui.label(
        migui::WidgetId::new("inv_title"),
        "INVENTARIO",
        migui::Rect::new(
            x + theme.padding,
            y + 4.0,
            panel_w - theme.padding * 2.0,
            theme.font_size as f32 + 6.0,
        ),
    );

    let slot_y = y + theme.padding + 30.0;

    for row in 0..rows {
        for col in 0..cols {
            let idx = row * cols + col;
            let slot_x = x + theme.padding + col as f32 * (slot_size + spacing);
            let slot_y = slot_y + row as f32 * (slot_size + spacing);

            let item = items.get(idx).and_then(|i| i.as_ref());
            let (item_name, count) = match item {
                Some((name, c)) => (Some(name.as_str()), *c),
                None => (None, 0),
            };

            let is_selected = idx == *selected_slot;

            if gui.button(
                migui::WidgetId::new(&format!("inv_slot_{}", idx)),
                migui::Rect::new(slot_x, slot_y, slot_size, slot_size),
                "",
            ) {
                *selected_slot = idx;
                return Some(idx);
            }

            draw_inventory_slot(
                gui,
                slot_x,
                slot_y,
                slot_size,
                item_name,
                count,
                is_selected,
                theme,
            );
        }
    }

    None
}

// ============================================================================
// NOTIFICACIONES
// ============================================================================

/// Notificación temporal (toast)
pub fn draw_notification(
    gui: &mut migui::Migui,
    text: &str,
    x: f32,
    y: f32,
    theme: &Theme,
) {
    let text_w = text.len() as f32 * theme.font_size as f32 * 0.5;
    let h = theme.font_size as f32 + 16.0;

    gui.panel(
        migui::WidgetId::new("notif_bg"),
        migui::Rect::new(x, y, text_w + 20.0, h),
        rgba_to_migui(theme.menu_item_hover),
    );

    gui.label(
        migui::WidgetId::new("notif_text"),
        text,
        migui::Rect::new(x + 10.0, y + 8.0, text_w, theme.font_size as f32),
    );
}

// ============================================================================
// MINIMAP
// ============================================================================

/// Minimapa simple
pub fn draw_minimap(
    gui: &mut migui::Migui,
    x: f32,
    y: f32,
    size: f32,
    player_px: f32,
    player_py: f32,
    world_w: f32,
    world_h: f32,
    theme: &Theme,
) {
    // Fondo del minimap
    gui.panel(
        migui::WidgetId::new("minimap_bg"),
        migui::Rect::new(x, y, size, size),
        rgba_to_migui(theme.slot_bg),
    );

    // Jugador (punto)
    let dot_x = x + (player_px / world_w) * size;
    let dot_y = y + (player_py / world_h) * size;
    let dot_size = 6.0;

    gui.panel(
        migui::WidgetId::new("minimap_player"),
        migui::Rect::new(dot_x - dot_size / 2.0, dot_y - dot_size / 2.0, dot_size, dot_size),
        rgba_to_migui(theme.health_bar_fill),
    );
}

// ============================================================================
// LOADING SCREEN
// ============================================================================

/// Pantalla de carga
pub fn draw_loading(
    gui: &mut migui::Migui,
    text: &str,
    progress: f32,
    theme: &Theme,
) {
    let screen_w = 800.0;
    let screen_h = 600.0;

    gui.panel(
        migui::WidgetId::new("load_bg"),
        migui::Rect::new(0.0, 0.0, screen_w, screen_h),
        rgba_to_migui(theme.bg_color),
    );

    gui.label(
        migui::WidgetId::new("load_text"),
        text,
        migui::Rect::new(
            screen_w / 2.0 - 100.0,
            250.0,
            200.0,
            theme.font_size as f32 + 8.0,
        ),
    );

    let bar_w = 300.0;
    let bar_h = 20.0;
    let bar_x = (screen_w - bar_w) / 2.0;
    let bar_y = 300.0;

    gui.panel(
        migui::WidgetId::new("load_bar_bg"),
        migui::Rect::new(bar_x, bar_y, bar_w, bar_h),
        rgba_to_migui(theme.slot_bg),
    );

    let fill_w = bar_w * progress.clamp(0.0, 1.0);
    gui.panel(
        migui::WidgetId::new("load_bar_fill"),
        migui::Rect::new(bar_x, bar_y, fill_w, bar_h),
        rgba_to_migui(theme.health_bar_fill),
    );

    let pct = format!("{:.0}%", progress * 100.0);
    gui.label(
        migui::WidgetId::new("load_pct"),
        &pct,
        migui::Rect::new(
            screen_w / 2.0 - 20.0,
            bar_y + bar_h + 8.0,
            40.0,
            theme.font_size_small as f32,
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let c = ColorRGBA::rgb(255, 128, 64);
        let migui_c = rgba_to_migui(c);
        assert_eq!(migui_c.r, 255);
        assert_eq!(migui_c.g, 128);
        assert_eq!(migui_c.b, 64);
    }

    #[test]
    fn test_sdl2_color_conversion() {
        let c = ColorRGBA::rgb(100, 150, 200);
        let sdl2_c = rgba_to_sdl2(c);
        assert_eq!(sdl2_c.r, 100);
        assert_eq!(sdl2_c.g, 150);
        assert_eq!(sdl2_c.b, 200);
    }

    #[test]
    fn test_rgba_transparent() {
        let c = ColorRGBA::transparent();
        assert_eq!(c.a, 0);
    }
}
