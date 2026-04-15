// crates/ry-science/src/lsystem.rs
// L-System (Lindenmayer System) — Árboles, plantas, coral, helechos procedurales
//
// Fórmula: Axioma → Reglas de reescritura iterativa
// Ejemplo: Árbol binario
//   Axioma: "0"
//   Reglas: "0" → "1[+0][-0]", "1" → "11"
//   Ángulo: 25°

use serde_json::{json, Value};

// ============================================================================
// L-SYSTEM CORE
// ============================================================================

/// Reescribir string usando reglas de producción
///
/// # Params
/// - axiom: string inicial
/// - rules: map de "char" → "string" de reemplazo
/// - iterations: número de iteraciones
///
/// # Returns
/// String final después de las iteraciones
pub fn rewrite(axiom: &str, rules: &[(String, String)], iterations: usize) -> String {
    let mut current = axiom.to_string();

    for _ in 0..iterations {
        let mut next = String::with_capacity(current.len() * 3);
        for c in current.chars() {
            let c_str = c.to_string();
            let mut found = false;
            for (pattern, replacement) in rules {
                if pattern == &c_str {
                    next.push_str(replacement);
                    found = true;
                    break;
                }
            }
            if !found {
                next.push(c);
            }
        }
        current = next;
    }

    current
}

/// Interpretar string L-System como segmentos de dibujo (turtle graphics)
///
/// Comandos:
/// - "F" o "0"-"9": avanzar y dibujar
/// - "f": avanzar sin dibujar
/// - "+": girar derecha (ángulo positivo)
/// - "-": girar izquierda (ángulo negativo)
/// - "[": guardar estado (push)
/// - "]": restaurar estado (pop)
///
/// # Returns
/// Array de segmentos [[x1, y1, x2, y2], ...] + información de ramas
pub fn interpret(
    axiom: &str,
    rules: &[(String, String)],
    iterations: usize,
    angle_deg: f64,
    segment_length: f64,
    start_x: f64,
    start_y: f64,
) -> Value {
    // Paso 1: Reescribir
    let rewritten = rewrite(axiom, rules, iterations);

    // Paso 2: Interpretar como turtle graphics
    let angle_rad = angle_deg.to_radians();
    let mut segments: Vec<[f64; 4]> = Vec::new();
    let mut branch_count = 0usize;
    let mut max_depth = 0usize;

    let mut x = start_x;
    let mut y = start_y;
    let mut angle = 90.0_f64.to_radians(); // Apuntando arriba
    let mut depth = 0usize;

    // Stack para guardar/restore estado: (x, y, angle, depth)
    let mut stack: Vec<(f64, f64, f64, usize)> = Vec::new();

    for c in rewritten.chars() {
        match c {
            'F' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // Avanzar y dibujar
                let new_x = x + segment_length * angle.cos();
                let new_y = y + segment_length * angle.sin();
                segments.push([x, y, new_x, new_y]);
                x = new_x;
                y = new_y;
            }
            'f' => {
                // Avanzar sin dibujar
                x += segment_length * angle.cos();
                y += segment_length * angle.sin();
            }
            '+' => {
                // Girar derecha
                angle -= angle_rad;
            }
            '-' => {
                // Girar izquierda
                angle += angle_rad;
            }
            '[' => {
                // Guardar estado
                stack.push((x, y, angle, depth));
                depth += 1;
                if depth > max_depth {
                    max_depth = depth;
                }
                branch_count += 1;
            }
            ']' => {
                // Restaurar estado
                if let Some((sx, sy, sa, sd)) = stack.pop() {
                    x = sx;
                    y = sy;
                    angle = sa;
                    depth = sd;
                }
            }
            _ => {
                // Ignorar caracteres desconocidos
            }
        }
    }

    json!({
        "segments": segments.iter().map(|s| json!([s[0], s[1], s[2], s[3]])).collect::<Vec<_>>(),
        "segment_count": segments.len(),
        "branch_count": branch_count,
        "max_depth": max_depth,
        "string_length": rewritten.len(),
        "iterations": iterations,
    })
}

// ============================================================================
// PRESETS CLÁSICOS
// ============================================================================

/// Árbol binario
/// Axioma: "0", Reglas: "0"→"1[+0][-0]", "1"→"11"
/// Ángulo: 25°, Segmento: 4.0
pub fn preset_binary_tree(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "0",
        &[("0".into(), "1[+0][-0]".into()), ("1".into(), "11".into())],
        iterations,
        25.0,
        segment_length,
        start_x,
        start_y,
    )
}

/// Árbol con 3 ramas (más denso)
/// Axioma: "X", Reglas: "X"→"F+[[X]-X]-F[-FX]+X", "F"→"FF"
/// Ángulo: 22.5°
pub fn preset_bush(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "X",
        &[
            ("X".into(), "F+[[X]-X]-F[-FX]+X".into()),
            ("F".into(), "FF".into()),
        ],
        iterations,
        22.5,
        segment_length,
        start_x,
        start_y,
    )
}

/// Helecho de Barnsley (L-System aproximado)
/// Axioma: "X", Reglas: "X"→"F+[[X]-X]-F[-FX]+X", "F"→"FF"
/// Ángulo: 25.7°, iteraciones: 6-7
pub fn preset_fern(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "X",
        &[
            ("X".into(), "F-[[X]+X]+F[+FX]-X".into()),
            ("F".into(), "FF".into()),
        ],
        iterations,
        25.7,
        segment_length,
        start_x,
        start_y,
    )
}

/// Coral ramificado
/// Axioma: "F", Reglas: "F"→"FF+[+F-F-F]-[-F+F+F]"
/// Ángulo: 22°
pub fn preset_coral(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "F",
        &[("F".into(), "FF+[+F-F-F]-[-F+F+F]".into())],
        iterations,
        22.0,
        segment_length,
        start_x,
        start_y,
    )
}

/// Planta de Koch (variante de copo de nieve)
/// Axioma: "F+F+F", Reglas: "F"→"F-F+F-F"
/// Ángulo: 90° (cuadrado)
pub fn preset_koch_plant(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "F+F+F",
        &[("F".into(), "F-F+F-F".into())],
        iterations,
        90.0,
        segment_length,
        start_x,
        start_y,
    )
}

/// Árbol fractal simple (el más clásico)
/// Axioma: "F", Reglas: "F"→"FF+[+F-F-F]-[-F+F+F]"
/// Ángulo: 22.5°
pub fn preset_fractal_tree(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "F",
        &[("F".into(), "F[+F]F[-F]F".into())],
        iterations,
        22.5,
        segment_length,
        start_x,
        start_y,
    )
}

/// Dragon curve (curva del dragón)
/// Axioma: "FX", Reglas: "X"→"X+YF+", "Y"→"-FX-Y"
/// Ángulo: 90°
pub fn preset_dragon_curve(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "FX",
        &[
            ("X".into(), "X+YF+".into()),
            ("Y".into(), "-FX-Y".into()),
        ],
        iterations,
        90.0,
        segment_length,
        start_x,
        start_y,
    )
}

/// Hilbert curve
/// Axioma: "A", Reglas: "A"→"-BF+AFA+FB-", "B"→"+AF-BFB-FA+"
/// Ángulo: 90°
pub fn preset_hilbert_curve(iterations: usize, segment_length: f64, start_x: f64, start_y: f64) -> Value {
    interpret(
        "A",
        &[
            ("A".into(), "-BF+AFA+FB-".into()),
            ("B".into(), "+AF-BFB-FA+".into()),
        ],
        iterations,
        90.0,
        segment_length,
        start_x,
        start_y,
    )
}

/// Obtener lista de presets disponibles
pub fn preset_names() -> Vec<&'static str> {
    vec![
        "binary_tree",
        "bush",
        "fern",
        "coral",
        "koch_plant",
        "fractal_tree",
        "dragon_curve",
        "hilbert_curve",
    ]
}

/// Ejecutar preset por nombre
pub fn run_preset(
    name: &str,
    iterations: usize,
    segment_length: f64,
    start_x: f64,
    start_y: f64,
) -> Option<Value> {
    match name {
        "binary_tree" => Some(preset_binary_tree(iterations, segment_length, start_x, start_y)),
        "bush" => Some(preset_bush(iterations, segment_length, start_x, start_y)),
        "fern" => Some(preset_fern(iterations, segment_length, start_x, start_y)),
        "coral" => Some(preset_coral(iterations, segment_length, start_x, start_y)),
        "koch_plant" => Some(preset_koch_plant(iterations, segment_length, start_x, start_y)),
        "fractal_tree" => Some(preset_fractal_tree(iterations, segment_length, start_x, start_y)),
        "dragon_curve" => Some(preset_dragon_curve(iterations, segment_length, start_x, start_y)),
        "hilbert_curve" => Some(preset_hilbert_curve(iterations, segment_length, start_x, start_y)),
        _ => None,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_simple() {
        // A → AB, iter 3: A → AB → ABAB → ABABABAB
        // Pero L-System aplica en paralelo: cada char del string actual se reemplaza
        // Iter 1: A → AB
        // Iter 2: A→AB, B→B = AB B = ABB  (B no tiene regla, se queda)
        // Iter 3: A→AB, B→B, B→B = ABBB
        let result = rewrite("A", &[("A".into(), "AB".into())], 3);
        assert_eq!(result, "ABBB");
    }

    #[test]
    fn test_rewrite_koch() {
        let result = rewrite("F", &[("F".into(), "F+F-F".into())], 2);
        // Iter 1: "F+F-F"
        // Iter 2: "F+F-F+F+F-F-F+F-F"
        assert_eq!(result, "F+F-F+F+F-F-F+F-F");
    }

    #[test]
    fn test_rewrite_no_match() {
        let result = rewrite("ABC", &[("A".into(), "XY".into())], 1);
        assert_eq!(result, "XYBC");
    }

    #[test]
    fn test_interpret_basic() {
        let result = interpret(
            "F",
            &[("F".into(), "FF".into())],
            2,
            90.0,
            10.0,
            0.0,
            0.0,
        );
        let obj = result.as_object().unwrap();
        let segments = obj["segments"].as_array().unwrap();
        // 4 segmentos (F → FF → FFFF)
        assert_eq!(segments.len(), 4);
    }

    #[test]
    fn test_interpret_branch() {
        // Árbol binario iteración 2
        let result = interpret(
            "0",
            &[
                ("0".into(), "1[+0][-0]".into()),
                ("1".into(), "11".into()),
            ],
            2,
            25.0,
            5.0,
            0.0,
            0.0,
        );
        let obj = result.as_object().unwrap();
        let segments = obj["segments"].as_array().unwrap();
        let branch_count = obj["branch_count"].as_i64().unwrap();
        // Debe tener segmentos y ramas
        assert!(segments.len() > 0);
        assert!(branch_count > 0);
    }

    #[test]
    fn test_interpret_bracket_stack() {
        // Probar push/pop con "F[+F]F[-F]F"
        let result = interpret("F[+F]F[-F]F", &[], 0, 25.0, 10.0, 0.0, 0.0);
        let obj = result.as_object().unwrap();
        let segments = obj["segments"].as_array().unwrap();
        // 5 F = 5 segmentos
        assert_eq!(segments.len(), 5);
        // Max depth debe ser 1
        assert_eq!(obj["max_depth"].as_i64().unwrap(), 1);
    }

    #[test]
    fn test_preset_binary_tree() {
        let result = preset_binary_tree(3, 5.0, 100.0, 200.0);
        let obj = result.as_object().unwrap();
        assert!(obj["segment_count"].as_i64().unwrap() > 0);
        assert!(obj["branch_count"].as_i64().unwrap() > 0);
    }

    #[test]
    fn test_preset_fern() {
        let result = preset_fern(4, 3.0, 200.0, 300.0);
        let obj = result.as_object().unwrap();
        assert!(obj["segment_count"].as_i64().unwrap() > 0);
    }

    #[test]
    fn test_preset_coral() {
        let result = preset_coral(3, 4.0, 150.0, 250.0);
        let obj = result.as_object().unwrap();
        assert!(obj["segment_count"].as_i64().unwrap() > 0);
    }

    #[test]
    fn test_preset_fractal_tree() {
        let result = preset_fractal_tree(3, 5.0, 200.0, 300.0);
        let obj = result.as_object().unwrap();
        assert!(obj["segment_count"].as_i64().unwrap() > 0);
        assert!(obj["branch_count"].as_i64().unwrap() > 0);
    }

    #[test]
    fn test_preset_dragon_curve() {
        let result = preset_dragon_curve(5, 3.0, 200.0, 200.0);
        let obj = result.as_object().unwrap();
        assert!(obj["segment_count"].as_i64().unwrap() > 0);
    }

    #[test]
    fn test_preset_hilbert_curve() {
        let result = preset_hilbert_curve(3, 5.0, 100.0, 100.0);
        let obj = result.as_object().unwrap();
        assert!(obj["segment_count"].as_i64().unwrap() > 0);
    }

    #[test]
    fn test_preset_names() {
        let names = preset_names();
        assert!(names.contains(&"binary_tree"));
        assert!(names.contains(&"fern"));
        assert!(names.contains(&"coral"));
        assert_eq!(names.len(), 8);
    }

    #[test]
    fn test_run_preset_unknown() {
        let result = run_preset("unknown_tree", 3, 5.0, 0.0, 0.0);
        assert!(result.is_none());
    }
}
