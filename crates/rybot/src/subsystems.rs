//! Subsistemas del motor — wrappers sobre los crates del ecosistema

// ============================================================================
// INPUT
// ============================================================================

use ry_input::{InputMap, InputState};

/// Subsistema de input
pub struct InputSubsystem {
    map: InputMap,
    state: InputState,
}

impl Default for InputSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl InputSubsystem {
    pub fn new() -> Self {
        let map = InputMap::with_defaults();
        let state = InputState::new(&map);
        Self { map, state }
    }

    pub fn update(&mut self) {
        self.state.begin_frame();
    }

    pub fn is_action_pressed(&self, action: &str) -> bool {
        self.state.is_action_pressed(action)
    }

    pub fn is_action_just_pressed(&self, action: &str) -> bool {
        self.state.is_action_just_pressed(action)
    }

    pub fn update_key(&mut self, key: &str, pressed: bool) {
        self.state.update_key(key, pressed);
    }

    pub fn action_count(&self) -> usize {
        self.state.list_actions().len()
    }

    pub fn load_input_map(&mut self, path: &str) -> Result<(), String> {
        let map = InputMap::load(path)?;
        self.state = InputState::new(&map);
        self.map = map;
        Ok(())
    }
}

// ============================================================================
// PHYSICS
// ============================================================================

/// Subsistema de físicas (wrapper sobre ry-physics)
pub struct PhysicsSubsystem {
    enabled: bool,
    gravity: f32,
}

impl Default for PhysicsSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsSubsystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            gravity: 9.8,
        }
    }

    pub fn update(&mut self, _delta: f32) {
        // ry-physics update
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
    }

    pub fn gravity(&self) -> f32 {
        self.gravity
    }

    pub fn set_gravity(&mut self, g: f32) {
        self.gravity = g;
    }
}

// ============================================================================
// ANIMATION
// ============================================================================

/// Subsistema de animación (wrapper sobre ry-anim)
pub struct AnimationSubsystem {
    enabled: bool,
}

impl Default for AnimationSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSubsystem {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn update(&mut self, _delta: f32) {
        // ry-anim update
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
    }
}

// ============================================================================
// SCIENCE
// ============================================================================

/// Subsistema de ciencia (wrapper sobre ry-science)
pub struct ScienceSubsystem {
    enabled: bool,
}

impl Default for ScienceSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ScienceSubsystem {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    pub fn update(&mut self, _delta: f32) {
        // ry-science update
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
    }
}

// ============================================================================
// RENDER
// ============================================================================

/// Subsistema de render (coordina ry-gfx + ry3d-gfx)
pub struct RenderSubsystem {
    pub use_3d: bool,
}

impl Default for RenderSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderSubsystem {
    pub fn new() -> Self {
        Self { use_3d: false }
    }

    pub fn update(&mut self) {
        // ry-gfx / ry3d-gfx update
    }
}

// ============================================================================
// NETWORK
// ============================================================================

/// Subsistema de red (wrapper sobre ry-stream)
pub struct NetworkSubsystem {
    enabled: bool,
}

impl Default for NetworkSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkSubsystem {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    pub fn update(&mut self, _delta: f32) {
        // ry-stream update
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_subsystem() {
        let mut input = InputSubsystem::new();
        input.update();
        input.update_key("W", true);
        assert!(input.is_action_pressed("move_up"));
        assert!(input.action_count() > 0);
    }

    #[test]
    fn test_physics_subsystem() {
        let mut physics = PhysicsSubsystem::new();
        assert!(physics.enabled());
        assert!((physics.gravity() - 9.8).abs() < 0.01);
        physics.set_gravity(20.0);
        assert!((physics.gravity() - 20.0).abs() < 0.01);
        physics.set_enabled(false);
        assert!(!physics.enabled());
    }

    #[test]
    fn test_animation_subsystem() {
        let anim = AnimationSubsystem::new();
        assert!(anim.enabled());
    }

    #[test]
    fn test_science_subsystem() {
        let science = ScienceSubsystem::new();
        assert!(!science.enabled()); // disabled by default
    }

    #[test]
    fn test_network_subsystem() {
        let net = NetworkSubsystem::new();
        assert!(!net.enabled()); // disabled by default
    }
}
