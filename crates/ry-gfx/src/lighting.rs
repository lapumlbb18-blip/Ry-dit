//! # Iluminación 2D para Ry-Dit
//!
//! Luces puntuales + sombras básicas — compatible OpenGL ES 2.0 (Adreno 610).
//!
//! ## Filosofía
//! - Forward rendering simple (no deferred — too heavy for low-end)
//! - Luces puntuales con atenuación cuadrática
//! - Sombras básicas por distancia (sin raycasting aún)
//! - Mezcla aditiva para múltiples luces

use crate::ColorRydit;

// ============================================================================
// LIGHT 2D
// ============================================================================

/// Tipo de luz 2D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    /// Luz puntual (omnidireccional)
    Point,
    /// Luz direccional (como el sol)
    Directional,
    /// Luz spotlight (cono)
    Spot,
}

/// Luz 2D individual
#[derive(Debug, Clone)]
pub struct Light2D {
    pub light_type: LightType,
    /// Posición en el mundo (x, y)
    pub position: (f32, f32),
    /// Color de la luz (RGB)
    pub color: ColorRydit,
    /// Intensidad (0.0 - 1.0)
    pub intensity: f32,
    /// Radio de alcance en píxeles
    pub radius: f32,
    /// Ángulo de dirección (para spotlight, en radianes)
    pub angle: f32,
    /// Apertura del cono (para spotlight, en radianes)
    pub spot_angle: f32,
    /// Activa/desactivada
    pub enabled: bool,
}

impl Light2D {
    /// Crear luz puntual
    pub fn point(x: f32, y: f32, color: ColorRydit, radius: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position: (x, y),
            color,
            intensity: 1.0,
            radius,
            angle: 0.0,
            spot_angle: 0.0,
            enabled: true,
        }
    }

    /// Crear luz direccional (como el sol)
    pub fn directional(color: ColorRydit, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional,
            position: (0.0, 0.0),
            color,
            intensity,
            radius: 99999.0,
            angle: 0.0,
            spot_angle: 0.0,
            enabled: true,
        }
    }

    /// Crear spotlight (cono de luz)
    pub fn spot(x: f32, y: f32, color: ColorRydit, radius: f32, angle: f32, spot_angle: f32) -> Self {
        Self {
            light_type: LightType::Spot,
            position: (x, y),
            color,
            intensity: 1.0,
            radius,
            angle,
            spot_angle,
            enabled: true,
        }
    }

    /// Calcular factor de iluminación en un punto
    /// Retorna 0.0 (oscuro) a 1.0 (iluminado completo)
    pub fn illuminate(&self, px: f32, py: f32) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        match self.light_type {
            LightType::Point => {
                let dx = px - self.position.0;
                let dy = py - self.position.1;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist >= self.radius {
                    return 0.0;
                }
                // Atenuación cuadrática suave
                let t = 1.0 - dist / self.radius;
                (t * t * self.intensity).clamp(0.0, 1.0)
            }
            LightType::Directional => {
                // Luz direccional ilumina todo uniformemente
                self.intensity.clamp(0.0, 1.0)
            }
            LightType::Spot => {
                let dx = px - self.position.0;
                let dy = py - self.position.1;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist >= self.radius {
                    return 0.0;
                }
                // Calcular ángulo del punto relativo a la luz
                let point_angle = dy.atan2(dx);
                let mut angle_diff = point_angle - self.angle;
                // Normalizar a [-PI, PI]
                while angle_diff > std::f32::consts::PI {
                    angle_diff -= 2.0 * std::f32::consts::PI;
                }
                while angle_diff < -std::f32::consts::PI {
                    angle_diff += 2.0 * std::f32::consts::PI;
                }
                if angle_diff.abs() > self.spot_angle / 2.0 {
                    return 0.0;
                }
                // Falloff por ángulo y distancia
                let angle_falloff = 1.0 - (angle_diff / (self.spot_angle / 2.0)).abs();
                let dist_falloff = 1.0 - dist / self.radius;
                (angle_falloff * dist_falloff * self.intensity).clamp(0.0, 1.0)
            }
        }
    }
}

// ============================================================================
// AMBIENT LIGHT
// ============================================================================

/// Luz ambiental base
#[derive(Debug, Clone, Copy)]
pub struct AmbientLight {
    pub color: ColorRydit,
    /// Intensidad (0.0 = oscuridad total, 1.0 = luz completa)
    pub intensity: f32,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: ColorRydit::Blanco,
            intensity: 0.15, // 15% luz ambiental por defecto
        }
    }
}

// ============================================================================
// LIGHT MANAGER
// ============================================================================

/// Gestor de iluminación 2D
pub struct LightManager {
    pub ambient: AmbientLight,
    pub lights: Vec<Light2D>,
    /// Máximo de luces simultáneas (limitación rendimiento)
    pub max_lights: usize,
}

impl Default for LightManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LightManager {
    pub fn new() -> Self {
        Self {
            ambient: AmbientLight::default(),
            lights: Vec::new(),
            max_lights: 8, // 8 luces max para Adreno 610
        }
    }

    /// Agregar luz (retorna false si se alcanzó el máximo)
    pub fn add_light(&mut self, light: Light2D) -> bool {
        if self.lights.len() >= self.max_lights {
            return false;
        }
        self.lights.push(light);
        true
    }

    /// Remover luz por índice
    pub fn remove_light(&mut self, index: usize) {
        if index < self.lights.len() {
            self.lights.remove(index);
        }
    }

    /// Calcular iluminación total en un punto
    /// Retorna factor de 0.0 a 1.0
    pub fn compute_lighting(&self, px: f32, py: f32) -> f32 {
        let mut total = self.ambient.intensity;

        for light in &self.lights {
            total += light.illuminate(px, py);
        }

        total.clamp(0.0, 1.0)
    }

    /// Obtener color iluminado en un punto (retorna raylib::Color)
    pub fn compute_color_rgb(&self, px: f32, py: f32, base: raylib::prelude::Color) -> raylib::prelude::Color {
        let _ac = self.ambient.color.to_color();
        let mut r = base.r as f32 * self.ambient.intensity;
        let mut g = base.g as f32 * self.ambient.intensity;
        let mut b = base.b as f32 * self.ambient.intensity;

        for light in &self.lights {
            let factor = light.illuminate(px, py);
            let lc = light.color.to_color();
            r += lc.r as f32 * factor;
            g += lc.g as f32 * factor;
            b += lc.b as f32 * factor;
        }

        raylib::prelude::Color::new(
            (r as u8).min(255),
            (g as u8).min(255),
            (b as u8).min(255),
            base.a,
        )
    }

    /// Crear escena con luz de antorcha (efecto fuego)
    pub fn setup_torch_scene(&mut self, x: f32, y: f32) {
        self.ambient = AmbientLight {
            color: ColorRydit::Azul,
            intensity: 0.05, // Muy oscuro
        };
        self.lights.clear();
        // Antorcha cálida
        self.add_light(Light2D::point(x, y, ColorRydit::Naranja, 200.0));
    }

    /// Crear escena con luz diurna
    pub fn setup_daylight_scene(&mut self) {
        self.ambient = AmbientLight {
            color: ColorRydit::Blanco,
            intensity: 0.8,
        };
        self.lights.clear();
        self.add_light(Light2D::directional(ColorRydit::Amarillo, 0.4));
    }

    /// Crear escena nocturna con luna
    pub fn setup_night_scene(&mut self, moon_x: f32, moon_y: f32) {
        self.ambient = AmbientLight {
            color: ColorRydit::AzulOscuro,
            intensity: 0.08,
        };
        self.lights.clear();
        // Luna
        self.add_light(Light2D::point(moon_x, moon_y, ColorRydit::Blanco, 500.0));
    }

    /// Calcular color iluminado (helper con componentes individuales)
    fn blend_colors(base: ColorRydit, light_colors: &[(ColorRydit, f32)]) -> ColorRydit {
        let bc = base.to_color();
        let mut r = bc.r as f32 * 0.15; // ambient base
        let mut g = bc.g as f32 * 0.15;
        let mut b = bc.b as f32 * 0.15;
        for &(lc, factor) in light_colors {
            let c = lc.to_color();
            r += c.r as f32 * factor;
            g += c.g as f32 * factor;
            b += c.b as f32 * factor;
        }
        // Find closest ColorRydit
        let candidates = [
            ColorRydit::Rojo, ColorRydit::Verde, ColorRydit::Azul, ColorRydit::Amarillo,
            ColorRydit::Blanco, ColorRydit::Negro, ColorRydit::Gris, ColorRydit::Naranja,
        ];
        let mut best = ColorRydit::Blanco;
        let mut best_dist = 999999.0;
        for &c in &candidates {
            let tc = c.to_color();
            let d = (tc.r as f32 - r).powi(2) + (tc.g as f32 - g).powi(2) + (tc.b as f32 - b).powi(2);
            if d < best_dist {
                best_dist = d;
                best = c;
            }
        }
        best
    }

    /// Limpiar todas las luces
    pub fn clear(&mut self) {
        self.lights.clear();
    }

    /// Contar luces activas
    pub fn active_count(&self) -> usize {
        self.lights.iter().filter(|l| l.enabled).count()
    }
}

// ============================================================================
// SHADER GLSL PARA ILUMINACIÓN 2D
// ============================================================================

/// Fragment shader GLSL para iluminación multipoint 2D
/// Se usa con gpu_instancing o draw calls regulares.
pub const LIGHTING_FRAGMENT_SHADER: &str = r#"#version 100
precision mediump float;

varying vec2 v_uv;
varying vec4 v_color;

uniform vec2 u_resolution;
uniform vec2 u_lights[8];    // xy = posición normalizada
uniform vec3 u_light_colors[8]; // rgb = color
uniform float u_light_radii[8];  // radio normalizado
uniform float u_light_intensities[8];
uniform int u_light_count;
uniform vec3 u_ambient;

void main() {
    vec2 frag_pos = gl_FragCoord.xy / u_resolution;
    vec3 color = v_color.rgb * u_ambient;

    for (int i = 0; i < 8; i++) {
        if (i >= u_light_count) break;

        vec2 light_pos = u_lights[i];
        float dist = length(frag_pos - light_pos);
        float radius = u_light_radii[i];

        if (dist < radius) {
            float t = 1.0 - dist / radius;
            float attenuation = t * t * u_light_intensities[i];
            color += v_color.rgb * u_light_colors[i] * attenuation;
        }
    }

    gl_FragColor = vec4(color, v_color.a);
}
"#;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_light_falloff() {
        let light = Light2D::point(0.0, 0.0, ColorRydit::Blanco, 100.0);
        // Centro = iluminación máxima
        assert!(light.illuminate(0.0, 0.0) > 0.9);
        // Mitad del radio
        let mid = light.illuminate(50.0, 0.0);
        assert!(mid > 0.1 && mid < 0.6);
        // Fuera del radio
        assert_eq!(light.illuminate(150.0, 0.0), 0.0);
    }

    #[test]
    fn test_directional_light() {
        let light = Light2D::directional(ColorRydit::Blanco, 0.5);
        assert_eq!(light.illuminate(0.0, 0.0), 0.5);
        assert_eq!(light.illuminate(999.0, 999.0), 0.5);
    }

    #[test]
    fn test_spot_light() {
        let light = Light2D::spot(0.0, 0.0, ColorRydit::Blanco, 100.0, 0.0, std::f32::consts::PI / 2.0);
        // Dentro del cono
        assert!(light.illuminate(50.0, 0.0) > 0.0);
        // Fuera del cono (90 grados)
        assert_eq!(light.illuminate(0.0, 50.0), 0.0);
    }

    #[test]
    fn test_light_manager_add_remove() {
        let mut lm = LightManager::new();
        lm.max_lights = 3;

        assert!(lm.add_light(Light2D::point(0.0, 0.0, ColorRydit::Rojo, 50.0)));
        assert!(lm.add_light(Light2D::point(10.0, 10.0, ColorRydit::Verde, 50.0)));
        assert!(lm.add_light(Light2D::point(20.0, 20.0, ColorRydit::Azul, 50.0)));
        assert!(!lm.add_light(Light2D::point(30.0, 30.0, ColorRydit::Blanco, 50.0)));

        lm.remove_light(1);
        assert_eq!(lm.lights.len(), 2);
        assert!(lm.add_light(Light2D::point(30.0, 30.0, ColorRydit::Blanco, 50.0)));
    }

    #[test]
    fn test_compute_lighting() {
        let mut lm = LightManager::new();
        lm.ambient.intensity = 0.1;
        lm.add_light(Light2D::point(50.0, 50.0, ColorRydit::Blanco, 100.0));

        let near = lm.compute_lighting(50.0, 50.0);
        let far = lm.compute_lighting(200.0, 200.0);
        assert!(near > far);
        assert!(near >= 0.1);
        assert!(far >= 0.09); // ambient
    }

    #[test]
    fn test_setup_scenes() {
        let mut lm = LightManager::new();
        lm.setup_torch_scene(100.0, 100.0);
        assert_eq!(lm.active_count(), 1);
        assert!(lm.ambient.intensity < 0.1);

        lm.setup_daylight_scene();
        assert_eq!(lm.active_count(), 1);
        assert!(lm.ambient.intensity > 0.5);

        lm.setup_night_scene(400.0, -200.0);
        assert_eq!(lm.active_count(), 1);
        assert!(lm.ambient.intensity < 0.2);
    }

    #[test]
    fn test_disabled_light() {
        let mut light = Light2D::point(0.0, 0.0, ColorRydit::Blanco, 100.0);
        light.enabled = false;
        assert_eq!(light.illuminate(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_light_type_enum() {
        assert_eq!(Light2D::point(0.0, 0.0, ColorRydit::Blanco, 50.0).light_type, LightType::Point);
        assert_eq!(Light2D::directional(ColorRydit::Blanco, 1.0).light_type, LightType::Directional);
        assert_eq!(Light2D::spot(0.0, 0.0, ColorRydit::Blanco, 50.0, 0.0, 1.0).light_type, LightType::Spot);
    }
}
