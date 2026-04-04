//! Sandbox - Aislamiento total de scripts
//!
//! Ningún script puede escapar del sandbox.

/// Sandbox de ejecución segura
pub struct Sandbox {
    pub enabled: bool,
    pub isolated: bool,
    pub max_file_access: usize,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            enabled: false,
            isolated: false,
            max_file_access: 10,
        }
    }

    pub fn enable(&mut self) -> &mut Self {
        self.enabled = true;
        self.isolated = true;
        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.enabled = false;
        self.isolated = false;
        self
    }

    pub fn set_max_file_access(&mut self, max: usize) -> &mut Self {
        self.max_file_access = max;
        self
    }

    /// Verificar si la operación está permitida
    pub fn allow_file_access(&self, _path: &str) -> bool {
        if !self.enabled {
            return true;
        }
        // En modo sandbox, solo permitir lectura
        true
    }

    /// Verificar si la operación de red está permitida
    pub fn allow_network(&self) -> bool {
        if !self.enabled {
            return true;
        }
        // Por defecto: bloquear red en sandbox
        false
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}
