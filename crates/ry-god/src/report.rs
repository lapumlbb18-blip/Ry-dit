//! Execution Report - Informe detallado de ejecución
//!
//! Lo que cargo se escapa, ry-god lo documenta todo.

use std::time::Instant;

/// Reporte de ejecución completo
pub struct ExecutionReport {
    pub script: String,
    pub success: bool,
    pub errors: Vec<(String, String)>,
    pub warnings: Vec<(String, String)>,
    pub start_time: Option<Instant>,
    pub elapsed_ms: u128,
    pub instructions_executed: u64,
}

impl ExecutionReport {
    pub fn new() -> Self {
        Self {
            script: String::new(),
            success: false,
            errors: Vec::new(),
            warnings: Vec::new(),
            start_time: None,
            elapsed_ms: 0,
            instructions_executed: 0,
        }
    }

    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop_timer(&mut self) {
        if let Some(start) = self.start_time.take() {
            self.elapsed_ms = start.elapsed().as_millis();
        }
    }

    pub fn mark_success(&mut self) {
        self.success = true;
    }

    pub fn add_error(&mut self, category: &str, msg: &str) {
        self.errors.push((category.to_string(), msg.to_string()));
    }

    pub fn add_warning(&mut self, category: &str, msg: &str) {
        self.warnings.push((category.to_string(), msg.to_string()));
    }

    /// Generar informe en texto
    pub fn generate_text(&self) -> String {
        let mut report = String::new();
        report.push_str("🛡️ RY-GOD EXECUTION REPORT\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');

        report.push_str(&format!("Script: {}\n", self.script));
        report.push_str(&format!(
            "Estado: {}\n",
            if self.success { "✅ ÉXITO" } else { "❌ FALLO" }
        ));
        report.push_str(&format!("Tiempo: {}ms\n", self.elapsed_ms));
        report.push_str(&format!("Instrucciones: {}\n", self.instructions_executed));

        if !self.errors.is_empty() {
            report.push_str(&format!("\n❌ ERRORES ({}):\n", self.errors.len()));
            for (cat, msg) in &self.errors {
                report.push_str(&format!("  - [{}] {}\n", cat, msg));
            }
        }

        if !self.warnings.is_empty() {
            report.push_str(&format!("\n⚠️  WARNINGS ({}):\n", self.warnings.len()));
            for (cat, msg) in &self.warnings {
                report.push_str(&format!("  - [{}] {}\n", cat, msg));
            }
        }

        report.push_str(&"\n".to_owned());
        report.push_str(&"=".repeat(50));
        report
    }

    /// Imprimir informe en consola
    pub fn print(&self) {
        println!("{}", self.generate_text());
    }
}

impl Default for ExecutionReport {
    fn default() -> Self {
        Self::new()
    }
}
