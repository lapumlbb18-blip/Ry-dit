// Demo Real - Panel de Control ry-dit
// Inspirado en console_emulator_prototype.cpp (bgfx_libs)
// Stack: SDL2 + ImGui (migui) + events-ry + ry-anim
// Pipeline gráfico: Zink/DRI3 → OpenGL ES → VirGL fallback
//
// Uso: cargo run --bin demo_panel --release
//
// Paneles:
// - Screen: Animación ry-anim en vivo
// - Console: Shell de events-ry interactivo
// - Input: Estado de input unificado
// - Controls: Mapeo de teclas

use events_ry::{InputManager, InputEvent, Key, TextInputAction, ShellResult};
use ry_anim::{disney, illusions, effects};

// ============================================================================
// CONFIGURACIÓN
// ============================================================================
const SCREEN_W: i32 = 800;
const SCREEN_H: i32 = 600;
const TARGET_FPS: u32 = 30;

// ============================================================================
// ESTADO DEL PANEL
// ============================================================================
struct PanelState {
    running: bool,
    frame: u64,
    t: f64,
    scene: usize,
    scenes: Vec<&'static str>,
    
    // Console
    console_input: String,
    console_lines: Vec<ConsoleLine>,
    cursor_pos: usize,
    composing: bool,
    
    // Input state
    keys_down: Vec<Key>,
    mouse_x: i32,
    mouse_y: i32,
    
    // Layout
    show_screen: bool,
    show_console: bool,
    show_input: bool,
    show_controls: bool,
}

#[derive(Clone)]
struct ConsoleLine {
    text: String,
    kind: ConsoleKind,
}

#[derive(Clone)]
enum ConsoleKind {
    Info,
    Success,
    Error,
    Input,
    Debug,
}

impl PanelState {
    fn new() -> Self {
        Self {
            running: true,
            frame: 0,
            t: 0.0,
            scene: 0,
            scenes: vec![
                "Disney: Follow Through",
                "Disney: Arcs",
                "Illusion: Rotating Snakes",
                "Effect: Neon Glow",
                "Effect: Morphing",
                "Science: Tusi Couple",
            ],
            console_input: String::new(),
            console_lines: vec![
                ConsoleLine { text: "ry-dit Console v0.13.0".to_string(), kind: ConsoleKind::Info },
                ConsoleLine { text: "Escribe 'help' para ver comandos".to_string(), kind: ConsoleKind::Info },
            ],
            cursor_pos: 0,
            composing: false,
            keys_down: Vec::new(),
            mouse_x: 0,
            mouse_y: 0,
            show_screen: true,
            show_console: true,
            show_input: true,
            show_controls: false,
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================
fn main() {
    println!("[DEMO PANEL] ry-dit Panel de Control");
    println!("[DEMO PANEL] Pipeline: Zink/DRI3 → OpenGL ES → VirGL fallback");
    println!("[DEMO PANEL] ESC = Salir | 1-4 = Toggle paneles | SPACE = Cambiar escena");
    println!("[DEMO PANEL] Escriba comandos en consola: help, load, exec, debug");

    let mut state = PanelState::new();
    let mut manager = InputManager::new();
    
    // Activar text input para consola
    manager.enable_text_input();

    // Loop principal (simulado sin SDL2 real - usa consola)
    // En producción real, esto conectaría con Sdl2InputBackend
    while state.running {
        // Simular frame
        state.t += 1.0 / TARGET_FPS as f64;
        state.frame += 1;

        // Input simulado (en demo real, vendría de Sdl2InputBackend)
        process_keyboard_input(&mut state, &mut manager);

        // Render (en demo real, iría a SDL2 + OpenGL ES)
        if state.frame % 30 == 0 {
            println!("\n[FRAME {}] t={:.2}s | Scene: {} | Consola: {} líneas", 
                     state.frame, state.t, 
                     state.scenes[state.scene],
                     state.console_lines.len());
        }

        // Limitar frames
        std::thread::sleep(std::time::Duration::from_millis(1000 / TARGET_FPS as u64));
    }

    println!("[DEMO PANEL] Shutdown. Frames: {}", state.frame);
}

// ============================================================================
// INPUT PROCESSING
// ============================================================================
fn process_keyboard_input(state: &mut PanelState, manager: &mut InputManager) {
    // Poll text actions
    for action in manager.poll_text_actions() {
        match action {
            TextInputAction::CharAdded(ch) => {
                state.console_input.push(ch);
                state.cursor_pos += 1;
                state.composing = true;
            }
            TextInputAction::Committed(text) => {
                if !text.is_empty() {
                    // Ejecutar comando
                    state.console_lines.push(ConsoleLine {
                        text: format!("> {}", text),
                        kind: ConsoleKind::Input,
                    });
                    
                    let result = manager.execute_command(&text);
                    add_shell_result(&mut state.console_lines, &result);
                }
                state.console_input.clear();
                state.cursor_pos = 0;
                state.composing = false;
            }
            TextInputAction::CharDeleted => {
                if !state.console_input.is_empty() {
                    state.console_input.pop();
                    if state.cursor_pos > 0 {
                        state.cursor_pos -= 1;
                    }
                }
            }
            _ => {}
        }
    }

    // Poll raw events
    for event in manager.poll_events() {
        match event {
            InputEvent::KeyPressed { key } => {
                state.keys_down.push(key);
                match key {
                    Key::Escape => state.running = false,
                    Key::Space => {
                        state.scene = (state.scene + 1) % state.scenes.len();
                        state.console_lines.push(ConsoleLine {
                            text: format!("Scene → {}", state.scenes[state.scene]),
                            kind: ConsoleKind::Info,
                        });
                    }
                    Key::Num1 => state.show_screen = !state.show_screen,
                    Key::Num2 => state.show_console = !state.show_console,
                    Key::Num3 => state.show_input = !state.show_input,
                    Key::Num4 => state.show_controls = !state.show_controls,
                    _ => {}
                }
            }
            InputEvent::KeyReleased { key } => {
                state.keys_down.retain(|k| *k != key);
            }
            InputEvent::MouseMoved { x, y } => {
                state.mouse_x = x;
                state.mouse_y = y;
            }
            _ => {}
        }
    }
}

fn add_shell_result(lines: &mut Vec<ConsoleLine>, result: &ShellResult) {
    if !result.output.is_empty() {
        for line in result.output.lines() {
            lines.push(ConsoleLine {
                text: line.to_string(),
                kind: if result.success { ConsoleKind::Success } else { ConsoleKind::Error },
            });
        }
    }
    // Limitar líneas
    if lines.len() > 200 {
        let excess = lines.len() - 200;
        lines.drain(0..excess);
    }
}
