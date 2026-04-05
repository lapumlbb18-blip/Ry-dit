//! Ilusiones Ópticas Animadas
//!
//! Generadores de ilusiones ópticas que cambian con el tiempo,
//! creando efectos de "confusión" visual para el ojo humano.
//!
//! ## Ilusiones implementadas
//!
//! - Rotating Snakes: ilusión de movimiento circular
//! - Cafe Wall: líneas paralelas que parecen inclinadas
//! - Troxler Fading: desvanecimiento por fijación visual
//! - Pulsing Star: estrella que pulsa
//! - Zöllner Effect: líneas paralelas con segmentos sesgados

use serde_json::{json, Value};

// ============================================================================
// ROTATING SNAKES — Ilusión de movimiento circular
// ============================================================================

/// Rotating Snakes — Genera segmentos de anillo que parecen rotar
///
/// # Args
/// - cx, cy: centro del patrón
/// - radius: radio del anillo
/// - segments: número de segmentos (8-32)
/// - t: tiempo (simula rotación)
/// - colors: array de colores ["c1", "c2", "c3", "c4"]
///
/// # Retorna
/// Array de segmentos [{x, y, angle, color}, ...]
pub fn rotating_snakes(cx: f64, cy: f64, radius: f64, segments: usize, t: f64, colors: &[String]) -> Vec<Value> {
    let segments = segments.max(8).min(64);
    let colors = if colors.is_empty() {
        &["#000000".to_string(), "#FFFFFF".to_string(), "#0000FF".to_string(), "#FFFF00".to_string()]
    } else {
        colors
    };
    let mut result = Vec::new();

    for i in 0..segments {
        let angle_base = (i as f64 / segments as f64) * std::f64::consts::PI * 2.0;
        let angle = angle_base + t * 0.5; // Rotación simulada
        let color_idx = (i + (t * 3.0) as usize) % colors.len();

        // Calcular posición del segmento
        let seg_angle = std::f64::consts::PI * 2.0 / segments as f64;
        let x = cx + (angle.cos()) * radius;
        let y = cy + (angle.sin()) * radius;

        result.push(json!({
            "type": "segment",
            "x": x,
            "y": y,
            "angle": angle_base,
            "arc": seg_angle,
            "color": colors[color_idx]
        }));
    }

    result
}

// ============================================================================
// CAFE WALL — Líneas paralelas que parecen inclinadas
// ============================================================================

/// Cafe Wall Animation — Genera filas desplazadas que parecen inclinadas
///
/// # Args
/// - start_x, start_y: esquina superior izquierda
/// - rows: número de filas
/// - cols: número de columnas (ladrillos)
/// - brick_w: ancho de cada ladrillo
/// - brick_h: alto de cada ladrillo
/// - mortar: grosor de la línea entre filas
/// - offset_t: desplazamiento temporal (0.0-1.0)
///
/// # Retorna
/// Array de rectángulos [{x, y, w, h, color}, ...] + líneas de mortero
pub fn cafe_wall(start_x: f64, start_y: f64, rows: usize, cols: usize,
                 brick_w: f64, brick_h: f64, mortar: f64, offset_t: f64) -> Vec<Value> {
    let mut result = Vec::new();
    let offset = offset_t * brick_w * 0.5; // Desplazamiento máximo = medio ladrillo

    for row in 0..rows {
        let row_offset = if row % 2 == 0 { 0.0 } else { offset };
        let y = start_y + row as f64 * (brick_h + mortar);

        // Línea de mortero
        result.push(json!({
            "type": "line",
            "x1": start_x, "y1": y - mortar / 2.0,
            "x2": start_x + cols as f64 * brick_w,
            "y2": y - mortar / 2.0,
            "color": "#808080"
        }));

        for col in 0..cols {
            let x = start_x + col as f64 * brick_w + row_offset;
            // Patrón de colores alternados para maximizar la ilusión
            let is_dark = (row + col) % 2 == 0;
            let color = if is_dark { "#000000" } else { "#FFFFFF" };

            result.push(json!({
                "type": "rect",
                "x": x, "y": y,
                "w": brick_w - mortar, "h": brick_h,
                "color": color
            }));
        }
    }

    result
}

// ============================================================================
// TROXLER FADING — Desvanecimiento por fijación visual
// ============================================================================

/// Troxler Fading — Círculos periféricos que se desvanecen al mirar el centro
///
/// # Args
/// - cx, cy: punto de fijación central
/// - num_circles: número de círculos periféricos (8-16)
/// - radius: distancia del centro a los círculos
/// - circle_size: tamaño de cada círculo
/// - t: tiempo de animación (0.0-1.0 ciclo)
///
/// # Retorna
/// Array de círculos [{x, y, size, alpha, color}, ...] + punto de fijación
pub fn troxler_fading(cx: f64, cy: f64, num_circles: usize, radius: f64,
                      circle_size: f64, t: f64) -> Vec<Value> {
    let num_circles = num_circles.max(8).min(24);
    let mut result = Vec::new();

    // Punto de fijación central (siempre visible)
    result.push(json!({
        "type": "circle",
        "x": cx, "y": cy,
        "radius": 5.0,
        "color": "#FF0000",
        "alpha": 1.0
    }));

    for i in 0..num_circles {
        let angle = (i as f64 / num_circles as f64) * std::f64::consts::PI * 2.0;
        let x = cx + angle.cos() * radius;
        let y = cy + angle.sin() * radius;

        // Alpha varía con el tiempo — círculos "desaparecen" periódicamente
        // El desfase depende de la posición angular
        let phase = angle + t * std::f64::consts::PI * 2.0;
        let alpha = (phase.sin() * 0.5 + 0.5).max(0.05); // Nunca desaparece completamente

        result.push(json!({
            "type": "circle",
            "x": x, "y": y,
            "radius": circle_size,
            "color": "#A080FF",
            "alpha": alpha
        }));
    }

    result
}

// ============================================================================
// PULSING STAR — Estrella que pulsa
// ============================================================================

/// Pulsing Star — Estrella con efecto de pulsación rítmica
///
/// # Args
/// - cx, cy: centro
/// - outer_radius: radio exterior
/// - inner_radius: radio interior
/// - points: número de puntas
/// - t: tiempo de animación
///
/// # Retorna
/// Array de puntos [{x, y}, ...] + centro
pub fn pulsing_star(cx: f64, cy: f64, outer_radius: f64, inner_radius: f64,
                    points: usize, t: f64) -> Vec<Value> {
    let points = points.max(4).min(20);
    let mut result = Vec::new();
    let pulse = 1.0 + 0.2 * (t * std::f64::consts::PI * 4.0).sin(); // Pulso suave
    let outer_r = outer_radius * pulse;
    let inner_r = inner_radius * pulse;

    // Puntas de la estrella
    for i in 0..(points * 2) {
        let angle = (i as f64 / (points * 2) as f64) * std::f64::consts::PI * 2.0 - std::f64::consts::PI / 2.0;
        let r = if i % 2 == 0 { outer_r } else { inner_r };
        let x = cx + angle.cos() * r;
        let y = cy + angle.sin() * r;

        result.push(json!({ "x": x, "y": y }));
    }

    result
}

// ============================================================================
// ZÖLLNER EFFECT — Líneas paralelas con segmentos sesgados
// ============================================================================

/// Zöllner Effect — Líneas paralelas que parecen no serlo
///
/// # Args
/// - start_x, start_y: inicio
/// - line_length: largo de las líneas principales
/// - line_spacing: espacio entre líneas
/// - num_lines: número de líneas
/// - tick_length: largo de los segmentos sesgados
/// - tick_angle: ángulo de los segmentos (en radianes)
/// - t: tiempo (para animación)
///
/// # Retorna
/// Array de líneas [{x1, y1, x2, y2, color}, ...]
pub fn zollner_effect(start_x: f64, start_y: f64, line_length: f64, line_spacing: f64,
                      num_lines: usize, tick_length: f64, tick_angle: f64, t: f64) -> Vec<Value> {
    let mut result = Vec::new();
    let animated_angle = tick_angle + t * 0.3; // Animación suave del ángulo
    let cos_a = animated_angle.cos();
    let sin_a = animated_angle.sin();

    for i in 0..num_lines {
        let y = start_y + i as f64 * line_spacing;
        let x_start = start_x;
        let x_end = start_x + line_length;

        // Línea principal (horizontal)
        result.push(json!({
            "type": "line",
            "x1": x_start, "y1": y,
            "x2": x_end, "y2": y,
            "color": "#FFFFFF",
            "thickness": 2.0
        }));

        // Segmentos sesgados
        let num_ticks = (line_length / (tick_length * 2.0)) as usize;
        let alternating = i % 2 == 0;

        for j in 0..num_ticks {
            let tx = x_start + j as f64 * tick_length * 2.0 + tick_length;
            let sign = if alternating { 1.0 } else { -1.0 };

            let dx = cos_a * tick_length * sign;
            let dy = sin_a * tick_length * sign;

            result.push(json!({
                "type": "line",
                "x1": tx, "y1": y,
                "x2": tx + dx, "y2": y + dy,
                "color": "#A0A0FF",
                "thickness": 1.0
            }));
        }
    }

    result
}

// ============================================================================
// MOTION-INDUCED BLINDNESS — Puntos que desaparecen
// ============================================================================

/// Motion-Induced Blindness — Puntos que desaparecen al mirar fijamente
///
/// # Args
/// - cx, cy: centro
/// - grid_size: tamaño de la cuadrícula (8-20)
/// - spacing: espacio entre puntos
/// - dot_size: tamaño de cada punto
/// - t: tiempo de animación
///
/// # Retorna
/// Array de puntos [{x, y, visible, color}, ...] + cruz de fijación
pub fn motion_induced_blindness(cx: f64, cy: f64, grid_size: usize, spacing: f64,
                                dot_size: f64, t: f64) -> Vec<Value> {
    let grid_size = grid_size.max(6).min(24);
    let mut result = Vec::new();

    // Cruz de fijación central
    result.push(json!({
        "type": "cross",
        "x": cx, "y": cy,
        "size": 15.0,
        "color": "#FF0000"
    }));

    // Cuadrícula de puntos azules
    let total_offset = (grid_size - 1) as f64 * spacing / 2.0;

    for row in 0..grid_size {
        for col in 0..grid_size {
            let x = cx + col as f64 * spacing - total_offset;
            let y = cy + row as f64 * spacing - total_offset;

            // Distancia al centro
            let dx = x - cx;
            let dy = y - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            // Los puntos desaparecen basados en la distancia y el tiempo
            let phase = dist * 0.05 - t * std::f64::consts::PI * 2.0;
            let visible = (phase.sin() + 1.0) / 2.0 > 0.3; // 70% del tiempo visible

            if visible {
                result.push(json!({
                    "type": "dot",
                    "x": x, "y": y,
                    "size": dot_size,
                    "color": "#4444FF"
                }));
            }
        }
    }

    result
}

// ============================================================================
// HELPERS
// ============================================================================

/// Rotar un punto alrededor de un centro
fn rotate_point(px: f64, py: f64, cx: f64, cy: f64, angle: f64) -> (f64, f64) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let dx = px - cx;
    let dy = py - cy;
    (cx + dx * cos_a - dy * sin_a, cy + dx * sin_a + dy * cos_a)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotating_snakes() {
        let colors = vec!["black".to_string(), "white".to_string()];
        let result = rotating_snakes(400.0, 300.0, 100.0, 16, 0.5, &colors);
        assert_eq!(result.len(), 16);
        assert!(result[0].get("type").is_some());
    }

    #[test]
    fn test_cafe_wall() {
        let result = cafe_wall(0.0, 0.0, 4, 8, 30.0, 15.0, 2.0, 0.5);
        assert!(!result.is_empty());
        // Debe tener rectángulos + líneas de mortero
        assert!(result.len() > 4 * 8);
    }

    #[test]
    fn test_troxler_fading() {
        let result = troxler_fading(400.0, 300.0, 12, 100.0, 10.0, 0.5);
        assert_eq!(result.len(), 13); // 12 círculos + 1 punto de fijación
    }

    #[test]
    fn test_pulsing_star() {
        let result = pulsing_star(400.0, 300.0, 50.0, 25.0, 5, 0.5);
        assert_eq!(result.len(), 10); // 5 puntas × 2 (outer + inner)
    }

    #[test]
    fn test_zollner_effect() {
        let result = zollner_effect(50.0, 50.0, 700.0, 40.0, 10, 15.0, 0.5, 0.0);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_motion_induced_blindness() {
        let result = motion_induced_blindness(400.0, 300.0, 10, 30.0, 5.0, 0.5);
        assert!(!result.is_empty()); // Al menos la cruz de fijación
    }

    #[test]
    fn test_rotate_point() {
        let (x, y) = rotate_point(1.0, 0.0, 0.0, 0.0, std::f64::consts::PI / 2.0);
        assert!((x - 0.0).abs() < 0.01);
        assert!((y - 1.0).abs() < 0.01);
    }
}
