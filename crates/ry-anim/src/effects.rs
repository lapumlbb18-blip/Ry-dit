//! Efectos Especiales para Animación
//!
//! Efectos visuales que mejoran la apariencia de animaciones y juegos.
//!
//! ## Efectos implementados
//!
//! - Neon Glow: Resplandor neón configurable
//! - Motion Blur: Desenfoque de movimiento
//! - Chromatic Aberration: Separación RGB en bordes
//! - Bloom: Brillo difuso en zonas claras
//! - Particle Trails: Estelas de partículas
//! - Morphing: Transición suave entre formas

use serde_json::{json, Value};

// ============================================================================
// NEON GLOW — Resplandor neón
// ============================================================================

/// Neon Glow — Genera capas de resplandor alrededor de un objeto
///
/// # Args
/// - cx, cy: centro del objeto
/// - base_radius: radio del objeto base
/// - glow_layers: número de capas de glow (3-8)
/// - glow_spread: cuánto se expande cada capa (1.5-3.0)
/// - intensity: intensidad del glow (0.3-1.0)
/// - base_color: color base en hex
/// - t: tiempo para animación de pulso
///
/// # Retorna
/// Array de círculos [{x, y, radius, color, alpha}, ...]
pub fn neon_glow(cx: f64, cy: f64, base_radius: f64, glow_layers: usize,
                 glow_spread: f64, intensity: f64, base_color: &str, t: f64) -> Vec<Value> {
    let glow_layers = glow_layers.clamp(3, 12);
    let glow_spread = glow_spread.clamp(1.2, 4.0);
    let intensity = intensity.clamp(0.1, 1.0);
    let pulse = 1.0 + 0.1 * (t * 3.0).sin(); // Pulso suave

    let mut result = Vec::new();

    // Capas de glow (de afuera hacia adentro)
    for i in (1..=glow_layers).rev() {
        let radius = base_radius * glow_spread.powi(i as i32) * pulse;
        let alpha = (intensity / (i as f64)).min(1.0) * 0.4;

        result.push(json!({
            "type": "glow_ring",
            "x": cx, "y": cy,
            "radius": radius,
            "color": base_color,
            "alpha": alpha
        }));
    }

    // Objeto central (brillante)
    result.push(json!({
        "type": "core",
        "x": cx, "y": cy,
        "radius": base_radius * pulse,
        "color": "#FFFFFF",
        "alpha": 1.0
    }));

    result
}

// ============================================================================
// MOTION BLUR — Desenfoque de movimiento
// ============================================================================

/// Motion Blur — Genera estelas de movimiento basadas en velocidad
///
/// # Args
/// - prev_positions: array de posiciones anteriores [(x,y), ...]
/// - current_pos: posición actual (x, y)
/// - blur_intensity: cuántas copias generar (0.3-1.0)
/// - fade_rate: qué tan rápido se desvanecen las copias (0.5-0.95)
///
/// # Retorna
/// Array de copias [{x, y, alpha, scale}, ...]
pub fn motion_blur(prev_positions: &[(f64, f64)], current_pos: (f64, f64),
                   blur_intensity: f64, fade_rate: f64) -> Vec<Value> {
    let blur_intensity = blur_intensity.clamp(0.1, 1.0);
    let fade_rate = fade_rate.clamp(0.3, 0.98);
    let max_blurs = (blur_intensity * 20.0) as usize;

    let mut result = Vec::new();

    // Posición actual (sin blur, alpha completo)
    result.push(json!({
        "type": "sharp",
        "x": current_pos.0,
        "y": current_pos.1,
        "alpha": 1.0,
        "scale": 1.0
    }));

    // Copias borrosas de posiciones anteriores
    let blurs = prev_positions.len().min(max_blurs);
    for i in 0..blurs {
        let (px, py) = prev_positions[prev_positions.len() - 1 - i];
        let alpha = fade_rate.powi((i + 1) as i32);
        let scale = 1.0 + 0.05 * (i as f64); // Las copias son ligeramente más grandes

        result.push(json!({
            "type": "blur_copy",
            "x": px,
            "y": py,
            "alpha": alpha,
            "scale": scale
        }));
    }

    result
}

// ============================================================================
// CHROMATIC ABERRATION — Separación RGB
// ============================================================================

/// Chromatic Aberration — Separa un objeto en canales R, G, B
///
/// # Args
/// - cx, cy: centro original
/// - radius: tamaño del objeto
/// - separation: cuánto separar los canales (0-20px)
/// - t: tiempo para animación
/// - shape_type: "circle", "rect", "star"
///
/// # Retorna
/// Array de canales [{channel, x, y, radius/size, color}, ...]
pub fn chromatic_aberration(cx: f64, cy: f64, radius: f64, separation: f64,
                            t: f64, shape_type: &str) -> Vec<Value> {
    let sep = separation.clamp(0.0, 30.0);
    // Dirección de separación basada en ángulo animado
    let angle = t * 0.5;
    let dx = angle.cos() * sep;
    let dy = angle.sin() * sep;

    let mut result = Vec::new();

    // Canal Rojo (desplazado)
    result.push(json!({
        "type": shape_type,
        "channel": "red",
        "x": cx - dx, "y": cy - dy,
        "radius": radius,
        "color": "#FF0000"
    }));

    // Canal Verde (centro)
    result.push(json!({
        "type": shape_type,
        "channel": "green",
        "x": cx, "y": cy,
        "radius": radius,
        "color": "#00FF00"
    }));

    // Canal Azul (desplazado opuesto)
    result.push(json!({
        "type": shape_type,
        "channel": "blue",
        "x": cx + dx, "y": cy + dy,
        "radius": radius,
        "color": "#0000FF"
    }));

    result
}

// ============================================================================
// BLOOM — Brillo difuso
// ============================================================================

/// Bloom — Efecto de brillo difuso en zonas claras
///
/// # Args
/// - sources: array de fuentes de luz [{x, y, intensity, radius}, ...]
/// - bloom_radius: radio del bloom
/// - bloom_intensity: intensidad del bloom
/// - t: tiempo para parpadeo
///
/// # Retorna
/// Array de halos de bloom [{x, y, radius, alpha, color}, ...]
pub fn bloom_effect(sources: &[(f64, f64, f64, f64)], bloom_radius: f64,
                    bloom_intensity: f64, t: f64) -> Vec<Value> {
    let bloom_radius = bloom_radius.max(5.0);
    let bloom_intensity = bloom_intensity.clamp(0.1, 1.0);

    let mut result = Vec::new();

    for (i, (x, y, intensity, radius)) in sources.iter().enumerate() {
        let flicker = 1.0 + 0.1 * ((t * 4.0 + i as f64) as f64).sin();
        let effective_intensity = intensity * bloom_intensity * flicker;
        let effective_radius = radius + bloom_radius * effective_intensity;

        // Halo exterior
        result.push(json!({
            "type": "bloom_halo",
            "x": x, "y": y,
            "radius": effective_radius,
            "alpha": effective_intensity * 0.5,
            "color": "#FFFFAA"
        }));

        // Centro brillante
        result.push(json!({
            "type": "bloom_core",
            "x": x, "y": y,
            "radius": radius,
            "alpha": intensity,
            "color": "#FFFFFF"
        }));
    }

    result
}

// ============================================================================
// PARTICLE TRAILS — Estelas de partículas
// ============================================================================

/// Particle Trails — Genera estela detrás de partículas en movimiento
///
/// # Args
/// - positions: array de posiciones actuales [(x, y, vx, vy), ...]
/// - trail_length: largo de la estela (5-30)
/// - trail_fade: qué tan rápido se desvanece (0.7-0.98)
/// - trail_color: color de la estela
///
/// # Retorna
/// Array de puntos de estela [{x, y, size, alpha, color}, ...]
pub fn particle_trails(positions: &[(f64, f64, f64, f64)], trail_length: usize,
                       trail_fade: f64, trail_color: &str) -> Vec<Value> {
    let trail_length = trail_length.clamp(3, 50);
    let trail_fade = trail_fade.clamp(0.5, 0.99);

    let mut result = Vec::new();

    for (px, py, vx, vy) in positions {
        let speed = (vx * vx + vy * vy).sqrt();

        for i in 0..trail_length {
            let trail_x = px - vx * (i as f64) * 0.3;
            let trail_y = py - vy * (i as f64) * 0.3;
            let alpha = trail_fade.powi((i + 1) as i32);
            let size = (3.0 - i as f64 * 0.1).max(0.5);

            result.push(json!({
                "type": "trail_dot",
                "x": trail_x,
                "y": trail_y,
                "size": size * (speed / 5.0).min(2.0),
                "alpha": alpha,
                "color": trail_color
            }));
        }

        // Partícula principal
        result.push(json!({
            "type": "particle",
            "x": px, "y": py,
            "size": 4.0,
            "alpha": 1.0,
            "color": "#FFFFFF"
        }));
    }

    result
}

// ============================================================================
// MORPHING — Transición entre formas
// ============================================================================

/// Morphing — Transición suave entre dos formas
///
/// # Args
/// - shape_a: puntos de la forma A [{x, y}, ...]
/// - shape_b: puntos de la forma B [{x, y}, ...]
/// - t: progreso de la transición (0.0 = forma A, 1.0 = forma B)
/// - easing: función de easing ("linear", "ease_in", "ease_out", "ease_in_out")
///
/// # Retorna
/// Array de puntos interpolados [{x, y}, ...]
pub fn morph_shapes(shape_a: &[(f64, f64)], shape_b: &[(f64, f64)],
                    t: f64, easing: &str) -> Vec<Value> {
    let t = t.clamp(0.0, 1.0);

    // Aplicar easing
    let et = match easing {
        "ease_in" => t * t,
        "ease_out" => t * (2.0 - t),
        "ease_in_out" => if t < 0.5 { 2.0 * t * t } else { 1.0 - 2.0 * (1.0 - t) * (1.0 - t) },
        _ => t, // linear
    };

    let len = shape_a.len().max(shape_b.len());
    let mut result = Vec::new();

    for i in 0..len {
        let ax = shape_a.get(i % shape_a.len()).map(|p| p.0).unwrap_or(0.0);
        let ay = shape_a.get(i % shape_a.len()).map(|p| p.1).unwrap_or(0.0);
        let bx = shape_b.get(i % shape_b.len()).map(|p| p.0).unwrap_or(0.0);
        let by = shape_b.get(i % shape_b.len()).map(|p| p.1).unwrap_or(0.0);

        let x = ax + (bx - ax) * et;
        let y = ay + (by - ay) * et;

        result.push(json!({ "x": x, "y": y }));
    }

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neon_glow() {
        let result = neon_glow(400.0, 300.0, 20.0, 5, 2.0, 0.8, "#FF00FF", 0.5);
        assert_eq!(result.len(), 6); // 5 capas glow + 1 core
        assert!(result[0].get("radius").is_some());
    }

    #[test]
    fn test_motion_blur() {
        let prev = vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)];
        let result = motion_blur(&prev, (30.0, 0.0), 0.8, 0.8);
        assert!(result.len() > 1); // Al menos la posición actual + copias
    }

    #[test]
    fn test_chromatic_aberration() {
        let result = chromatic_aberration(400.0, 300.0, 30.0, 10.0, 0.5, "circle");
        assert_eq!(result.len(), 3); // R, G, B
        let red = &result[0];
        let blue = &result[2];
        assert!(red.get("x").unwrap().as_f64().unwrap() < blue.get("x").unwrap().as_f64().unwrap());
    }

    #[test]
    fn test_bloom_effect() {
        let sources = vec![(400.0, 300.0, 1.0, 10.0)];
        let result = bloom_effect(&sources, 50.0, 0.8, 0.5);
        assert!(result.len() >= 2); // Halo + core
    }

    #[test]
    fn test_particle_trails() {
        let positions = vec![(100.0, 100.0, 5.0, 0.0)];
        let result = particle_trails(&positions, 10, 0.85, "#FFAA00");
        assert!(result.len() > 10); // 10 trail dots + 1 particle
    }

    #[test]
    fn test_morph_shapes() {
        let shape_a = vec![(0.0, 0.0), (100.0, 0.0), (50.0, 100.0)]; // Triángulo
        let shape_b = vec![(0.0, 0.0), (100.0, 0.0), (100.0, 100.0), (0.0, 100.0)]; // Cuadrado
        let result = morph_shapes(&shape_a, &shape_b, 0.5, "linear");
        assert_eq!(result.len(), 4); // Max de los dos shapes
    }

    #[test]
    fn test_morph_easing() {
        let a = vec![(0.0, 0.0)];
        let b = vec![(100.0, 0.0)];

        let linear = morph_shapes(&a, &b, 0.5, "linear");
        let ease_in = morph_shapes(&a, &b, 0.5, "ease_in");

        // ease_in debe estar más cerca de A (más lento al inicio)
        assert!(linear[0].get("x").unwrap().as_f64().unwrap() > ease_in[0].get("x").unwrap().as_f64().unwrap());
    }
}
