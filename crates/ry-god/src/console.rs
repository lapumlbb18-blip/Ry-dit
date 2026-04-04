//! Console Reporter - Errores, warnings, info detallado
//!
//! Lo que a cargo se le escapa, ry-god lo reporta todo.

use std::time::Instant;

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Ok,
}

/// Reporter de consola con colores y detalles
pub struct ConsoleReporter {
    pub warnings_enabled: bool,
    pub errors: Vec<(String, String)>,
    pub warnings: Vec<(String, String)>,
    pub infos: Vec<String>,
}

impl ConsoleReporter {
    pub fn new() -> Self {
        Self {
            warnings_enabled: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            infos: Vec::new(),
        }
    }

    pub fn enable_warnings(&mut self) {
        self.warnings_enabled = true;
    }

    pub fn disable_warnings(&mut self) {
        self.warnings_enabled = false;
    }

    /// Reportar info
    pub fn info(&mut self, label: &str, msg: &str) {
        let entry = format!("[INFO] {}: {}", label, msg);
        println!("{}", entry);
        self.infos.push(entry);
    }

    /// Reportar warning (lo que cargo ignora)
    pub fn warn(&mut self, label: &str, msg: &str) {
        if self.warnings_enabled {
            let entry = format!("[WARN] {}: {}", label, msg);
            println!("⚠️  {}", entry);
            self.warnings.push((label.to_string(), msg.to_string()));
        }
    }

    /// Reportar error con detalle
    pub fn error(&mut self, label: &str, msg: &str) {
        let entry = format!("[ERROR] {}: {}", label, msg);
        println!("❌ {}", entry);
        self.errors.push((label.to_string(), msg.to_string()));
    }

    /// Reportar éxito
    pub fn ok(&mut self, label: &str, msg: &str) {
        let entry = format!("[OK] {}: {}", label, msg);
        println!("✅ {}", entry);
    }

    /// Resumen de warnings y errores
    pub fn summary(&self) {
        println!("\n📊 Resumen:");
        println!("   Info:     {}", self.infos.len());
        println!("   Warnings: {}", self.warnings.len());
        println!("   Errors:   {}", self.errors.len());

        if !self.warnings.is_empty() {
            println!("\n⚠️  Warnings detallados:");
            for (label, msg) in &self.warnings {
                println!("   - {}: {}", label, msg);
            }
        }

        if !self.errors.is_empty() {
            println!("\n❌ Errores detallados:");
            for (label, msg) in &self.errors {
                println!("   - {}: {}", label, msg);
            }
        }
    }
}

impl Default for ConsoleReporter {
    fn default() -> Self {
        Self::new()
    }
}
