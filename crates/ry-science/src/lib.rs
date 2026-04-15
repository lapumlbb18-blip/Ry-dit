//! RyDit Science - Módulo de Ciencia para RyDit
//!
//! Proporciona funcionalidad de:
//! - Curvas Bezier (lineal, cuadrática, cúbica)
//! - Estadísticas (media, mediana, mínimo, máximo)
//! - Geometría (ilusiones ópticas: Penrose, Cubo imposible, Espiral)
//! - L-System (Lindenmayer: árboles, helechos, coral, curvas fractales)

pub mod geometry;
pub mod lsystem;
pub mod genetics;

use ry_core::{ModuleError, ModuleResult, RyditModule};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Módulo de Ciencia - Bezier y Estadísticas
pub struct ScienceModule;

impl RyditModule for ScienceModule {
    fn name(&self) -> &'static str {
        "science"
    }

    fn version(&self) -> &'static str {
        "0.7.3"
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("bezier::linear", "Curva Bezier lineal");
        cmds.insert("bezier::quadratic", "Curva Bezier cuadrática");
        cmds.insert("bezier::cubic", "Curva Bezier cúbica");
        cmds.insert("stats::mean", "Media aritmética");
        cmds.insert("stats::median", "Mediana");
        cmds.insert("stats::min", "Valor mínimo");
        cmds.insert("stats::max", "Valor máximo");
        cmds.insert("geometry::penrose", "Triángulo de Penrose");
        cmds.insert("geometry::impossible_cube", "Cubo imposible");
        cmds.insert("geometry::spiral", "Espiral óptica");
        cmds.insert("geometry::muller_lyer", "Ilusión Müller-Lyer");
        cmds.insert("geometry::ponzo", "Ilusión de Ponzo");
        // Fase 1: Fractales, autómatas, trig
        cmds.insert("fractal::mandelbrot", "Conjunto de Mandelbrot (puntos por fila)");
        cmds.insert("fractal::julia", "Conjunto de Julia (puntos por fila)");
        cmds.insert("fractal::koch", "Curva de Koch (puntos de copo de nieve)");
        cmds.insert("fractal::sierpinski", "Triángulo de Sierpinski (puntos)");
        cmds.insert("cellular::game_of_life", "Game of Life (Conway) — un paso");
        cmds.insert("cellular::rule1d", "Autómata 1D (Regla 30/90/110)");
        cmds.insert("trig::wave", "Onda sinusoidal/cuadrada/triangular (puntos)");
        cmds.insert("trig::lissajous", "Curva de Lissajous (puntos)");
        cmds.insert("trig::spiral", "Espiral trigonométrica (puntos)");
        cmds.insert("trig::harmonic", "Movimiento armónico simple (x, v, a en t)");
        // Fase 2: L-System (Lindenmayer)
        cmds.insert("lsystem::rewrite", "Reescribir string L-System (axioma + reglas)");
        cmds.insert("lsystem::interpret", "Interpretar L-System → segmentos de dibujo");
        cmds.insert("lsystem::preset", "Ejecutar preset (binary_tree, fern, coral, etc.)");
        cmds.insert("lsystem::presets", "Lista de presets disponibles");
        // Fase 2: Genética y Mutación
        cmds.insert("dna::new", "Generar nueva secuencia de ADN aleatoria");
        cmds.insert("dna::mutate", "Mutar secuencia de ADN (incluye factor radiación)");
        cmds.insert("dna::express", "Expresar ADN como fenotipo (atributos numéricos)");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "bezier::linear" => self.bezier_linear(params),
            "bezier::quadratic" => self.bezier_quadratic(params),
            "bezier::cubic" => self.bezier_cubic(params),
            "stats::mean" => self.stats_mean(params),
            "stats::median" => self.stats_median(params),
            "stats::min" => self.stats_min(params),
            "stats::max" => self.stats_max(params),
            "geometry::penrose" => self.geometry_penrose(params),
            "geometry::impossible_cube" => self.geometry_impossible_cube(params),
            "geometry::spiral" => self.geometry_spiral(params),
            "geometry::muller_lyer" => self.geometry_muller_lyer(params),
            "geometry::ponzo" => self.geometry_ponzo(params),
            // Fase 1
            "fractal::mandelbrot" => self.fractal_mandelbrot(params),
            "fractal::julia" => self.fractal_julia(params),
            "fractal::koch" => self.fractal_koch(params),
            "fractal::sierpinski" => self.fractal_sierpinski(params),
            "cellular::game_of_life" => self.cellular_game_of_life(params),
            "cellular::rule1d" => self.cellular_rule1d(params),
            "trig::wave" => self.trig_wave(params),
            "trig::lissajous" => self.trig_lissajous(params),
            "trig::spiral" => self.trig_spiral(params),
            "trig::harmonic" => self.trig_harmonic(params),
            // Fase 2: L-System
            "lsystem::rewrite" => self.lsystem_rewrite(params),
            "lsystem::interpret" => self.lsystem_interpret(params),
            "lsystem::preset" => self.lsystem_preset(params),
            "lsystem::presets" => self.lsystem_presets_list(params),
            // Fase 2: Genética
            "dna::new" => Ok(genetics::dna_new(params)),
            "dna::mutate" => Ok(genetics::dna_mutate(params)),
            "dna::express" => Ok(genetics::dna_express(params)),
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}", command),
            }),
        }
    }
}

impl ScienceModule {
    /// Curva Bezier lineal: P(t) = (1-t)*P0 + t*P1
    fn bezier_linear(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 5 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "bezier::linear requires 5 params: p0_x, p0_y, p1_x, p1_y, t".to_string(),
            });
        }

        let p0_x = arr[0].as_f64().unwrap_or(0.0);
        let p0_y = arr[1].as_f64().unwrap_or(0.0);
        let p1_x = arr[2].as_f64().unwrap_or(0.0);
        let p1_y = arr[3].as_f64().unwrap_or(0.0);
        let t = arr[4].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

        let x = (1.0 - t) * p0_x + t * p1_x;
        let y = (1.0 - t) * p0_y + t * p1_y;

        Ok(json!([x, y]))
    }

    /// Curva Bezier cuadrática: P(t) = (1-t)²*P0 + 2(1-t)t*P1 + t²*P2
    fn bezier_quadratic(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 7 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message:
                    "bezier::quadratic requires 7 params: p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, t"
                        .to_string(),
            });
        }

        let p0_x = arr[0].as_f64().unwrap_or(0.0);
        let p0_y = arr[1].as_f64().unwrap_or(0.0);
        let p1_x = arr[2].as_f64().unwrap_or(0.0);
        let p1_y = arr[3].as_f64().unwrap_or(0.0);
        let p2_x = arr[4].as_f64().unwrap_or(0.0);
        let p2_y = arr[5].as_f64().unwrap_or(0.0);
        let t = arr[6].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

        let mt = 1.0 - t;
        let x = mt * mt * p0_x + 2.0 * mt * t * p1_x + t * t * p2_x;
        let y = mt * mt * p0_y + 2.0 * mt * t * p1_y + t * t * p2_y;

        Ok(json!([x, y]))
    }

    /// Curva Bezier cúbica: P(t) = (1-t)³*P0 + 3(1-t)²t*P1 + 3(1-t)t²*P2 + t³*P3
    fn bezier_cubic(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 9 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "bezier::cubic requires 9 params: p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t".to_string(),
            });
        }

        let p0_x = arr[0].as_f64().unwrap_or(0.0);
        let p0_y = arr[1].as_f64().unwrap_or(0.0);
        let p1_x = arr[2].as_f64().unwrap_or(0.0);
        let p1_y = arr[3].as_f64().unwrap_or(0.0);
        let p2_x = arr[4].as_f64().unwrap_or(0.0);
        let p2_y = arr[5].as_f64().unwrap_or(0.0);
        let p3_x = arr[6].as_f64().unwrap_or(0.0);
        let p3_y = arr[7].as_f64().unwrap_or(0.0);
        let t = arr[8].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);

        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let t2 = t * t;

        let x = mt2 * mt * p0_x + 3.0 * mt2 * t * p1_x + 3.0 * mt * t2 * p2_x + t2 * t * p3_x;
        let y = mt2 * mt * p0_y + 3.0 * mt2 * t * p1_y + 3.0 * mt * t2 * p2_y + t2 * t * p3_y;

        Ok(json!([x, y]))
    }

    /// Media aritmética: sum / n
    fn stats_mean(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.is_empty() {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "Empty array".to_string(),
            });
        }

        let sum: f64 = arr.iter().filter_map(|v| v.as_f64()).sum();
        Ok(json!(sum / arr.len() as f64))
    }

    /// Mediana: valor central de array ordenado
    fn stats_median(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        let mut nums: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();

        if nums.is_empty() {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "Empty array or no numbers".to_string(),
            });
        }

        nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = nums.len() / 2;

        let median = if nums.len().is_multiple_of(2) {
            (nums[mid - 1] + nums[mid]) / 2.0
        } else {
            nums[mid]
        };

        Ok(json!(median))
    }

    /// Valor mínimo de un array
    fn stats_min(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        let mut min_val = f64::MAX;
        let mut found = false;

        for v in arr {
            if let Some(n) = v.as_f64() {
                if n < min_val {
                    min_val = n;
                }
                found = true;
            }
        }

        if found {
            Ok(json!(min_val))
        } else {
            Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "No numbers in array".to_string(),
            })
        }
    }

    /// Valor máximo de un array
    fn stats_max(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        let mut max_val = f64::MIN;
        let mut found = false;

        for v in arr {
            if let Some(n) = v.as_f64() {
                if n > max_val {
                    max_val = n;
                }
                found = true;
            }
        }

        if found {
            Ok(json!(max_val))
        } else {
            Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "No numbers in array".to_string(),
            })
        }
    }

    // ================================================================
    // Funciones de Geometría - Ilusiones Ópticas
    // ================================================================

    /// Triángulo de Penrose (tribar imposible)
    fn geometry_penrose(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::penrose requires 3 params: center_x, center_y, size"
                    .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let size = arr[2].as_f64().unwrap_or(100.0);

        Ok(geometry::penrose(center_x, center_y, size))
    }

    /// Cubo imposible (Necker cube)
    fn geometry_impossible_cube(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::impossible_cube requires 3 params: center_x, center_y, size"
                    .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let size = arr[2].as_f64().unwrap_or(100.0);

        Ok(geometry::impossible_cube(center_x, center_y, size))
    }

    /// Espiral óptica (Arquímedes)
    fn geometry_spiral(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 5 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message:
                    "geometry::spiral requires 5 params: center_x, center_y, turns, radius, points"
                        .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let turns = arr[2].as_i64().unwrap_or(3) as i32;
        let radius = arr[3].as_f64().unwrap_or(100.0);
        let points = arr[4].as_i64().unwrap_or(20) as i32;

        Ok(geometry::spiral(center_x, center_y, turns, radius, points))
    }

    /// Ilusión de Müller-Lyer (flechas)
    fn geometry_muller_lyer(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::muller_lyer requires 3 params: center_x, center_y, length"
                    .to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let length = arr[2].as_f64().unwrap_or(200.0);

        Ok(geometry::muller_lyer(center_x, center_y, length))
    }

    /// Ilusión de Ponzo (perspectiva)
    fn geometry_ponzo(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 5 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "geometry::ponzo requires 5 params: center_x, center_y, height, width_top, width_bottom".to_string(),
            });
        }

        let center_x = arr[0].as_f64().unwrap_or(400.0);
        let center_y = arr[1].as_f64().unwrap_or(300.0);
        let height = arr[2].as_f64().unwrap_or(300.0);
        let width_top = arr[3].as_f64().unwrap_or(100.0);
        let width_bottom = arr[4].as_f64().unwrap_or(300.0);

        Ok(geometry::ponzo(
            center_x,
            center_y,
            height,
            width_top,
            width_bottom,
        ))
    }

    // ========================================================================
    // FRACTALES — Mandelbrot, Julia, Koch, Sierpinski
    // ========================================================================

    /// Conjunto de Mandelbrot: z_{n+1} = z_n² + c
    ///
    /// # Params
    /// - width, height: resolución de la imagen
    /// - max_iter: iteraciones máximas (default: 100)
    /// - center_x, center_y: centro del plano complejo (default: -0.5, 0)
    /// - zoom: zoom (default: 1.0)
    ///
    /// # Returns
    /// Array de filas, cada fila es array de [escape_iter, real, imag] por píxel
    fn fractal_mandelbrot(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "mandelbrot: width, height, max_iter, cx, cy, zoom".to_string(),
        })?;

        let width = arr[0].as_i64().unwrap_or(200) as usize;
        let height = arr[1].as_i64().unwrap_or(150) as usize;
        let max_iter = arr[2].as_i64().unwrap_or(100) as u32;
        let cx = arr[3].as_f64().unwrap_or(-0.5);
        let cy = arr[4].as_f64().unwrap_or(0.0);
        let zoom = arr[5].as_f64().unwrap_or(1.0);

        let x_range = 3.5 / zoom;
        let y_range = 2.0 / zoom;
        let x_min = cx - x_range / 2.0;
        let y_min = cy - y_range / 2.0;

        let mut result = Vec::new();
        for py in 0..height {
            let mut row = Vec::new();
            for px in 0..width {
                let c_re = x_min + (px as f64) / (width as f64) * x_range;
                let c_im = y_min + (py as f64) / (height as f64) * y_range;

                let mut z_re = 0.0;
                let mut z_im = 0.0;
                let mut iter = 0u32;

                while z_re * z_re + z_im * z_im < 4.0 && iter < max_iter {
                    let z_re_new = z_re * z_re - z_im * z_im + c_re;
                    z_im = 2.0 * z_re * z_im + c_im;
                    z_re = z_re_new;
                    iter += 1;
                }

                row.push(json!([iter as i64, px as i64, py as i64]));
            }
            result.push(json!(row));
        }

        Ok(json!(result))
    }

    /// Conjunto de Julia: z_{n+1} = z_n² + c (c fijo, z variable)
    ///
    /// # Params
    /// - width, height: resolución
    /// - max_iter: iteraciones (default: 100)
    /// - c_re, c_im: constante c del Julia (default: -0.7, 0.27015)
    fn fractal_julia(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "julia: width, height, max_iter, c_re, c_im".to_string(),
        })?;

        let width = arr[0].as_i64().unwrap_or(200) as usize;
        let height = arr[1].as_i64().unwrap_or(150) as usize;
        let max_iter = arr[2].as_i64().unwrap_or(100) as u32;
        let c_re = arr[3].as_f64().unwrap_or(-0.7);
        let c_im = arr[4].as_f64().unwrap_or(0.27015);

        let mut result = Vec::new();
        for py in 0..height {
            let mut row = Vec::new();
            for px in 0..width {
                let mut z_re = 3.0 * (px as f64) / (width as f64) - 1.5;
                let mut z_im = 2.0 * (py as f64) / (height as f64) - 1.0;
                let mut iter = 0u32;

                while z_re * z_re + z_im * z_im < 4.0 && iter < max_iter {
                    let z_re_new = z_re * z_re - z_im * z_im + c_re;
                    z_im = 2.0 * z_re * z_im + c_im;
                    z_re = z_re_new;
                    iter += 1;
                }

                row.push(json!([iter as i64, px as i64, py as i64]));
            }
            result.push(json!(row));
        }

        Ok(json!(result))
    }

    /// Curva de Koch (copo de nieve)
    ///
    /// # Params
    /// - iterations: nivel de recursión (default: 4)
    ///
    /// # Returns
    /// Array de puntos [x, y] del copo de nieve
    fn fractal_koch(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "koch: iterations".to_string(),
        })?;

        let iterations = arr[0].as_i64().unwrap_or(4) as usize;

        fn koch_segment(p1: (f64, f64), p2: (f64, f64), depth: usize) -> Vec<(f64, f64)> {
            if depth == 0 {
                return vec![p1, p2];
            }
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            let a = (p1.0 + dx / 3.0, p1.1 + dy / 3.0);
            let b = (p1.0 + 2.0 * dx / 3.0, p1.1 + 2.0 * dy / 3.0);
            let tip_x = (p1.0 + p2.0) / 2.0 + (p1.1 - p2.1) * 3_f64.sqrt() / 6.0;
            let tip_y = (p1.1 + p2.1) / 2.0 + (p2.0 - p1.0) * 3_f64.sqrt() / 6.0;
            let tip = (tip_x, tip_y);

            let mut result = koch_segment(p1, a, depth - 1);
            result.pop();
            let mut r2 = koch_segment(a, tip, depth - 1);
            r2.pop();
            result.extend(r2);
            let mut r3 = koch_segment(tip, b, depth - 1);
            r3.pop();
            result.extend(r3);
            let mut r4 = koch_segment(b, p2, depth - 1);
            result.extend(r4);
            result
        }

        let p1 = (0.0, 0.0);
        let p2 = (1.0, 0.0);
        let angle = std::f64::consts::PI / 3.0;
        let p3 = (0.5, angle.sin());

        let mut points = koch_segment(p1, p2, iterations);
        points.pop();
        points.extend(koch_segment(p2, p3, iterations));
        points.pop();
        points.extend(koch_segment(p3, p1, iterations));

        let result: Vec<Value> = points.into_iter().map(|(x, y)| json!([x, y])).collect();
        Ok(json!(result))
    }

    /// Triángulo de Sierpinski
    ///
    /// # Params
    /// - iterations: profundidad (default: 6)
    ///
    /// # Returns
    /// Array de triángulos [[x1,y1],[x2,y2],[x3,y3]]
    fn fractal_sierpinski(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "sierpinski: iterations".to_string(),
        })?;

        let iterations = arr[0].as_i64().unwrap_or(6) as usize;

        fn sierpinski(p1: (f64,f64), p2: (f64,f64), p3: (f64,f64), depth: usize) -> Vec<Value> {
            if depth == 0 {
                return vec![json!([
                    [p1.0, p1.1], [p2.0, p2.1], [p3.0, p3.1]
                ])];
            }
            let m1 = ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0);
            let m2 = ((p2.0 + p3.0) / 2.0, (p2.1 + p3.1) / 2.0);
            let m3 = ((p1.0 + p3.0) / 2.0, (p1.1 + p3.1) / 2.0);

            let mut result = sierpinski(p1, m1, m3, depth - 1);
            result.extend(sierpinski(m1, p2, m2, depth - 1));
            result.extend(sierpinski(m3, m2, p3, depth - 1));
            result
        }

        let h = 3_f64.sqrt() / 2.0;
        let triangles = sierpinski((0.0, 0.0), (1.0, 0.0), (0.5, h), iterations);
        Ok(json!(triangles))
    }

    // ========================================================================
    // AUTÓMATAS CELULARES
    // ========================================================================

    /// Game of Life (Conway) — un paso de simulación
    ///
    /// # Params
    /// - grid: array 2D de 0/1
    ///
    /// # Returns
    /// Nuevo grid 2D después de un paso
    fn cellular_game_of_life(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "game_of_life: grid".to_string(),
        })?;

        let grid: Vec<Vec<u8>> = arr.iter().map(|row| {
            let r = row.as_array().expect("grid row must be array");
            r.iter().map(|v| v.as_u64().unwrap_or(0) as u8).collect()
        }).collect();

        if grid.is_empty() { return Ok(json!([])); }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut new_grid = vec![vec![0u8; cols]; rows];

        for r in 0..rows {
            for c in 0..cols {
                let mut neighbors = 0;
                for dr in -1..=1i64 {
                    for dc in -1..=1i64 {
                        if dr == 0 && dc == 0 { continue; }
                        let nr = ((r as i64 + dr) % rows as i64 + rows as i64) % rows as i64;
                        let nc = ((c as i64 + dc) % cols as i64 + cols as i64) % cols as i64;
                        if grid[nr as usize][nc as usize] == 1 { neighbors += 1; }
                    }
                }

                if grid[r][c] == 1 {
                    new_grid[r][c] = if neighbors == 2 || neighbors == 3 { 1 } else { 0 };
                } else {
                    new_grid[r][c] = if neighbors == 3 { 1 } else { 0 };
                }
            }
        }

        let result: Vec<Value> = new_grid.iter().map(|row| {
            json!(row.iter().map(|&x| x as i64).collect::<Vec<_>>())
        }).collect();
        Ok(json!(result))
    }

    /// Autómata 1D (Regla 30/90/110/etc.)
    ///
    /// # Params
    /// - row: fila inicial (array de 0/1)
    /// - rule: número de regla (0-255, default: 30)
    /// - generations: número de generaciones (default: 50)
    ///
    /// # Returns
    /// Array de filas (generaciones)
    fn cellular_rule1d(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "rule1d: row, rule, generations".to_string(),
        })?;

        let row: Vec<u8> = arr[0].as_array().map(|r|
            r.iter().filter_map(|v| v.as_u64().map(|x| x as u8)).collect()
        ).unwrap_or_default();

        let rule = arr[1].as_u64().unwrap_or(30) as u8;
        let generations = arr[2].as_i64().unwrap_or(50) as usize;

        let mut result = vec![json!(row.clone())];
        let mut current = row;

        for _ in 0..generations {
            let mut next = vec![0u8; current.len()];
            for i in 0..current.len() {
                let left = current[(i + current.len() - 1) % current.len()];
                let center = current[i];
                let right = current[(i + 1) % current.len()];
                let idx = (left << 2) | (center << 1) | right;
                next[i] = (rule >> idx) & 1;
            }
            result.push(json!(next.iter().map(|&x| x as i64).collect::<Vec<_>>()));
            current = next;
        }

        Ok(json!(result))
    }

    // ========================================================================
    // TRIGONOMETRÍA ANIMADA
    // ========================================================================

    /// Onda sinusoidal/cuadrada/triangular
    ///
    /// # Params
    /// - num_points: puntos a generar (default: 100)
    /// - wave_type: "sine", "square", "triangle", "sawtooth" (default: "sine")
    /// - frequency: frecuencia (default: 1.0)
    /// - amplitude: amplitud (default: 1.0)
    /// - phase: fase inicial (default: 0)
    ///
    /// # Returns
    /// Array de [x, y] puntos de la onda
    fn trig_wave(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "wave: num_points, wave_type, frequency, amplitude, phase".to_string(),
        })?;

        let num_points = arr[0].as_i64().unwrap_or(100) as usize;
        let wave_type = arr[1].as_str().unwrap_or("sine");
        let frequency = arr[2].as_f64().unwrap_or(1.0);
        let amplitude = arr[3].as_f64().unwrap_or(1.0);
        let phase = arr[4].as_f64().unwrap_or(0.0);

        let result: Vec<Value> = (0..num_points).map(|i| {
            let t = (i as f64) / (num_points as f64) * 2.0 * std::f64::consts::PI * frequency + phase;
            let y = match wave_type {
                "square" => if t.sin() >= 0.0 { 1.0 } else { -1.0 },
                "triangle" => {
                    let norm = (t / std::f64::consts::PI).fract();
                    if norm < 0.5 { 4.0 * norm - 1.0 } else { 3.0 - 4.0 * norm }
                },
                "sawtooth" => {
                    let norm = (t / std::f64::consts::PI).fract();
                    2.0 * norm - 1.0
                },
                _ => t.sin(), // sine
            };
            json!([t, y * amplitude])
        }).collect();

        Ok(json!(result))
    }

    /// Curva de Lissajous: x = A·sin(a·t + δ), y = B·sin(b·t)
    ///
    /// # Params
    /// - num_points: puntos (default: 200)
    /// - a, b: frecuencias (default: 3, 2)
    /// - delta: desfase (default: π/2)
    fn trig_lissajous(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "lissajous: num_points, a, b, delta".to_string(),
        })?;

        let num_points = arr[0].as_i64().unwrap_or(200) as usize;
        let a = arr[1].as_f64().unwrap_or(3.0);
        let b = arr[2].as_f64().unwrap_or(2.0);
        let delta = arr[3].as_f64().unwrap_or(std::f64::consts::PI / 2.0);

        let result: Vec<Value> = (0..num_points).map(|i| {
            let t = (i as f64) / (num_points as f64) * 2.0 * std::f64::consts::PI;
            let x = (a * t + delta).sin();
            let y = (b * t).sin();
            json!([x, y])
        }).collect();

        Ok(json!(result))
    }

    /// Espiral trigonométrica: x = r·t·cos(t), y = r·t·sin(t)
    ///
    /// # Params
    /// - num_points: puntos (default: 200)
    /// - growth: tasa de crecimiento (default: 0.1)
    /// - turns: vueltas (default: 5)
    fn trig_spiral(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "spiral: num_points, growth, turns".to_string(),
        })?;

        let num_points = arr[0].as_i64().unwrap_or(200) as usize;
        let growth = arr[1].as_f64().unwrap_or(0.1);
        let turns = arr[2].as_f64().unwrap_or(5.0);

        let result: Vec<Value> = (0..num_points).map(|i| {
            let t = (i as f64) / (num_points as f64) * turns * 2.0 * std::f64::consts::PI;
            let r = growth * t;
            let x = r * t.cos();
            let y = r * t.sin();
            json!([x, y])
        }).collect();

        Ok(json!(result))
    }

    /// Movimiento armónico simple: x = A·cos(ωt), v = -Aω·sin(ωt), a = -Aω²·cos(ωt)
    ///
    /// # Params
    /// - amplitude: amplitud (default: 1.0)
    /// - frequency: frecuencia angular ω (default: 1.0)
    /// - time: tiempo t (default: 0)
    ///
    /// # Returns
    /// [position, velocity, acceleration]
    fn trig_harmonic(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "harmonic: amplitude, frequency, time".to_string(),
        })?;

        let a = arr[0].as_f64().unwrap_or(1.0);
        let omega = arr[1].as_f64().unwrap_or(1.0);
        let t = arr[2].as_f64().unwrap_or(0.0);

        let x = a * (omega * t).cos();
        let v = -a * omega * (omega * t).sin();
        let accel = -a * omega * omega * (omega * t).cos();

        Ok(json!([x, v, accel]))
    }

    // ========================================================================
    // L-SYSTEM — Lindenmayer System (árboles, helechos, coral, fractales)
    // ========================================================================

    /// Reescribir string L-System
    ///
    /// # Params
    /// - axiom: string inicial
    /// - rules: array de [patrón, reemplazo] ej: [["F", "FF"]]
    /// - iterations: número de iteraciones
    ///
    /// # Returns
    /// { "string": "resultante", "length": longitud }
    fn lsystem_rewrite(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "lsystem::rewrite: axiom, rules, iterations".to_string(),
        })?;

        if arr.len() < 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "lsystem::rewrite requiere 3 params".to_string(),
            });
        }

        let axiom = arr[0].as_str().unwrap_or("F");
        let rules_arr = arr[1].as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "rules debe ser array de [[patrón, reemplazo]]".to_string(),
        })?;
        let iterations = arr[2].as_u64().unwrap_or(3) as usize;

        let rules: Vec<(String, String)> = rules_arr.iter().filter_map(|r| {
            let a = r.as_array()?;
            if a.len() < 2 { return None; }
            Some((a[0].as_str()?.into(), a[1].as_str()?.into()))
        }).collect();

        let result = lsystem::rewrite(axiom, &rules, iterations);
        Ok(json!({ "string": result, "length": result.len() }))
    }

    /// Interpretar L-System → segmentos de dibujo
    ///
    /// # Params
    /// - axiom, rules, iterations: como rewrite
    /// - angle_deg: ángulo en grados
    /// - segment_length: longitud de segmento
    /// - start_x, start_y: posición inicial
    ///
    /// # Returns
    /// { segments, segment_count, branch_count, max_depth, string_length }
    fn lsystem_interpret(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "lsystem::interpret: axiom, rules, iterations, angle, seg_len, sx, sy".to_string(),
        })?;

        if arr.len() < 7 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "lsystem::interpret requiere 7 params".to_string(),
            });
        }

        let axiom = arr[0].as_str().unwrap_or("F");
        let rules_arr = arr[1].as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "rules debe ser array".to_string(),
        })?;
        let iterations = arr[2].as_u64().unwrap_or(3) as usize;
        let angle = arr[3].as_f64().unwrap_or(25.0);
        let seg_len = arr[4].as_f64().unwrap_or(5.0);
        let sx = arr[5].as_f64().unwrap_or(200.0);
        let sy = arr[6].as_f64().unwrap_or(400.0);

        let rules: Vec<(String, String)> = rules_arr.iter().filter_map(|r| {
            let a = r.as_array()?;
            if a.len() < 2 { return None; }
            Some((a[0].as_str()?.into(), a[1].as_str()?.into()))
        }).collect();

        Ok(lsystem::interpret(axiom, &rules, iterations, angle, seg_len, sx, sy))
    }

    /// Ejecutar preset de L-System
    ///
    /// # Params
    /// - name: nombre del preset (binary_tree, fern, coral, etc.)
    /// - iterations: nivel de detalle (default: 4)
    /// - segment_length: longitud de segmento (default: 5.0)
    /// - start_x, start_y: posición inicial (default: 200, 400)
    ///
    /// # Returns
    /// { segments, segment_count, branch_count, max_depth, string_length }
    fn lsystem_preset(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "lsystem::preset: name, iterations, seg_len, sx, sy".to_string(),
        })?;

        if arr.len() < 1 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "lsystem::preset requiere al menos name".to_string(),
            });
        }

        let name = arr[0].as_str().unwrap_or("binary_tree");
        let iterations = arr[1].as_u64().unwrap_or(4) as usize;
        let seg_len = arr[2].as_f64().unwrap_or(5.0);
        let sx = arr[3].as_f64().unwrap_or(200.0);
        let sy = arr[4].as_f64().unwrap_or(400.0);

        match lsystem::run_preset(name, iterations, seg_len, sx, sy) {
            Some(result) => Ok(json!({ "preset": name, "result": result })),
            None => Err(ModuleError {
                code: "UNKNOWN_PRESET".to_string(),
                message: format!("Preset desconocido: {}. Usa lsystem::presets para ver la lista", name),
            }),
        }
    }

    /// Lista de presets disponibles
    fn lsystem_presets_list(&self, _params: Value) -> ModuleResult {
        let names = lsystem::preset_names();
        Ok(json!({ "presets": names, "count": names.len() }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_science_module_name() {
        let module = ScienceModule;
        assert_eq!(module.name(), "science");
        assert_eq!(module.version(), "0.7.3");
    }

    #[test]
    fn test_science_register() {
        let module = ScienceModule;
        let cmds = module.register();

        assert!(cmds.contains_key("bezier::linear"));
        assert!(cmds.contains_key("bezier::cubic"));
        assert!(cmds.contains_key("stats::mean"));
        assert!(cmds.contains_key("stats::median"));
        assert!(cmds.contains_key("geometry::penrose"));
        assert!(cmds.contains_key("geometry::impossible_cube"));
        assert!(cmds.contains_key("geometry::spiral"));
    }

    #[test]
    fn test_bezier_linear() {
        let module = ScienceModule;
        let params = json!([0.0, 0.0, 100.0, 100.0, 0.5]);
        let result = module.execute("bezier::linear", params).unwrap();

        assert_eq!(result, json!([50.0, 50.0]));
    }

    #[test]
    fn test_bezier_cubic() {
        let module = ScienceModule;
        // p0=(0,0), p1=(30,100), p2=(70,100), p3=(100,0), t=0.5
        let params = json!([0.0, 0.0, 30.0, 100.0, 70.0, 100.0, 100.0, 0.0, 0.5]);
        let result = module.execute("bezier::cubic", params).unwrap();

        assert_eq!(result, json!([50.0, 75.0]));
    }

    #[test]
    fn test_stats_mean() {
        let module = ScienceModule;
        let params = json!([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = module.execute("stats::mean", params).unwrap();

        assert_eq!(result, json!(3.0));
    }

    #[test]
    fn test_stats_median_odd() {
        let module = ScienceModule;
        let params = json!([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = module.execute("stats::median", params).unwrap();

        assert_eq!(result, json!(3.0));
    }

    #[test]
    fn test_stats_median_even() {
        let module = ScienceModule;
        let params = json!([1.0, 2.0, 3.0, 4.0]);
        let result = module.execute("stats::median", params).unwrap();

        assert_eq!(result, json!(2.5));
    }

    #[test]
    fn test_stats_min_max() {
        let module = ScienceModule;
        let params = json!([3.0, 1.0, 4.0, 1.0, 5.0]);

        let min_result = module.execute("stats::min", params.clone()).unwrap();
        assert_eq!(min_result, json!(1.0));

        let max_result = module.execute("stats::max", params).unwrap();
        assert_eq!(max_result, json!(5.0));
    }

    #[test]
    fn test_unknown_command() {
        let module = ScienceModule;
        let result = module.execute("unknown", json!([]));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "UNKNOWN_COMMAND");
    }

    // Tests de Geometría
    #[test]
    fn test_geometry_penrose() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 100.0]);
        let result = module.execute("geometry::penrose", params).unwrap();

        let lines = result.as_array().unwrap();
        assert!(!lines.is_empty());
        assert!(lines.len() >= 10);
    }

    #[test]
    fn test_geometry_impossible_cube() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 100.0]);
        let result = module.execute("geometry::impossible_cube", params).unwrap();

        let lines = result.as_array().unwrap();
        assert!(!lines.is_empty());
        assert!(lines.len() >= 12);
    }

    #[test]
    fn test_geometry_spiral() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 3, 100.0, 20]);
        let result = module.execute("geometry::spiral", params).unwrap();

        let points = result.as_array().unwrap();
        assert_eq!(points.len(), 60); // 3 turns * 20 points
    }

    #[test]
    fn test_geometry_muller_lyer() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 200.0]);
        let result = module.execute("geometry::muller_lyer", params).unwrap();

        let lines = result.as_array().unwrap();
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn test_geometry_ponzo() {
        let module = ScienceModule;
        let params = json!([400.0, 300.0, 300.0, 100.0, 300.0]);
        let result = module.execute("geometry::ponzo", params).unwrap();

        let lines = result.as_array().unwrap();
        assert_eq!(lines.len(), 6); // 2 rieles + 4 horizontales
    }

    // ========================================================================
    // TESTS — FASE 1: Fractales, autómatas, trig
    // ========================================================================

    #[test]
    fn test_fractal_mandelbrot() {
        let module = ScienceModule;
        let params = json!([50, 40, 50, -0.5, 0.0, 1.0]);
        let result = module.execute("fractal::mandelbrot", params).unwrap();
        let rows = result.as_array().unwrap();
        assert_eq!(rows.len(), 40);
        let first_row = rows[0].as_array().unwrap();
        assert_eq!(first_row.len(), 50);
    }

    #[test]
    fn test_fractal_julia() {
        let module = ScienceModule;
        let params = json!([50, 40, 50, -0.7, 0.27015]);
        let result = module.execute("fractal::julia", params).unwrap();
        let rows = result.as_array().unwrap();
        assert_eq!(rows.len(), 40);
    }

    #[test]
    fn test_fractal_koch() {
        let module = ScienceModule;
        let params = json!([3]); // 3 iterations
        let result = module.execute("fractal::koch", params).unwrap();
        let points = result.as_array().unwrap();
        // Koch snowflake at depth 3: 3 * 4^3 = 192 segments + 1
        assert!(!points.is_empty());
        assert!(points.len() > 50);
    }

    #[test]
    fn test_fractal_sierpinski() {
        let module = ScienceModule;
        let params = json!([3]);
        let result = module.execute("fractal::sierpinski", params).unwrap();
        let triangles = result.as_array().unwrap();
        // 3^3 = 27 triangles
        assert_eq!(triangles.len(), 27);
    }

    #[test]
    fn test_cellular_game_of_life() {
        let module = ScienceModule;
        // Simple: 3x3 block that stays stable
        let grid = json!([
            [0i64, 1i64, 0i64],
            [0i64, 1i64, 0i64],
            [0i64, 1i64, 0i64]
        ]);
        let result = module.execute("cellular::game_of_life", grid).unwrap();
        let new_grid = result.as_array().unwrap();
        assert_eq!(new_grid.len(), 3, "grid should have 3 rows");
        for row in new_grid {
            assert_eq!(row.as_array().unwrap().len(), 3, "each row should have 3 cols");
        }
    }

    #[test]
    fn test_cellular_rule30() {
        let module = ScienceModule;
        let row = json!([0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]);
        let params = json!([row, 30, 10]);
        let result = module.execute("cellular::rule1d", params).unwrap();
        let generations = result.as_array().unwrap();
        assert_eq!(generations.len(), 11); // initial + 10
    }

    #[test]
    fn test_cellular_rule90_sierpinski() {
        let module = ScienceModule;
        let row = json!([0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0]);
        let params = json!([row, 90, 5]);
        let result = module.execute("cellular::rule1d", params).unwrap();
        let generations = result.as_array().unwrap();
        assert_eq!(generations.len(), 6);
    }

    #[test]
    fn test_trig_wave_sine() {
        let module = ScienceModule;
        let params = json!([10, "sine", 1.0, 1.0, 0.0]);
        let result = module.execute("trig::wave", params).unwrap();
        let points = result.as_array().unwrap();
        assert_eq!(points.len(), 10);
    }

    #[test]
    fn test_trig_wave_square() {
        let module = ScienceModule;
        let params = json!([10, "square", 1.0, 1.0, 0.0]);
        let result = module.execute("trig::wave", params).unwrap();
        let points = result.as_array().unwrap();
        // Square wave: values should be either 1.0 or -1.0
        for p in points {
            let y = p.as_array().unwrap()[1].as_f64().unwrap();
            assert!((y - 1.0).abs() < 0.01 || (y + 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_trig_lissajous() {
        let module = ScienceModule;
        let params = json!([100, 3.0, 2.0, 1.5708]);
        let result = module.execute("trig::lissajous", params).unwrap();
        let points = result.as_array().unwrap();
        assert_eq!(points.len(), 100);
    }

    #[test]
    fn test_trig_spiral() {
        let module = ScienceModule;
        let params = json!([100, 0.1, 5.0]);
        let result = module.execute("trig::spiral", params).unwrap();
        let points = result.as_array().unwrap();
        assert_eq!(points.len(), 100);
    }

    #[test]
    fn test_trig_harmonic() {
        let module = ScienceModule;
        let params = json!([1.0, 1.0, 0.0]);
        let result = module.execute("trig::harmonic", params).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        // At t=0: x = A*cos(0) = 1, v = -A*ω*sin(0) = 0, a = -A*ω²*cos(0) = -1
        assert!((arr[0].as_f64().unwrap() - 1.0).abs() < 0.01);
        assert!(arr[1].as_f64().unwrap().abs() < 0.01);
        assert!((arr[2].as_f64().unwrap() + 1.0).abs() < 0.01);
    }

    #[test]
    fn test_science_register_complete() {
        let module = ScienceModule;
        let cmds = module.register();
        assert_eq!(cmds.len(), 29); // 26 originales + 3 ADN

        assert!(cmds.contains_key("fractal::mandelbrot"));
        assert!(cmds.contains_key("fractal::julia"));
        assert!(cmds.contains_key("fractal::koch"));
        assert!(cmds.contains_key("fractal::sierpinski"));
        assert!(cmds.contains_key("cellular::game_of_life"));
        assert!(cmds.contains_key("cellular::rule1d"));
        assert!(cmds.contains_key("trig::wave"));
        assert!(cmds.contains_key("trig::lissajous"));
        assert!(cmds.contains_key("trig::spiral"));
        assert!(cmds.contains_key("trig::harmonic"));
        // L-System
        assert!(cmds.contains_key("lsystem::rewrite"));
        assert!(cmds.contains_key("lsystem::interpret"));
        assert!(cmds.contains_key("lsystem::preset"));
        assert!(cmds.contains_key("lsystem::presets"));
        // Genética
        assert!(cmds.contains_key("dna::new"));
        assert!(cmds.contains_key("dna::mutate"));
        assert!(cmds.contains_key("dna::express"));
    }

    #[test]
    fn test_dna_mutation() {
        let module = ScienceModule;
        let params = json!({
            "sequence": "AAAAAAAAAA",
            "rate": 1.0, // 100% mutación
            "radiation": 0.0
        });
        let result = module.execute("dna::mutate", params).unwrap();
        let seq = result["sequence"].as_str().unwrap();
        assert_ne!(seq, "AAAAAAAAAA");
        assert_eq!(seq.len(), 10);
    }
}
