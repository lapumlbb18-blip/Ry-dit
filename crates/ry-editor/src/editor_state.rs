use crate::viewport_manager::ViewportManager;
use crate::viewport_controller::ViewportController;
use rybot::{RybotEngine, RybotGui};
use migui::Migui;
use ry_gfx::RyditGfx;
use ry_core::ModuleRegistry;

pub struct EditorState {
    pub gui: Migui,
    pub rybot_gui: RybotGui,
    pub engine: RybotEngine,
    pub registry: ModuleRegistry, // 🆕 El Ensamblador
    pub viewports: ViewportManager,
    pub viewport_ctrl: ViewportController,
    pub is_running: bool,
}

impl EditorState {
    pub fn new() -> Self {
        let mut rybot_gui = RybotGui::new();
        rybot_gui.open = true;

        // 🆕 Inicializar el Ensamblador Maestro
        let mut registry = ModuleRegistry::new();
        
        // Registrar módulos fundamentales del sistema universal RY
        registry.register(ry_rs::modules::physics::PhysicsWorldModule);
        registry.register(ry_rs::modules::script_particles::ParticleModule);

        Self {
            gui: Migui::new(),
            rybot_gui,
            engine: RybotEngine::new(),
            registry,
            viewports: ViewportManager::new(),
            viewport_ctrl: ViewportController::new(),
            is_running: true,
        }
    }

    /// Actualiza el estado del editor y sus módulos ensamblados
    pub fn update(&mut self, _gfx: &mut RyditGfx, dt: f32) {
        if !self.is_running { return; }

        // 1. Actualizar el motor de datos (Scene Tree)
        self.engine.update(dt);

        // 2. Actualizar la lógica ensamblada (Físicas, Partículas, etc.)
        self.registry.update_all(dt);

        // 3. Sincronizar UI del editor
        // ... lógica de UI ...
    }

    /// Dibuja los módulos que tienen representación visual
    pub fn draw(&mut self) {
        // El Ensamblador pide a los módulos que dibujen
        self.registry.draw_all();
    }
}
