//! gamepad.rs - Sistema de gamepads multiplataforma (Capa 1)
//!
//! - `GamepadState`: estado backend-agnóstico (botones + ejes) — siempre disponible
//! - `GamepadManager`: gestión SDL2 (hotplug, dead zones, rumble) — feature `sdl2-backend`
//!
//! ## Arquitectura
//!
//! ```text
//! SDL2 Event (Controller*) ──handle_event──► GamepadManager ──► GamepadState (por instance_id)
//! Hardware polling        ──poll──────────► GamepadManager ──► dead zone + rumble
//! ```

use crate::input_event::{GamepadAxis, GamepadButton};
use std::collections::{HashMap, HashSet};

// ============================================================================
// ESTADO DE GAMEPAD (backend-agnóstico)
// ============================================================================

/// Estado de un gamepad: botones presionados + valor de ejes (-1.0..=1.0)
#[derive(Debug, Clone, Default)]
pub struct GamepadState {
    buttons: HashSet<GamepadButton>,
    axes: HashMap<GamepadAxis, f32>,
}

impl GamepadState {
    /// Crear estado vacío
    pub fn new() -> Self {
        Self::default()
    }

    /// Marcar/desmarcar botón
    pub fn set_button(&mut self, button: GamepadButton, pressed: bool) {
        if pressed {
            self.buttons.insert(button);
        } else {
            self.buttons.remove(&button);
        }
    }

    /// Asignar valor de eje (se recorta a -1.0..=1.0)
    pub fn set_axis(&mut self, axis: GamepadAxis, value: f32) {
        self.axes.insert(axis, value.clamp(-1.0, 1.0));
    }

    /// ¿Botón presionado?
    pub fn is_button_down(&self, button: GamepadButton) -> bool {
        self.buttons.contains(&button)
    }

    /// Valor crudo de eje (sin dead zone)
    pub fn axis(&self, axis: GamepadAxis) -> f32 {
        self.axes.get(&axis).copied().unwrap_or(0.0)
    }

    /// Botones presionados actualmente
    pub fn pressed_buttons(&self) -> Vec<GamepadButton> {
        self.buttons.iter().copied().collect()
    }

    /// Vaciar estado
    pub fn clear(&mut self) {
        self.buttons.clear();
        self.axes.clear();
    }
}

// ============================================================================
// DEAD ZONES
// ============================================================================

/// Dead zone radial para sticks (par de ejes). Devuelve (x, y) renormalizados
/// para que el movimiento sea proporcional desde el borde de la dead zone.
pub fn apply_radial_dead_zone(x: f32, y: f32, dead_zone: f32) -> (f32, f32) {
    let dz = dead_zone.clamp(0.0, 0.95);
    let magnitude = (x * x + y * y).sqrt();
    if magnitude < dz {
        return (0.0, 0.0);
    }
    let scale = ((magnitude - dz) / (1.0 - dz)).min(1.0) / magnitude;
    (x * scale, y * scale)
}

/// Dead zone simple para triggers (0.0..=1.0)
pub fn apply_trigger_dead_zone(value: f32, dead_zone: f32) -> f32 {
    if value < dead_zone {
        0.0
    } else {
        value.min(1.0)
    }
}

// ============================================================================
// GAMEPADMANAGER (SDL2)
// ============================================================================

#[cfg(feature = "sdl2-backend")]
pub use sdl_manager::{sdl_axis_to_ry, sdl_button_to_ry, GamepadManager};

#[cfg(feature = "sdl2-backend")]
mod sdl_manager {
    use super::{apply_radial_dead_zone, apply_trigger_dead_zone, GamepadState};
    use crate::input_event::{GamepadAxis, GamepadButton};
    use sdl2::controller::{Axis as SdlAxis, Button as SdlButton, GameController};
    use sdl2::event::Event;
    use sdl2::GameControllerSubsystem;
    use std::collections::HashMap;

    // ---- Conversiones SDL2 ↔ events-ry ----

    pub fn sdl_button_to_ry(button: SdlButton) -> GamepadButton {
        match button {
            SdlButton::A => GamepadButton::FaceDown,
            SdlButton::B => GamepadButton::FaceRight,
            SdlButton::X => GamepadButton::FaceLeft,
            SdlButton::Y => GamepadButton::FaceUp,
            SdlButton::LeftShoulder => GamepadButton::LeftShoulder,
            SdlButton::RightShoulder => GamepadButton::RightShoulder,
            SdlButton::LeftStick => GamepadButton::LeftStick,
            SdlButton::RightStick => GamepadButton::RightStick,
            SdlButton::Back => GamepadButton::Back,
            SdlButton::Start => GamepadButton::Start,
            SdlButton::DPadUp => GamepadButton::DPadUp,
            SdlButton::DPadDown => GamepadButton::DPadDown,
            SdlButton::DPadLeft => GamepadButton::DPadLeft,
            SdlButton::DPadRight => GamepadButton::DPadRight,
            _ => GamepadButton::FaceDown,
        }
    }

    pub fn sdl_axis_to_ry(axis: SdlAxis) -> GamepadAxis {
        match axis {
            SdlAxis::LeftX => GamepadAxis::LeftX,
            SdlAxis::LeftY => GamepadAxis::LeftY,
            SdlAxis::RightX => GamepadAxis::RightX,
            SdlAxis::RightY => GamepadAxis::RightY,
            SdlAxis::TriggerLeft => GamepadAxis::LeftTrigger,
            SdlAxis::TriggerRight => GamepadAxis::RightTrigger,
        }
    }

    fn ry_button_to_sdl(button: GamepadButton) -> SdlButton {
        match button {
            GamepadButton::FaceDown => SdlButton::A,
            GamepadButton::FaceRight => SdlButton::B,
            GamepadButton::FaceLeft => SdlButton::X,
            GamepadButton::FaceUp => SdlButton::Y,
            GamepadButton::LeftShoulder => SdlButton::LeftShoulder,
            GamepadButton::RightShoulder => SdlButton::RightShoulder,
            GamepadButton::LeftStick => SdlButton::LeftStick,
            GamepadButton::RightStick => SdlButton::RightStick,
            GamepadButton::Back => SdlButton::Back,
            GamepadButton::Start => SdlButton::Start,
            GamepadButton::DPadUp => SdlButton::DPadUp,
            GamepadButton::DPadDown => SdlButton::DPadDown,
            GamepadButton::DPadLeft => SdlButton::DPadLeft,
            GamepadButton::DPadRight => SdlButton::DPadRight,
        }
    }

    fn ry_axis_to_sdl(axis: GamepadAxis) -> SdlAxis {
        match axis {
            GamepadAxis::LeftX => SdlAxis::LeftX,
            GamepadAxis::LeftY => SdlAxis::LeftY,
            GamepadAxis::RightX => SdlAxis::RightX,
            GamepadAxis::RightY => SdlAxis::RightY,
            GamepadAxis::LeftTrigger => SdlAxis::TriggerLeft,
            GamepadAxis::RightTrigger => SdlAxis::TriggerRight,
        }
    }

    const ALL_BUTTONS: [GamepadButton; 14] = [
        GamepadButton::FaceDown,
        GamepadButton::FaceRight,
        GamepadButton::FaceLeft,
        GamepadButton::FaceUp,
        GamepadButton::LeftShoulder,
        GamepadButton::RightShoulder,
        GamepadButton::LeftStick,
        GamepadButton::RightStick,
        GamepadButton::Back,
        GamepadButton::Start,
        GamepadButton::DPadUp,
        GamepadButton::DPadDown,
        GamepadButton::DPadLeft,
        GamepadButton::DPadRight,
    ];

    const ALL_AXES: [GamepadAxis; 6] = [
        GamepadAxis::LeftX,
        GamepadAxis::LeftY,
        GamepadAxis::RightX,
        GamepadAxis::RightY,
        GamepadAxis::LeftTrigger,
        GamepadAxis::RightTrigger,
    ];

    // ---- Manager ----

    /// Gestión de gamepads SDL2: hotplug, estado por controller, dead zones y rumble.
    ///
    /// ```no_run
    /// # fn main() -> Result<(), String> {
    /// # let sdl = sdl2::init().map_err(|e| e.to_string())?;
    /// let subsystem = sdl.game_controller()?;
    /// let mut gamepads = events_ry::GamepadManager::new(subsystem, 0.15);
    /// gamepads.open_all()?;
    /// // Cada frame:
    /// // for ev in eventos { gamepads.handle_event(&ev); }
    /// gamepads.poll();
    /// if gamepads.is_button_down(events_ry::GamepadButton::FaceDown) { /* ... */ }
    /// # Ok(()) }
    /// ```
    pub struct GamepadManager {
        subsystem: GameControllerSubsystem,
        controllers: HashMap<u32, GameController>,
        states: HashMap<u32, GamepadState>,
        dead_zone: f32,
    }

    impl GamepadManager {
        /// Crear manager. `dead_zone` recomendado: 0.10–0.20
        pub fn new(subsystem: GameControllerSubsystem, dead_zone: f32) -> Self {
            Self {
                subsystem,
                controllers: HashMap::new(),
                states: HashMap::new(),
                dead_zone: dead_zone.clamp(0.0, 0.95),
            }
        }

        /// Abrir controller por joystick_index. Devuelve `instance_id`.
        /// Si el controller ya está abierto, devuelve su id sin duplicar.
        pub fn open(&mut self, joystick_index: u32) -> Result<u32, String> {
            let controller = self
                .subsystem
                .open(joystick_index)
                .map_err(|e| e.to_string())?;
            let instance_id = controller.instance_id();
            if self.controllers.contains_key(&instance_id) {
                return Ok(instance_id);
            }
            self.states.entry(instance_id).or_default();
            self.controllers.insert(instance_id, controller);
            Ok(instance_id)
        }

        /// Abrir todos los controllers conectados. Devuelve cuántos abrió.
        pub fn open_all(&mut self) -> Result<usize, String> {
            let n = self.subsystem.num_joysticks().map_err(|e| e.to_string())?;
            let mut opened = 0;
            for i in 0..n {
                if self.subsystem.is_game_controller(i) && self.open(i).is_ok() {
                    opened += 1;
                }
            }
            Ok(opened)
        }

        /// Cerrar un controller (por `instance_id`)
        pub fn close(&mut self, instance_id: u32) {
            self.controllers.remove(&instance_id);
            self.states.remove(&instance_id);
        }

        /// Cerrar todos los controllers
        pub fn close_all(&mut self) {
            self.controllers.clear();
            self.states.clear();
        }

        /// Procesar evento SDL2 (hotplug + botones + ejes).
        /// Llamar con cada evento crudo antes de descartarlo.
        pub fn handle_event(&mut self, event: &Event) {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    let _ = self.open(*which);
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    self.close(*which);
                }
                Event::ControllerButtonDown { which, button, .. } => {
                    let b = sdl_button_to_ry(*button);
                    self.update_button(*which, b, true);
                }
                Event::ControllerButtonUp { which, button, .. } => {
                    let b = sdl_button_to_ry(*button);
                    self.update_button(*which, b, false);
                }
                Event::ControllerAxisMotion {
                    which, axis, value, ..
                } => {
                    let a = sdl_axis_to_ry(*axis);
                    let v = *value as f32 / 32767.0;
                    self.update_axis(*which, a, v);
                }
                _ => {}
            }
        }

        /// Registrar botón en el estado (si el controller no existe, lo crea)
        pub fn update_button(&mut self, instance_id: u32, button: GamepadButton, pressed: bool) {
            self.states
                .entry(instance_id)
                .or_default()
                .set_button(button, pressed);
        }

        /// Registrar eje en el estado (valor crudo -1.0..=1.0)
        pub fn update_axis(&mut self, instance_id: u32, axis: GamepadAxis, value: f32) {
            self.states
                .entry(instance_id)
                .or_default()
                .set_axis(axis, value);
        }

        /// Refrescar estado desde hardware y purgar controllers desconectados.
        /// Llamar una vez por frame.
        pub fn poll(&mut self) {
            // Purgar desconectados
            let detached: Vec<u32> = self
                .controllers
                .iter()
                .filter(|(_, c)| !c.attached())
                .map(|(id, _)| *id)
                .collect();
            for id in detached {
                self.close(id);
            }

            // Refrescar estados desde hardware
            let updates: Vec<(u32, GamepadState)> = self
                .controllers
                .iter()
                .map(|(id, c)| {
                    let mut st = GamepadState::new();
                    for b in ALL_BUTTONS {
                        st.set_button(b, c.button(ry_button_to_sdl(b)));
                    }
                    for a in ALL_AXES {
                        st.set_axis(a, c.axis(ry_axis_to_sdl(a)) as f32 / 32767.0);
                    }
                    (*id, st)
                })
                .collect();
            for (id, st) in updates {
                self.states.insert(id, st);
            }
        }

        /// Controllers conectados: `(instance_id, nombre)`
        pub fn connected(&self) -> Vec<(u32, String)> {
            self.controllers
                .iter()
                .map(|(id, c)| (*id, c.name()))
                .collect()
        }

        /// Número de controllers conectados
        pub fn count(&self) -> usize {
            self.controllers.len()
        }

        /// ¿Hay algún controller conectado?
        pub fn is_any_connected(&self) -> bool {
            !self.controllers.is_empty()
        }

        /// Estado bruto de un controller (sin dead zone)
        pub fn state(&self, instance_id: u32) -> Option<&GamepadState> {
            self.states.get(&instance_id)
        }

        /// ¿Botón presionado en algún controller?
        pub fn is_button_down(&self, button: GamepadButton) -> bool {
            self.states.values().any(|s| s.is_button_down(button))
        }

        /// ¿Botón presionado en un controller específico?
        pub fn is_button_down_on(&self, instance_id: u32, button: GamepadButton) -> bool {
            self.states
                .get(&instance_id)
                .map_or(false, |s| s.is_button_down(button))
        }

        /// Valor de eje con dead zone (controller con mayor magnitud)
        pub fn axis_value(&self, axis: GamepadAxis) -> f32 {
            self.states
                .values()
                .map(|s| self.axis_with_dead_zone(s, axis))
                .max_by(|a, b| {
                    a.abs()
                        .partial_cmp(&b.abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(0.0)
        }

        /// Valor de eje con dead zone de un controller específico
        pub fn axis_value_on(&self, instance_id: u32, axis: GamepadAxis) -> f32 {
            self.states
                .get(&instance_id)
                .map(|s| self.axis_with_dead_zone(s, axis))
                .unwrap_or(0.0)
        }

        fn axis_with_dead_zone(&self, state: &GamepadState, axis: GamepadAxis) -> f32 {
            match axis {
                GamepadAxis::LeftX | GamepadAxis::LeftY => {
                    let (x, y) = apply_radial_dead_zone(
                        state.axis(GamepadAxis::LeftX),
                        state.axis(GamepadAxis::LeftY),
                        self.dead_zone,
                    );
                    if axis == GamepadAxis::LeftX {
                        x
                    } else {
                        y
                    }
                }
                GamepadAxis::RightX | GamepadAxis::RightY => {
                    let (x, y) = apply_radial_dead_zone(
                        state.axis(GamepadAxis::RightX),
                        state.axis(GamepadAxis::RightY),
                        self.dead_zone,
                    );
                    if axis == GamepadAxis::RightX {
                        x
                    } else {
                        y
                    }
                }
                GamepadAxis::LeftTrigger | GamepadAxis::RightTrigger => {
                    apply_trigger_dead_zone(state.axis(axis), self.dead_zone)
                }
            }
        }

        /// Dead zone activa
        pub fn dead_zone(&self) -> f32 {
            self.dead_zone
        }

        /// Cambiar dead zone en caliente
        pub fn set_dead_zone(&mut self, dead_zone: f32) {
            self.dead_zone = dead_zone.clamp(0.0, 0.95);
        }

        /// Vibrar un controller. `duration_ms` en milisegundos.
        /// `low`: frecuencia baja (motores grandes), `high`: alta (motores pequeños).
        pub fn rumble(
            &mut self,
            instance_id: u32,
            low: u16,
            high: u16,
            duration_ms: u32,
        ) -> Result<(), String> {
            self.controllers
                .get_mut(&instance_id)
                .ok_or_else(|| format!("gamepad {instance_id} no conectado"))?
                .set_rumble(low, high, duration_ms)
                .map_err(|e| e.to_string())
        }

        /// Vibrar todos los controllers. Devuelve cuántos respondieron.
        pub fn rumble_all(&mut self, low: u16, high: u16, duration_ms: u32) -> usize {
            let mut ok = 0;
            for c in self.controllers.values_mut() {
                if c.set_rumble(low, high, duration_ms).is_ok() {
                    ok += 1;
                }
            }
            ok
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamepad_state_buttons() {
        let mut st = GamepadState::new();
        assert!(!st.is_button_down(GamepadButton::FaceDown));
        st.set_button(GamepadButton::FaceDown, true);
        assert!(st.is_button_down(GamepadButton::FaceDown));
        st.set_button(GamepadButton::FaceDown, false);
        assert!(!st.is_button_down(GamepadButton::FaceDown));
    }

    #[test]
    fn test_gamepad_state_axes_clamp() {
        let mut st = GamepadState::new();
        st.set_axis(GamepadAxis::LeftX, 2.5);
        assert_eq!(st.axis(GamepadAxis::LeftX), 1.0);
        st.set_axis(GamepadAxis::LeftY, -3.0);
        assert_eq!(st.axis(GamepadAxis::LeftY), -1.0);
    }

    #[test]
    fn test_radial_dead_zone() {
        // Dentro de la dead zone → 0
        let (x, y) = apply_radial_dead_zone(0.05, 0.05, 0.15);
        assert_eq!((x, y), (0.0, 0.0));

        // Fuera → magnitud proporcional
        let (x, y) = apply_radial_dead_zone(1.0, 0.0, 0.15);
        assert!((x - 1.0).abs() < 1e-6);
        assert_eq!(y, 0.0);

        // Esquina: magnitud 1.0 → unitaria tras renormalizar
        let (x, y) = apply_radial_dead_zone(0.707, 0.707, 0.0);
        let m = (x * x + y * y).sqrt();
        assert!((m - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_trigger_dead_zone() {
        assert_eq!(apply_trigger_dead_zone(0.1, 0.15), 0.0);
        assert_eq!(apply_trigger_dead_zone(0.5, 0.15), 0.5);
        assert_eq!(apply_trigger_dead_zone(2.0, 0.15), 1.0);
    }
}
