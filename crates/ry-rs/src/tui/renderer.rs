//! Renderer del TUI - genera DrawCommands de migui

use migui::{DrawCommand, Rect, Color};
use super::{TuiSystem, ConsoleKind};

/// Constantes de renderizado
const LINE_HEIGHT: f32 = 18.0;
const CHAR_WIDTH: f32 = 8.0;
const PADDING: f32 = 8.0;
const INPUT_HEIGHT: f32 = 24.0;

/// Colores del TUI
const C_BG: Color = Color { r: 18, g: 18, b: 24, a: 240 };
const C_PANEL_BG: Color = Color { r: 28, g: 28, b: 36, a: 230 };
const C_HEADER_BG: Color = Color { r: 40, g: 40, b: 55, a: 255 };
const C_TEXT: Color = Color { r: 200, g: 200, b: 210, a: 255 };
const C_GREEN: Color = Color { r: 80, g: 200, b: 120, a: 255 };
const C_RED: Color = Color { r: 220, g: 80, b: 80, a: 255 };
const C_CYAN: Color = Color { r: 80, g: 180, b: 220, a: 255 };
const C_YELLOW: Color = Color { r: 220, g: 200, b: 60, a: 255 };
const C_INPUT: Color = Color { r: 140, g: 200, b: 140, a: 255 };
const C_BORDER: Color = Color { r: 50, g: 50, b: 65, a: 255 };

/// Renderizar el TUI completo y retornar DrawCommands
pub fn render_tui(tui: &TuiSystem, x: f32, y: f32, w: f32, h: f32) -> Vec<DrawCommand> {
    let mut cmds = Vec::new();

    if !tui.state().visible {
        return cmds;
    }

    // Fondo del panel
    cmds.push(DrawCommand::DrawRect {
        rect: Rect { x, y, w, h },
        color: C_PANEL_BG,
    });

    // Borde
    cmds.push(DrawCommand::DrawRect {
        rect: Rect { x, y, w, h: 1.0 },
        color: C_BORDER,
    });
    cmds.push(DrawCommand::DrawRect {
        rect: Rect { x, y: y + h - 1.0, w, h: 1.0 },
        color: C_BORDER,
    });
    cmds.push(DrawCommand::DrawRect {
        rect: Rect { x, y, w: 1.0, h },
        color: C_BORDER,
    });
    cmds.push(DrawCommand::DrawRect {
        rect: Rect { x: x + w - 1.0, y, w: 1.0, h },
        color: C_BORDER,
    });

    // Header
    let header_h = 22.0;
    cmds.push(DrawCommand::DrawRect {
        rect: Rect { x: x + 1.0, y: y + 1.0, w: w - 2.0, h: header_h },
        color: C_HEADER_BG,
    });
    cmds.push(DrawCommand::DrawText {
        text: "TUI Console".to_string(),
        x: x + PADDING,
        y: y + 5.0,
        size: 14,
        color: C_GREEN,
    });

    // Scrollback lines
    let content_y = y + header_h + 2.0;
    let content_h = h - header_h - INPUT_HEIGHT - 4.0;
    let max_lines = (content_h / LINE_HEIGHT) as usize;
    let lines = tui.console_lines();
    let start = if lines.len() > max_lines {
        lines.len() - max_lines
    } else {
        0
    };

    let mut ly = content_y;
    for line in lines.iter().skip(start) {
        if ly + LINE_HEIGHT > content_y + content_h {
            break;
        }
        let color = match line.kind {
            ConsoleKind::Info => C_TEXT,
            ConsoleKind::Success => C_GREEN,
            ConsoleKind::Error => C_RED,
            ConsoleKind::Input => C_YELLOW,
            ConsoleKind::Debug => C_CYAN,
        };
        cmds.push(DrawCommand::DrawText {
            text: line.text.clone(),
            x: x + PADDING,
            y: ly,
            size: 12,
            color,
        });
        ly += LINE_HEIGHT;
    }

    // Input line
    let input_y = y + h - INPUT_HEIGHT - 2.0;
    cmds.push(DrawCommand::DrawRect {
        rect: Rect { x: x + 1.0, y: input_y, w: w - 2.0, h: INPUT_HEIGHT },
        color: C_BG,
    });

    // Prompt ">"
    cmds.push(DrawCommand::DrawText {
        text: ">".to_string(),
        x: x + PADDING,
        y: input_y + 4.0,
        size: 14,
        color: C_GREEN,
    });

    // Input text
    let input_text = tui.input_buffer();
    cmds.push(DrawCommand::DrawText {
        text: input_text.to_string(),
        x: x + PADDING + CHAR_WIDTH + 4.0,
        y: input_y + 4.0,
        size: 14,
        color: C_INPUT,
    });

    // Cursor (si input activo)
    if tui.state().input_active {
        let cursor_x = x + PADDING + CHAR_WIDTH + 4.0 + (input_text.len() as f32 * CHAR_WIDTH);
        cmds.push(DrawCommand::DrawRect {
            rect: Rect { x: cursor_x, y: input_y + 2.0, w: 2.0, h: INPUT_HEIGHT - 4.0 },
            color: C_GREEN,
        });
    }

    cmds
}

/// Renderizar solo el header del TUI (para uso en status bar)
pub fn render_tui_status(tui: &TuiSystem, x: f32, y: f32) -> Vec<DrawCommand> {
    let mut cmds = Vec::new();

    if !tui.state().visible {
        return cmds;
    }

    cmds.push(DrawCommand::DrawText {
        text: format!("TUI | FPS: {:.1}", tui.state().fps),
        x,
        y,
        size: 12,
        color: C_GREEN,
    });

    cmds
}
