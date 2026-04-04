//! RyPrime - Comando de autorización sobre el proyecto
//!
//! ryprime es el "sudo" de Ry-Dit. Requiere autorización explícita
//! para operaciones sensibles sobre el proyecto.

/// Sistema de autorización ryprime
pub struct RyPrime {
    pub enabled: bool,
    pub authorized_projects: Vec<String>,
    pub current_project: String,
}

impl RyPrime {
    pub fn new() -> Self {
        Self {
            enabled: true,
            authorized_projects: vec!["Ry-dit".to_string()],
            current_project: "Ry-dit".to_string(),
        }
    }

    /// Verificar si un script requiere autorización
    pub fn requires_authorization(&self, script_path: &str) -> bool {
        // Scripts en demos/ no requieren autorización
        if script_path.starts_with("demos/") {
            return false;
        }

        // Scripts que modifican el proyecto requieren autorización
        script_path.contains("modificar")
            || script_path.contains("delete")
            || script_path.contains("admin")
    }

    /// Autorizar ejecución
    pub fn authorize(&mut self, script_path: &str) -> bool {
        if self.requires_authorization(script_path) {
            // En modo real, pedir confirmación al usuario
            // Por ahora, auto-autorizar para desarrollo
            println!("🔑 ryprime: Autorizando {}", script_path);
            true
        } else {
            true
        }
    }

    /// Activar modo estricto
    pub fn enable_strict(&mut self) {
        self.enabled = true;
    }

    /// Desactivar verificación (NO recomendado en producción)
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Default for RyPrime {
    fn default() -> Self {
        Self::new()
    }
}
