//! Audit Logger - Registro de cada acción ejecutada
//!
//! Todo queda registrado. Sin excepciones.

use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: String,
    pub action: String,
    pub target: String,
    pub status: String,
}

/// Logger de auditoría completa
pub struct AuditLogger {
    pub enabled: bool,
    pub entries: Vec<AuditEntry>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            enabled: false,
            entries: Vec::new(),
        }
    }

    pub fn enable(&mut self) -> &mut Self {
        self.enabled = true;
        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.enabled = false;
        self
    }

    fn now() -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        format!("{}", now.as_secs())
    }

    fn log(&mut self, action: &str, target: &str, status: &str) {
        if self.enabled {
            self.entries.push(AuditEntry {
                timestamp: Self::now(),
                action: action.to_string(),
                target: target.to_string(),
                status: status.to_string(),
            });
        }
    }

    pub fn log_start(&mut self, target: &str) {
        self.log("start", target, "pending");
    }

    pub fn log_action(&mut self, action: &str, target: &str) {
        self.log(action, target, "running");
    }

    pub fn log_success(&mut self, target: &str) {
        self.log("complete", target, "success");
    }

    pub fn log_error(&mut self, action: &str, error: &str) {
        self.log(action, error, "error");
    }

    /// Generar reporte de auditoría
    pub fn generate_report(&self) -> String {
        if !self.enabled {
            return "Audit disabledado".to_string();
        }

        let mut report = String::from("📋 AUDIT LOG\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');

        for entry in &self.entries {
            report.push_str(&format!(
                "[{}] {} -> {} ({})\n",
                entry.timestamp, entry.action, entry.target, entry.status
            ));
        }

        report.push_str(&format!("\nTotal entries: {}\n", self.entries.len()));
        report
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}
