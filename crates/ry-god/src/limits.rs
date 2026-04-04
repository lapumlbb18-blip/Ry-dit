//! Resource Limits - Límites de memoria/CPU para low-end
//!
//! Sin OOM, sin infinite loops, sin crashes.

/// Límites de recursos para ejecución segura
pub struct ResourceLimits {
    pub max_memory_mb: u64,
    pub max_instructions: u64,
    pub max_loop_iterations: u64,
    pub max_script_size_kb: usize,
    pub max_variables: usize,
    pub max_call_depth: usize,
}

impl ResourceLimits {
    pub fn new() -> Self {
        Self {
            max_memory_mb: 64,
            max_instructions: 1_000_000,
            max_loop_iterations: 100_000,
            max_script_size_kb: 256,
            max_variables: 1000,
            max_call_depth: 32,
        }
    }

    pub fn set_max_memory_mb(&mut self, mb: u64) -> &mut Self {
        self.max_memory_mb = mb;
        self
    }

    pub fn set_max_instructions(&mut self, count: u64) -> &mut Self {
        self.max_instructions = count;
        self
    }

    pub fn set_max_loop_iterations(&mut self, count: u64) -> &mut Self {
        self.max_loop_iterations = count;
        self
    }

    /// Verificar si se excedieron los límites
    pub fn check_limits(
        &self,
        current_memory_mb: u64,
        current_instructions: u64,
        current_loops: u64,
    ) -> Result<(), String> {
        if current_memory_mb > self.max_memory_mb {
            return Err(format!(
                "Memoria excedida: {}MB > {}MB",
                current_memory_mb, self.max_memory_mb
            ));
        }

        if current_instructions > self.max_instructions {
            return Err(format!(
                "Instrucciones excedidas: {} > {}",
                current_instructions, self.max_instructions
            ));
        }

        if current_loops > self.max_loop_iterations {
            return Err(format!(
                "Loop infinito detectado: {} > {}",
                current_loops, self.max_loop_iterations
            ));
        }

        Ok(())
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::new()
    }
}
