//! TUI - Terminal User Interface subsystem unificado
//!
//! Re-exporta tipos existentes de events-ry y agrega wrappers visuales.
//! NO redefine tipos que ya existen en events-ry.

// === Módulos ===
pub mod renderer;

// === Re-exports desde events-ry ===
pub use events_ry::{Shell, ShellCommand, ShellResult, CommandHandler};
pub use events_ry::{ConsoleLine, ConsoleKind};
pub use renderer::{render_tui, render_tui_status};

/// Identificador de tecla (string simple, backend-agnóstico)
pub type KeyId = &'static str;

/// Teclas por defecto del TUI
pub mod default_keys {
    pub const TOGGLE_CONSOLE: &str = "F1";
    pub const TOGGLE_INPUT: &str = "`";
    pub const HISTORY_UP: &str = "Up";
    pub const HISTORY_DOWN: &str = "Down";
    pub const EXECUTE: &str = "Return";
    pub const BACKSPACE: &str = "Backspace";
    pub const TAB: &str = "Tab";
    pub const CLEAR: &str = "Escape";
}

/// Configuración de keybindings
#[derive(Debug, Clone)]
pub struct Keybindings {
    pub toggle_console: KeyId,
    pub toggle_input: KeyId,
    pub history_up: KeyId,
    pub history_down: KeyId,
    pub execute: KeyId,
    pub backspace: KeyId,
    pub tab: KeyId,
    pub clear: KeyId,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            toggle_console: default_keys::TOGGLE_CONSOLE,
            toggle_input: default_keys::TOGGLE_INPUT,
            history_up: default_keys::HISTORY_UP,
            history_down: default_keys::HISTORY_DOWN,
            execute: default_keys::EXECUTE,
            backspace: default_keys::BACKSPACE,
            tab: default_keys::TAB,
            clear: default_keys::CLEAR,
        }
    }
}

/// Estado visual del TUI
#[derive(Debug, Clone)]
pub struct TuiState {
    pub visible: bool,
    pub input_active: bool,
    pub input_buffer: String,
    pub console_lines: Vec<ConsoleLine>,
    pub max_console_lines: usize,
    pub fps: f64,
    pub frame: u64,
}

impl Default for TuiState {
    fn default() -> Self {
        Self {
            visible: true,
            input_active: true,
            input_buffer: String::new(),
            console_lines: Vec::new(),
            max_console_lines: 200,
            fps: 60.0,
            frame: 0,
        }
    }
}

/// Sistema TUI completo
pub struct TuiSystem {
    shell: Shell,
    state: TuiState,
    keybindings: Keybindings,
}

impl Default for TuiSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiSystem {
    pub fn new() -> Self {
        Self {
            shell: Shell::new(),
            state: TuiState::default(),
            keybindings: Keybindings::default(),
        }
    }

    pub fn state(&self) -> &TuiState { &self.state }
    pub fn state_mut(&mut self) -> &mut TuiState { &mut self.state }
    pub fn keybindings(&self) -> &Keybindings { &self.keybindings }
    pub fn keybindings_mut(&mut self) -> &mut Keybindings { &mut self.keybindings }

    /// Manejar pulsación de tecla (retorna true si fue procesada)
    pub fn on_key(&mut self, key: &str) -> bool {
        if key == self.keybindings.toggle_console {
            self.toggle_console();
            return true;
        }
        if key == self.keybindings.toggle_input && self.state.visible {
            self.toggle_input();
            return true;
        }
        if !self.state.input_active {
            return false;
        }
        if key == self.keybindings.execute {
            if !self.state.input_buffer.is_empty() {
                let input = self.state.input_buffer.clone();
                self.state.input_buffer.clear();
                self.execute_command(&input);
            }
            return true;
        }
        if key == self.keybindings.backspace {
            self.remove_input();
            return true;
        }
        if key == self.keybindings.tab {
            if let Some(completion) = self.complete_current() {
                self.state.input_buffer = completion;
            }
            return true;
        }
        if key == self.keybindings.history_up {
            if let Some(cmd) = self.history_prev() {
                self.state.input_buffer = cmd;
            }
            return true;
        }
        if key == self.keybindings.history_down {
            if let Some(cmd) = self.history_next() {
                self.state.input_buffer = cmd;
            }
            return true;
        }
        if key == self.keybindings.clear {
            self.clear_console();
            return true;
        }
        false
    }

    /// Manejar evento de migui Key
    pub fn on_migui_key(&mut self, key: &migui::Key) -> bool {
        let key_str = match key {
            migui::Key::F1 => "F1",
            migui::Key::Backquote => "`",
            migui::Key::ArrowUp => "Up",
            migui::Key::ArrowDown => "Down",
            migui::Key::Enter => "Return",
            migui::Key::Backspace => "Backspace",
            migui::Key::Tab => "Tab",
            migui::Key::Escape => "Escape",
            _ => return false,
        };
        self.on_key(key_str)
    }

    /// Manejar entrada de texto (caracteres)
    pub fn on_text(&mut self, text: &str) {
        if self.state.input_active {
            self.state.input_buffer.push_str(text);
        }
    }

    /// Ejecutar comando y agregar a consola
    pub fn execute_command(&mut self, input: &str) -> ShellResult {
        let result = self.shell.execute(input);

        self.state.console_lines.push(ConsoleLine {
            text: input.to_string(),
            kind: ConsoleKind::Input,
        });

        let kind = if result.success { ConsoleKind::Success } else { ConsoleKind::Error };
        self.state.console_lines.push(ConsoleLine {
            text: result.output.clone(),
            kind,
        });

        if self.state.console_lines.len() > self.state.max_console_lines {
            let excess = self.state.console_lines.len() - self.state.max_console_lines;
            self.state.console_lines.drain(0..excess);
        }

        result
    }

    pub fn console_lines(&self) -> &[ConsoleLine] { &self.state.console_lines }
    pub fn input_buffer(&self) -> &str { &self.state.input_buffer }
    pub fn set_input(&mut self, text: &str) { self.state.input_buffer = text.to_string(); }
    pub fn append_input(&mut self, ch: char) { self.state.input_buffer.push(ch); }
    pub fn remove_input(&mut self) { self.state.input_buffer.pop(); }
    pub fn clear_console(&mut self) { self.state.console_lines.clear(); }
    pub fn toggle_console(&mut self) { self.state.visible = !self.state.visible; }
    pub fn toggle_input(&mut self) { self.state.input_active = !self.state.input_active; }

    // === Historial (↑↓) ===

    pub fn history_prev(&mut self) -> Option<String> {
        self.shell.history_prev().map(|s| s.to_string())
    }

    pub fn history_next(&mut self) -> Option<String> {
        self.shell.history_next().map(|s| s.to_string())
    }

    pub fn history(&self) -> &[String] {
        self.shell.history()
    }

    // === Tab completion ===

    pub fn complete(&self, prefix: &str) -> Vec<String> {
        self.shell.complete(prefix)
    }

    pub fn complete_current(&self) -> Option<String> {
        let parts: Vec<&str> = self.state.input_buffer.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }
        let prefix = parts.last().unwrap_or(&"");
        let completions = self.shell.complete(prefix);
        if completions.len() == 1 {
            Some(completions[0].clone())
        } else {
            None
        }
    }

    /// Registrar comandos engine-aware adicionales
    pub fn register_engine_commands(&mut self) {
        self.shell.register("entities", |_| ShellResult {
            success: true,
            output: "Entidades: mostrando lista...".to_string(),
            data: None,
        });
        self.shell.register("assets", |_| ShellResult {
            success: true,
            output: "Assets: listando catálogo...".to_string(),
            data: None,
        });
        self.shell.register("fps", |_| ShellResult {
            success: true,
            output: "FPS: 60".to_string(),
            data: None,
        });
    }
}
