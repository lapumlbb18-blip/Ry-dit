// crates/toolkit-ry/src/theme.rs
// Themes predefinidos para UI de juegos Ry-Dit

/// Colores RGBA
#[derive(Debug, Clone, Copy)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn transparent() -> Self {
        Self { r: 0, g: 0, b: 0, a: 0 }
    }
}

/// Tema visual para toolkit-ry
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    // Colores base
    pub bg_color: ColorRGBA,
    pub panel_bg: ColorRGBA,
    pub panel_border: ColorRGBA,
    // Texto
    pub text_color: ColorRGBA,
    pub text_dim: ColorRGBA,
    pub text_highlight: ColorRGBA,
    // Widgets
    pub button_bg: ColorRGBA,
    pub button_hover: ColorRGBA,
    pub button_pressed: ColorRGBA,
    pub button_border: ColorRGBA,
    // HUD
    pub health_bar_bg: ColorRGBA,
    pub health_bar_fill: ColorRGBA,
    pub health_bar_border: ColorRGBA,
    pub mana_bar_bg: ColorRGBA,
    pub mana_bar_fill: ColorRGBA,
    pub xp_bar_bg: ColorRGBA,
    pub xp_bar_fill: ColorRGBA,
    // Inventário
    pub slot_bg: ColorRGBA,
    pub slot_hover: ColorRGBA,
    pub slot_border: ColorRGBA,
    // Diálogos
    pub dialog_bg: ColorRGBA,
    pub dialog_border: ColorRGBA,
    // Menú
    pub menu_bg: ColorRGBA,
    pub menu_item_bg: ColorRGBA,
    pub menu_item_hover: ColorRGBA,
    pub menu_item_selected: ColorRGBA,
    // Tipografía
    pub font_size: u32,
    pub font_size_small: u32,
    pub font_size_title: u32,
    // Espaciado
    pub padding: f32,
    pub spacing: f32,
    pub border_width: f32,
}

impl Theme {
    /// Tema oscuro (default) — estilo moderno
    pub fn dark() -> Self {
        Self {
            name: "Dark",
            bg_color: ColorRGBA::rgb(18, 18, 24),
            panel_bg: ColorRGBA::rgb(28, 28, 36),
            panel_border: ColorRGBA::rgb(60, 60, 80),
            text_color: ColorRGBA::rgb(230, 230, 240),
            text_dim: ColorRGBA::rgb(140, 140, 160),
            text_highlight: ColorRGBA::rgb(255, 255, 100),
            button_bg: ColorRGBA::rgb(50, 50, 70),
            button_hover: ColorRGBA::rgb(70, 70, 100),
            button_pressed: ColorRGBA::rgb(90, 90, 130),
            button_border: ColorRGBA::rgb(100, 100, 130),
            health_bar_bg: ColorRGBA::rgb(40, 20, 20),
            health_bar_fill: ColorRGBA::rgb(220, 50, 50),
            health_bar_border: ColorRGBA::rgb(180, 40, 40),
            mana_bar_bg: ColorRGBA::rgb(20, 20, 50),
            mana_bar_fill: ColorRGBA::rgb(50, 100, 220),
            xp_bar_bg: ColorRGBA::rgb(40, 40, 20),
            xp_bar_fill: ColorRGBA::rgb(220, 200, 50),
            slot_bg: ColorRGBA::rgb(35, 35, 45),
            slot_hover: ColorRGBA::rgb(50, 50, 65),
            slot_border: ColorRGBA::rgb(70, 70, 90),
            dialog_bg: ColorRGBA::rgb(25, 25, 35),
            dialog_border: ColorRGBA::rgb(80, 80, 100),
            menu_bg: ColorRGBA::rgb(22, 22, 30),
            menu_item_bg: ColorRGBA::rgb(30, 30, 42),
            menu_item_hover: ColorRGBA::rgb(50, 50, 70),
            menu_item_selected: ColorRGBA::rgb(70, 70, 100),
            font_size: 16,
            font_size_small: 12,
            font_size_title: 20,
            padding: 8.0,
            spacing: 4.0,
            border_width: 2.0,
        }
    }

    /// Tema claro — estilo limpio
    pub fn light() -> Self {
        Self {
            name: "Light",
            bg_color: ColorRGBA::rgb(240, 240, 245),
            panel_bg: ColorRGBA::rgb(255, 255, 255),
            panel_border: ColorRGBA::rgb(200, 200, 210),
            text_color: ColorRGBA::rgb(30, 30, 40),
            text_dim: ColorRGBA::rgb(120, 120, 140),
            text_highlight: ColorRGBA::rgb(200, 160, 0),
            button_bg: ColorRGBA::rgb(220, 220, 230),
            button_hover: ColorRGBA::rgb(200, 200, 215),
            button_pressed: ColorRGBA::rgb(180, 180, 200),
            button_border: ColorRGBA::rgb(170, 170, 185),
            health_bar_bg: ColorRGBA::rgb(230, 210, 210),
            health_bar_fill: ColorRGBA::rgb(220, 60, 60),
            health_bar_border: ColorRGBA::rgb(190, 50, 50),
            mana_bar_bg: ColorRGBA::rgb(210, 210, 240),
            mana_bar_fill: ColorRGBA::rgb(60, 110, 220),
            xp_bar_bg: ColorRGBA::rgb(240, 240, 210),
            xp_bar_fill: ColorRGBA::rgb(220, 200, 60),
            slot_bg: ColorRGBA::rgb(235, 235, 240),
            slot_hover: ColorRGBA::rgb(220, 220, 230),
            slot_border: ColorRGBA::rgb(200, 200, 210),
            dialog_bg: ColorRGBA::rgb(250, 250, 255),
            dialog_border: ColorRGBA::rgb(190, 190, 200),
            menu_bg: ColorRGBA::rgb(245, 245, 250),
            menu_item_bg: ColorRGBA::rgb(240, 240, 248),
            menu_item_hover: ColorRGBA::rgb(220, 220, 235),
            menu_item_selected: ColorRGBA::rgb(200, 200, 220),
            font_size: 16,
            font_size_small: 12,
            font_size_title: 20,
            padding: 8.0,
            spacing: 4.0,
            border_width: 1.0,
        }
    }

    /// Tema retro — estilo pixel art 8-bit
    pub fn retro() -> Self {
        Self {
            name: "Retro",
            bg_color: ColorRGBA::rgb(0, 0, 0),
            panel_bg: ColorRGBA::rgb(32, 32, 32),
            panel_border: ColorRGBA::rgb(255, 255, 255),
            text_color: ColorRGBA::rgb(0, 255, 0),
            text_dim: ColorRGBA::rgb(0, 180, 0),
            text_highlight: ColorRGBA::rgb(255, 255, 0),
            button_bg: ColorRGBA::rgb(64, 64, 64),
            button_hover: ColorRGBA::rgb(96, 96, 96),
            button_pressed: ColorRGBA::rgb(128, 128, 128),
            button_border: ColorRGBA::rgb(255, 255, 255),
            health_bar_bg: ColorRGBA::rgb(32, 0, 0),
            health_bar_fill: ColorRGBA::rgb(255, 0, 0),
            health_bar_border: ColorRGBA::rgb(255, 255, 255),
            mana_bar_bg: ColorRGBA::rgb(0, 0, 32),
            mana_bar_fill: ColorRGBA::rgb(0, 100, 255),
            xp_bar_bg: ColorRGBA::rgb(32, 32, 0),
            xp_bar_fill: ColorRGBA::rgb(255, 255, 0),
            slot_bg: ColorRGBA::rgb(48, 48, 48),
            slot_hover: ColorRGBA::rgb(80, 80, 80),
            slot_border: ColorRGBA::rgb(255, 255, 255),
            dialog_bg: ColorRGBA::rgb(0, 0, 64),
            dialog_border: ColorRGBA::rgb(255, 255, 255),
            menu_bg: ColorRGBA::rgb(0, 0, 0),
            menu_item_bg: ColorRGBA::rgb(0, 0, 0),
            menu_item_hover: ColorRGBA::rgb(0, 64, 0),
            menu_item_selected: ColorRGBA::rgb(0, 128, 0),
            font_size: 14,
            font_size_small: 10,
            font_size_title: 18,
            padding: 6.0,
            spacing: 3.0,
            border_width: 2.0,
        }
    }

    /// Tema neón — estilo cyberpunk con glow
    pub fn neon() -> Self {
        Self {
            name: "Neon",
            bg_color: ColorRGBA::rgb(5, 5, 15),
            panel_bg: ColorRGBA::rgb(10, 10, 30),
            panel_border: ColorRGBA::rgb(0, 255, 255),
            text_color: ColorRGBA::rgb(255, 255, 255),
            text_dim: ColorRGBA::rgb(100, 200, 255),
            text_highlight: ColorRGBA::rgb(255, 0, 255),
            button_bg: ColorRGBA::rgb(20, 20, 50),
            button_hover: ColorRGBA::rgb(40, 40, 100),
            button_pressed: ColorRGBA::rgb(60, 60, 150),
            button_border: ColorRGBA::rgb(0, 255, 255),
            health_bar_bg: ColorRGBA::rgb(30, 5, 5),
            health_bar_fill: ColorRGBA::rgb(255, 50, 50),
            health_bar_border: ColorRGBA::rgb(255, 100, 100),
            mana_bar_bg: ColorRGBA::rgb(5, 5, 30),
            mana_bar_fill: ColorRGBA::rgb(100, 150, 255),
            xp_bar_bg: ColorRGBA::rgb(30, 30, 5),
            xp_bar_fill: ColorRGBA::rgb(255, 255, 100),
            slot_bg: ColorRGBA::rgb(15, 15, 35),
            slot_hover: ColorRGBA::rgb(30, 30, 70),
            slot_border: ColorRGBA::rgb(0, 255, 255),
            dialog_bg: ColorRGBA::rgb(8, 8, 25),
            dialog_border: ColorRGBA::rgb(255, 0, 255),
            menu_bg: ColorRGBA::rgb(5, 5, 20),
            menu_item_bg: ColorRGBA::rgb(10, 10, 30),
            menu_item_hover: ColorRGBA::rgb(30, 30, 70),
            menu_item_selected: ColorRGBA::rgb(50, 50, 120),
            font_size: 16,
            font_size_small: 12,
            font_size_title: 22,
            padding: 10.0,
            spacing: 6.0,
            border_width: 1.0,
        }
    }

    /// Tema minimalista — ultra limpio
    pub fn minimal() -> Self {
        Self {
            name: "Minimal",
            bg_color: ColorRGBA::transparent(),
            panel_bg: ColorRGBA::rgb(20, 20, 25),
            panel_border: ColorRGBA::transparent(),
            text_color: ColorRGBA::rgb(240, 240, 245),
            text_dim: ColorRGBA::rgb(120, 120, 130),
            text_highlight: ColorRGBA::rgb(255, 255, 255),
            button_bg: ColorRGBA::transparent(),
            button_hover: ColorRGBA::rgb(40, 40, 50),
            button_pressed: ColorRGBA::rgb(60, 60, 70),
            button_border: ColorRGBA::transparent(),
            health_bar_bg: ColorRGBA::rgb(40, 20, 20),
            health_bar_fill: ColorRGBA::rgb(230, 60, 60),
            health_bar_border: ColorRGBA::transparent(),
            mana_bar_bg: ColorRGBA::rgb(20, 20, 50),
            mana_bar_fill: ColorRGBA::rgb(60, 120, 230),
            xp_bar_bg: ColorRGBA::rgb(40, 40, 20),
            xp_bar_fill: ColorRGBA::rgb(230, 210, 60),
            slot_bg: ColorRGBA::rgb(30, 30, 38),
            slot_hover: ColorRGBA::rgb(45, 45, 55),
            slot_border: ColorRGBA::transparent(),
            dialog_bg: ColorRGBA::rgb(15, 15, 22),
            dialog_border: ColorRGBA::transparent(),
            menu_bg: ColorRGBA::transparent(),
            menu_item_bg: ColorRGBA::transparent(),
            menu_item_hover: ColorRGBA::rgb(30, 30, 40),
            menu_item_selected: ColorRGBA::rgb(50, 50, 60),
            font_size: 15,
            font_size_small: 11,
            font_size_title: 19,
            padding: 6.0,
            spacing: 3.0,
            border_width: 0.0,
        }
    }

    /// Obtener todos los temas disponibles
    pub fn all() -> &'static [fn() -> Self] {
        &[Self::dark, Self::light, Self::retro, Self::neon, Self::minimal]
    }

    /// Obtener tema por nombre
    pub fn by_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dark" | "oscuro" => Self::dark(),
            "light" | "claro" => Self::light(),
            "retro" | "pixel" | "8bit" => Self::retro(),
            "neon" | "cyber" | "cyberpunk" => Self::neon(),
            "minimal" | "minimalist" | "min" => Self::minimal(),
            _ => Self::dark(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "Dark");
        assert_eq!(theme.bg_color.r, 18);
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.name, "Light");
        assert_eq!(theme.bg_color.r, 240);
    }

    #[test]
    fn test_theme_retro() {
        let theme = Theme::retro();
        assert_eq!(theme.name, "Retro");
        assert_eq!(theme.text_color.g, 255);
    }

    #[test]
    fn test_theme_neon() {
        let theme = Theme::neon();
        assert_eq!(theme.name, "Neon");
        assert_eq!(theme.panel_border.g, 255);
    }

    #[test]
    fn test_theme_minimal() {
        let theme = Theme::minimal();
        assert_eq!(theme.name, "Minimal");
        assert_eq!(theme.border_width, 0.0);
    }

    #[test]
    fn test_theme_by_name() {
        let dark = Theme::by_name("dark");
        assert_eq!(dark.name, "Dark");

        let retro = Theme::by_name("retro");
        assert_eq!(retro.name, "Retro");

        let neon = Theme::by_name("cyber");
        assert_eq!(neon.name, "Neon");

        let default = Theme::by_name("unknown");
        assert_eq!(default.name, "Dark");
    }

    #[test]
    fn test_theme_all() {
        let themes = Theme::all();
        assert_eq!(themes.len(), 5);
    }
}
