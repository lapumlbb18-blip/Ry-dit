//! RyDit Physics - Módulo de Física para RyDit
//!
//! Proporciona funcionalidad de:
//! - Proyectil 2D (trayectoria, altura máxima, alcance)
//! - Gravedad N-cuerpos (2 cuerpos)

use ry_core::{ModuleError, ModuleResult, RyditModule};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Módulo de Física - Proyectil y Gravedad
pub struct PhysicsModule;

impl RyditModule for PhysicsModule {
    fn name(&self) -> &'static str {
        "physics"
    }

    fn version(&self) -> &'static str {
        "0.7.3"
    }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("projectile", "Simulación de proyectil");
        cmds.insert("nbody_2", "Simulación N-cuerpos (2 cuerpos)");
        cmds.insert("nbody_simulate", "Simulación N-cuerpos (múltiples cuerpos)");
        cmds.insert("elastic_collision_2", "Colisión elástica 2D (2 cuerpos)");
        cmds.insert("elastic_collision_multi", "Colisiones elásticas múltiples");
        cmds.insert("flocking_step", "Paso de flocking (Reynolds: cohesión+separación+alineación)");
        cmds.insert("drag_force", "Fuerza de arrastre/fricción del aire");
        cmds.insert("wind_force", "Fuerza de viento con turbulencia");
        cmds.insert("pendulum_period", "Período de péndulo simple");
        cmds.insert("spring_force", "Fuerza de resorte (Hooke)");
        // Fase 2: Radiación
        cmds.insert("radiation::decay", "Decaimiento radiactivo N(t) = N₀·e^(-λt)");
        cmds.insert("radiation::half_life", "Vida media T½ = ln(2)/λ");
        cmds.insert("radiation::activity", "Actividad A = λ·N (Becquerels)");
        cmds.insert("radiation::chain_reaction", "Reacción en cadena de fisión (pasos)");
        cmds.insert("radiation::fission_energy", "Energía de fisión E = Δm·c²");
        cmds.insert("radiation::fusion_energy", "Energía de fusión H+H→He");
        cmds.insert("radiation::geiger_rate", "Tasa de clicks Geiger ∝ radiación");
        cmds.insert("radiation::exposure", "Dosis de exposición (Sieverts)");
        // Fase 2: Sistema solar orbital (Kepler)
        cmds.insert("kepler::orbit", "Órbita elíptica (x,y en tiempo t, ley de Kepler)");
        cmds.insert("kepler::period", "Período orbital T² = 4π²a³/(GM)");
        cmds.insert("kepler::velocity", "Velocidad orbital en punto de órbita");
        cmds.insert("kepler::ellipse_points", "Puntos de elipse completa para dibujar");
        cmds.insert("solar_system::step", "Simulación paso a paso de planetas");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "projectile" => self.projectile(params),
            "nbody_2" => self.nbody_2(params),
            "nbody_simulate" => self.nbody_simulate(params),
            "elastic_collision_2" => self.elastic_collision_2(params),
            "elastic_collision_multi" => self.elastic_collision_multi(params),
            "flocking_step" => self.flocking_step(params),
            "drag_force" => self.drag_force(params),
            "wind_force" => self.wind_force(params),
            "pendulum_period" => self.pendulum_period(params),
            "spring_force" => self.spring_force(params),
            // Fase 2: Radiación
            "radiation::decay" => self.radiation_decay(params),
            "radiation::half_life" => self.radiation_half_life(params),
            "radiation::activity" => self.radiation_activity(params),
            "radiation::chain_reaction" => self.radiation_chain_reaction(params),
            "radiation::fission_energy" => self.radiation_fission_energy(params),
            "radiation::fusion_energy" => self.radiation_fusion_energy(params),
            "radiation::geiger_rate" => self.radiation_geiger_rate(params),
            "radiation::exposure" => self.radiation_exposure(params),
            // Fase 2: Kepler
            "kepler::orbit" => self.kepler_orbit(params),
            "kepler::period" => self.kepler_period(params),
            "kepler::velocity" => self.kepler_velocity(params),
            "kepler::ellipse_points" => self.kepler_ellipse_points(params),
            "solar_system::step" => self.solar_system_step(params),
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}", command),
            }),
        }
    }
}

impl PhysicsModule {
    /// Simulación de proyectil 2D
    ///
    /// # Params
    /// - x0, y0: Posición inicial
    /// - v0: Velocidad inicial (m/s)
    /// - angle: Ángulo en grados
    ///
    /// # Returns
    /// [x_final, y_final, flight_time, max_height, range]
    fn projectile(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 4 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "physics::projectile requires 4 params: x0, y0, v0, angle".to_string(),
            });
        }

        let x0 = arr[0].as_f64().unwrap_or(0.0);
        let y0 = arr[1].as_f64().unwrap_or(0.0);
        let v0 = arr[2].as_f64().unwrap_or(0.0);
        let angle = arr[3].as_f64().unwrap_or(0.0);

        let rad = angle.to_radians();
        let vx = v0 * rad.cos();
        let vy = v0 * rad.sin();
        let g = 9.81;

        let flight_time = 2.0 * vy / g;
        let max_height = (vy * vy) / (2.0 * g);
        let range = vx * flight_time;

        Ok(json!([
            x0 + vx * flight_time, // x final
            y0,                    // y final
            flight_time,           // tiempo vuelo
            max_height,            // altura máxima
            range                  // alcance horizontal
        ]))
    }

    /// Gravedad entre 2 cuerpos (Ley de Newton)
    ///
    /// # Params
    /// - m1, m2: Masas de los cuerpos
    /// - x1, y1: Posición del cuerpo 1
    /// - x2, y2: Posición del cuerpo 2
    /// - G: Constante gravitacional (default: 6.674e-11)
    ///
    /// # Returns
    /// [fx1, fy1, fx2, fy2, distancia]
    fn nbody_2(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "Params must be an array".to_string(),
        })?;

        if arr.len() != 7 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "physics::nbody_2 requires 7 params: m1, m2, x1, y1, x2, y2, G"
                    .to_string(),
            });
        }

        let m1 = arr[0].as_f64().unwrap_or(0.0);
        let m2 = arr[1].as_f64().unwrap_or(0.0);
        let x1 = arr[2].as_f64().unwrap_or(0.0);
        let y1 = arr[3].as_f64().unwrap_or(0.0);
        let x2 = arr[4].as_f64().unwrap_or(0.0);
        let y2 = arr[5].as_f64().unwrap_or(0.0);
        let g = arr[6].as_f64().unwrap_or(6.674e-11);

        let dx = x2 - x1;
        let dy = y2 - y1;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.001 {
            let force = g * m1 * m2 / (dist * dist);
            let fx = force * dx / dist;
            let fy = force * dy / dist;

            Ok(json!([fx, fy, -fx, -fy, dist]))
        } else {
            Ok(json!([0.0, 0.0, 0.0, 0.0, dist]))
        }
    }

    // ========================================================================
    // N-BODY SIMULATION (N cuerpos, O(n²)) — v0.13.1
    // ========================================================================

    /// Simulación N-cuerpos con gravedad mutua
    ///
    /// # Params
    /// - bodies: array de [mass, x, y, vx, vy, is_static] por cuerpo
    /// - dt: delta tiempo
    /// - G: constante gravitacional (default: 6.674e-11)
    ///
    /// # Returns
    /// Array de [x, y, vx, vy] actualizado por cuerpo
    pub fn nbody_simulate(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "nbody_simulate requiere array de bodies + dt + G".to_string(),
        })?;

        if arr.len() < 3 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "nbody_simulate requiere [bodies, dt, G]".to_string(),
            });
        }

        let bodies_arr = arr[0].as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "bodies debe ser array de [mass, x, y, vx, vy, is_static]".to_string(),
        })?;

        let dt = arr[1].as_f64().unwrap_or(0.016);
        let g = arr[2].as_f64().unwrap_or(6.674e-11);

        let n = bodies_arr.len();
        if n == 0 {
            return Ok(json!([]));
        }

        // Parse bodies: [mass, x, y, vx, vy, is_static]
        struct Body {
            mass: f64, x: f64, y: f64, vx: f64, vy: f64, is_static: bool,
            ax: f64, ay: f64,
        }

        let mut bodies: Vec<Body> = bodies_arr.iter().filter_map(|b| {
            let a = b.as_array()?;
            if a.len() < 6 { return None; }
            Some(Body {
                mass: a[0].as_f64()?,
                x: a[1].as_f64()?,
                y: a[2].as_f64()?,
                vx: a[3].as_f64()?,
                vy: a[4].as_f64()?,
                is_static: a[5].as_f64().map(|v| v != 0.0).unwrap_or(false),
                ax: 0.0, ay: 0.0,
            })
        }).collect();

        // Calculate gravitational accelerations: O(n²)
        for i in 0..n {
            for j in (i + 1)..n {
                let dx = bodies[j].x - bodies[i].x;
                let dy = bodies[j].y - bodies[i].y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt();
                if dist < 0.001 { continue; }
                let force = g * bodies[i].mass * bodies[j].mass / dist_sq;
                let ax = force * dx / (dist * bodies[i].mass);
                let ay = force * dy / (dist * bodies[i].mass);
                let ax_j = -force * dx / (dist * bodies[j].mass);
                let ay_j = -force * dy / (dist * bodies[j].mass);
                if !bodies[i].is_static { bodies[i].ax += ax; bodies[i].ay += ay; }
                if !bodies[j].is_static { bodies[j].ax += ax_j; bodies[j].ay += ay_j; }
            }
        }

        // Update positions and velocities
        let result: Vec<Value> = bodies.iter().map(|b| {
            let vx = if b.is_static { b.vx } else { b.vx + b.ax * dt };
            let vy = if b.is_static { b.vy } else { b.vy + b.ay * dt };
            let x = if b.is_static { b.x } else { b.x + vx * dt };
            let y = if b.is_static { b.y } else { b.y + vy * dt };
            json!([x, y, vx, vy])
        }).collect();

        Ok(json!(result))
    }

    // ========================================================================
    // COLISIONES ELÁSTICAS 2D — Conservación de momentum + energía cinética
    // ========================================================================

    /// Colisión elástica entre 2 cuerpos en 2D
    ///
    /// # Params
    /// - m1, x1, y1, vx1, vy1: cuerpo 1
    /// - m2, x2, y2, vx2, vy2: cuerpo 2
    ///
    /// # Returns
    /// [vx1', vy1', vx2', vy2', energía_perdida]
    /// Energía perdida = 0 (elástica perfecta)
    fn elastic_collision_2(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "elastic_collision_2 requiere 10 params".to_string(),
        })?;

        if arr.len() != 10 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "elastic_collision_2: m1, x1, y1, vx1, vy1, m2, x2, y2, vx2, vy2"
                    .to_string(),
            });
        }

        let m1 = arr[0].as_f64().unwrap_or(1.0);
        let x1 = arr[1].as_f64().unwrap_or(0.0);
        let y1 = arr[2].as_f64().unwrap_or(0.0);
        let vx1 = arr[3].as_f64().unwrap_or(0.0);
        let vy1 = arr[4].as_f64().unwrap_or(0.0);
        let m2 = arr[5].as_f64().unwrap_or(1.0);
        let x2 = arr[6].as_f64().unwrap_or(0.0);
        let y2 = arr[7].as_f64().unwrap_or(0.0);
        let vx2 = arr[8].as_f64().unwrap_or(0.0);
        let vy2 = arr[9].as_f64().unwrap_or(0.0);

        // Vector normal (de 1 a 2)
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 0.001 {
            // Sin colisión válida
            return Ok(json!([vx1, vy1, vx2, vy2, 0.0]));
        }

        // Normal unitaria
        let nx = dx / dist;
        let ny = dy / dist;

        // Velocidad relativa en dirección normal
        let dvx = vx1 - vx2;
        let dvy = vy1 - vy2;
        let dvn = dvx * nx + dvy * ny;

        // Si se separan, no hay colisión
        if dvn < 0.0 {
            return Ok(json!([vx1, vy1, vx2, vy2, 0.0]));
        }

        // Impulso (colisión elástica: restitution = 1.0)
        let impulse = 2.0 * dvn / (m1 + m2);

        // Nuevas velocidades
        let vx1_new = vx1 - impulse * m2 * nx;
        let vy1_new = vy1 - impulse * m2 * ny;
        let vx2_new = vx2 + impulse * m1 * nx;
        let vy2_new = vy2 + impulse * m1 * ny;

        Ok(json!([vx1_new, vy1_new, vx2_new, vy2_new, 0.0]))
    }

    /// Colisiones elásticas múltiples (O(n²))
    ///
    /// # Params
    /// - bodies: array de [mass, x, y, vx, vy, radius]
    /// - dt: delta tiempo
    ///
    /// # Returns
    /// Array de [x, y, vx, vy] actualizado
    fn elastic_collision_multi(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "elastic_collision_multi requiere [bodies, dt]".to_string(),
        })?;

        if arr.len() < 2 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "elastic_collision_multi: [bodies, dt]".to_string(),
            });
        }

        let bodies_arr = arr[0].as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "bodies: [mass, x, y, vx, vy, radius]".to_string(),
        })?;

        let dt = arr[1].as_f64().unwrap_or(0.016);
        let n = bodies_arr.len();
        if n == 0 { return Ok(json!([])); }

        struct Body {
            mass: f64, x: f64, y: f64, vx: f64, vy: f64, radius: f64,
        }

        let mut bodies: Vec<Body> = bodies_arr.iter().filter_map(|b| {
            let a = b.as_array()?;
            if a.len() < 6 { return None; }
            Some(Body {
                mass: a[0].as_f64()?, x: a[1].as_f64()?, y: a[2].as_f64()?,
                vx: a[3].as_f64()?, vy: a[4].as_f64()?, radius: a[5].as_f64()?,
            })
        }).collect();

        // Detectar colisiones y resolver
        for i in 0..n {
            for j in (i + 1)..n {
                let dx = bodies[j].x - bodies[i].x;
                let dy = bodies[j].y - bodies[i].y;
                let dist = (dx * dx + dy * dy).sqrt();
                let min_dist = bodies[i].radius + bodies[j].radius;

                if dist < min_dist && dist > 0.001 {
                    let nx = dx / dist;
                    let ny = dy / dist;
                    let dvx = bodies[i].vx - bodies[j].vx;
                    let dvy = bodies[i].vy - bodies[j].vy;
                    let dvn = dvx * nx + dvy * ny;

                    if dvn > 0.0 {
                        let impulse = 2.0 * dvn / (bodies[i].mass + bodies[j].mass);
                        bodies[i].vx -= impulse * bodies[j].mass * nx;
                        bodies[i].vy -= impulse * bodies[j].mass * ny;
                        bodies[j].vx += impulse * bodies[i].mass * nx;
                        bodies[j].vy += impulse * bodies[i].mass * ny;

                        // Separar cuerpos que se superponen
                        let overlap = min_dist - dist;
                        let sep = overlap / 2.0 + 0.01;
                        bodies[i].x -= nx * sep;
                        bodies[i].y -= ny * sep;
                        bodies[j].x += nx * sep;
                        bodies[j].y += ny * sep;
                    }
                }
            }
        }

        // Update positions
        let result: Vec<Value> = bodies.iter().map(|b| {
            json!([b.x + b.vx * dt, b.y + b.vy * dt, b.vx, b.vy])
        }).collect();

        Ok(json!(result))
    }

    // ========================================================================
    // FLOCKING — Reglas de Reynolds (cohesión + separación + alineación)
    // ========================================================================

    /// Paso de flocking (Reynolds)
    ///
    /// # Params
    /// - boids: array de [x, y, vx, vy] por boid
    /// - perception_radius: radio de percepción (default: 50)
    /// - separation_radius: radio de separación (default: 25)
    /// - coh_weight, align_weight, sep_weight: pesos de cada regla (default: 1.0)
    /// - max_speed: velocidad máxima (default: 5)
    ///
    /// # Returns
    /// Array de [x, y, vx, vy] actualizado
    fn flocking_step(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "flocking_step requiere [boids, perception_r, separation_r, coh_w, align_w, sep_w, max_speed]".to_string(),
        })?;

        if arr.len() < 7 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "flocking_step: 7 params requeridos".to_string(),
            });
        }

        let boids_arr = arr[0].as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "boids: [x, y, vx, vy]".to_string(),
        })?;

        let perception_r = arr[1].as_f64().unwrap_or(50.0);
        let separation_r = arr[2].as_f64().unwrap_or(25.0);
        let coh_w = arr[3].as_f64().unwrap_or(1.0);
        let align_w = arr[4].as_f64().unwrap_or(1.0);
        let sep_w = arr[5].as_f64().unwrap_or(1.5);
        let max_speed = arr[6].as_f64().unwrap_or(5.0);

        let n = boids_arr.len();
        if n == 0 { return Ok(json!([])); }

        struct Boid { x: f64, y: f64, vx: f64, vy: f64 }

        let boids: Vec<Boid> = boids_arr.iter().filter_map(|b| {
            let a = b.as_array()?;
            if a.len() < 4 { return None; }
            Some(Boid {
                x: a[0].as_f64()?, y: a[1].as_f64()?,
                vx: a[2].as_f64()?, vy: a[3].as_f64()?,
            })
        }).collect();

        let mut result = Vec::new();

        for i in 0..n {
            let mut coh_x = 0.0;
            let mut coh_y = 0.0;
            let mut coh_count = 0;
            let mut align_vx = 0.0;
            let mut align_vy = 0.0;
            let mut align_count = 0;
            let mut sep_x = 0.0;
            let mut sep_y = 0.0;

            for j in 0..n {
                if i == j { continue; }
                let dx = boids[j].x - boids[i].x;
                let dy = boids[j].y - boids[i].y;
                let dist = (dx * dx + dy * dy).sqrt();

                // Cohesión: centro de masa de vecinos
                if dist < perception_r {
                    coh_x += boids[j].x;
                    coh_y += boids[j].y;
                    coh_count += 1;

                    // Alineación: velocidad promedio de vecinos
                    align_vx += boids[j].vx;
                    align_vy += boids[j].vy;
                    align_count += 1;
                }

                // Separación: repulsión de cercanos
                if dist < separation_r && dist > 0.001 {
                    sep_x -= dx / dist;
                    sep_y -= dy / dist;
                }
            }

            // Aplicar reglas
            let mut new_vx = boids[i].vx;
            let mut new_vy = boids[i].vy;

            if coh_count > 0 {
                new_vx += ((coh_x / coh_count as f64) - boids[i].x) * coh_w * 0.01;
                new_vy += ((coh_y / coh_count as f64) - boids[i].y) * coh_w * 0.01;
            }

            if align_count > 0 {
                new_vx += ((align_vx / align_count as f64) - boids[i].vx) * align_w * 0.05;
                new_vy += ((align_vy / align_count as f64) - boids[i].vy) * align_w * 0.05;
            }

            new_vx += sep_x * sep_w * 0.1;
            new_vy += sep_y * sep_w * 0.1;

            // Limitar velocidad
            let speed = (new_vx * new_vx + new_vy * new_vy).sqrt();
            if speed > max_speed {
                new_vx = new_vx / speed * max_speed;
                new_vy = new_vy / speed * max_speed;
            }

            let new_x = boids[i].x + new_vx;
            let new_y = boids[i].y + new_vy;

            result.push(json!([new_x, new_y, new_vx, new_vy]));
        }

        Ok(json!(result))
    }

    // ========================================================================
    // FUERZA DE ARRASTRE — F_d = ½ · ρ · v² · C_d · A
    // ========================================================================

    /// Fuerza de arrastre/fricción del aire
    ///
    /// # Params
    /// - vx, vy: velocidad
    /// - density: densidad del fluido (aire = 1.225 kg/m³)
    /// - cd: coeficiente de arrastre (esfera = 0.47, paracaídas = 1.5)
    /// - area: área transversal (m²)
    ///
    /// # Returns
    /// [fx, fy, magnitude]
    fn drag_force(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "drag_force: vx, vy, density, cd, area".to_string(),
        })?;

        if arr.len() != 5 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "drag_force requiere 5 params".to_string(),
            });
        }

        let vx = arr[0].as_f64().unwrap_or(0.0);
        let vy = arr[1].as_f64().unwrap_or(0.0);
        let density = arr[2].as_f64().unwrap_or(1.225);
        let cd = arr[3].as_f64().unwrap_or(0.47);
        let area = arr[4].as_f64().unwrap_or(1.0);

        let speed = (vx * vx + vy * vy).sqrt();
        if speed < 0.001 {
            return Ok(json!([0.0, 0.0, 0.0]));
        }

        let magnitude = 0.5 * density * speed * speed * cd * area;
        let fx = -magnitude * vx / speed;
        let fy = -magnitude * vy / speed;

        Ok(json!([fx, fy, magnitude]))
    }

    // ========================================================================
    // FUERZA DE VIENTO — Vector variable con turbulencia simple
    // ========================================================================

    /// Fuerza de viento con turbulencia
    ///
    /// # Params
    /// - x, y: posición
    /// - base_wx, base_wy: viento base
    /// - turbulence: intensidad turbulencia (0-1)
    /// - time: tiempo actual (para variación)
    ///
    /// # Returns
    /// [wx, wy, magnitude]
    fn wind_force(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "wind_force: x, y, base_wx, base_wy, turbulence, time".to_string(),
        })?;

        if arr.len() != 6 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "wind_force requiere 6 params".to_string(),
            });
        }

        let x = arr[0].as_f64().unwrap_or(0.0);
        let y = arr[1].as_f64().unwrap_or(0.0);
        let base_wx = arr[2].as_f64().unwrap_or(1.0);
        let base_wy = arr[3].as_f64().unwrap_or(0.0);
        let turbulence = arr[4].as_f64().unwrap_or(0.0).clamp(0.0, 1.0);
        let time = arr[5].as_f64().unwrap_or(0.0);

        // Turbulencia simple con seno/coseno (simula Perlin noise básico)
        let turb_x = (x * 0.1 + time * 1.3).sin() * turbulence * 2.0
                    + (y * 0.15 + time * 0.7).cos() * turbulence;
        let turb_y = (y * 0.12 + time * 1.1).sin() * turbulence * 1.5
                    + (x * 0.08 + time * 0.9).cos() * turbulence * 0.5;

        let wx = base_wx + turb_x;
        let wy = base_wy + turb_y;
        let mag = (wx * wx + wy * wy).sqrt();

        Ok(json!([wx, wy, mag]))
    }

    // ========================================================================
    // PÉNDULO SIMPLE — T = 2π · √(L/g)
    // ========================================================================

    /// Período de péndulo simple
    ///
    /// # Params
    /// - length: longitud del péndulo (m)
    /// - gravity: gravedad (default: 9.81 m/s²)
    ///
    /// # Returns
    /// [period, frequency, angular_freq]
    fn pendulum_period(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "pendulum_period: length, gravity".to_string(),
        })?;

        let length = arr[0].as_f64().unwrap_or(1.0);
        let g = arr[1].as_f64().unwrap_or(9.81);

        if length <= 0.0 || g <= 0.0 {
            return Ok(json!([0.0, 0.0, 0.0]));
        }

        let period = 2.0 * std::f64::consts::PI * (length / g).sqrt();
        let freq = 1.0 / period;
        let angular = 2.0 * std::f64::consts::PI * freq;

        Ok(json!([period, freq, angular]))
    }

    // ========================================================================
    // RESORTE — F = -kx (Ley de Hooke)
    // ========================================================================

    /// Fuerza de resorte (Hooke)
    ///
    /// # Params
    /// - k: constante del resorte (N/m)
    /// - displacement: desplazamiento desde equilibrio (m)
    /// - damping: coeficiente de amortiguamiento (default: 0)
    /// - velocity: velocidad actual (para damping)
    ///
    /// # Returns
    /// [force, potential_energy, kinetic_energy_if_damped]
    fn spring_force(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "spring_force: k, displacement, damping, velocity".to_string(),
        })?;

        let k = arr[0].as_f64().unwrap_or(10.0);
        let x = arr[1].as_f64().unwrap_or(0.0);
        let damping = arr[2].as_f64().unwrap_or(0.0);
        let v = arr[3].as_f64().unwrap_or(0.0);

        let force = -k * x - damping * v;
        let pe = 0.5 * k * x * x;

        Ok(json!([force, pe, damping * v * v]))
    }

    // ========================================================================
    // RADIACIÓN — Decaimiento, fisión, fusión, Geiger
    // ========================================================================

    /// Decaimiento radiactivo: N(t) = N₀ · e^(-λt)
    ///
    /// # Params
    /// - n0: cantidad inicial de núcleos
    /// - lambda: constante de decaimiento (s⁻¹)
    /// - time: tiempo transcurrido (s)
    ///
    /// # Returns
    /// [remaining_n, decayed_n, fraction_remaining]
    fn radiation_decay(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "decay: n0, lambda, time".to_string(),
        })?;

        let n0 = arr[0].as_f64().unwrap_or(1000.0);
        let lambda = arr[1].as_f64().unwrap_or(0.1);
        let t = arr[2].as_f64().unwrap_or(1.0);

        let remaining = n0 * (-lambda * t).exp();
        let decayed = n0 - remaining;
        let fraction = remaining / n0;

        Ok(json!([remaining, decayed, fraction]))
    }

    /// Vida media: T½ = ln(2) / λ
    ///
    /// # Params
    /// - lambda: constante de decaimiento (s⁻¹)
    ///
    /// # Returns
    /// [half_life, mean_lifetime]
    fn radiation_half_life(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "half_life: lambda".to_string(),
        })?;

        let lambda = arr[0].as_f64().unwrap_or(0.1);
        if lambda <= 0.0 {
            return Ok(json!([0.0, 0.0]));
        }

        let half_life = std::f64::consts::LN_2 / lambda;
        let mean_lifetime = 1.0 / lambda;

        Ok(json!([half_life, mean_lifetime]))
    }

    /// Actividad radiactiva: A = λ · N (Becquerels)
    ///
    /// # Params
    /// - lambda: constante de decaimiento (s⁻¹)
    /// - n: número de núcleos
    ///
    /// # Returns
    /// [activity_Bq, activity_Ci] (Becquerels y Curies)
    fn radiation_activity(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "activity: lambda, n".to_string(),
        })?;

        let lambda = arr[0].as_f64().unwrap_or(0.1);
        let n = arr[1].as_f64().unwrap_or(1000.0);

        let bq = lambda * n;
        let ci = bq / 3.7e10; // 1 Ci = 3.7×10¹⁰ Bq

        Ok(json!([bq, ci]))
    }

    /// Reacción en cadena de fisión (simulación por pasos)
    ///
    /// # Params
    /// - initial_neutrons: neutrones iniciales
    /// - multiplication_factor: factor de multiplicación (k)
    /// - steps: número de generaciones
    /// - absorption: probabilidad de absorción (0-1)
    ///
    /// # Returns
    /// Array de [step, neutrons, cumulative_fissions]
    fn radiation_chain_reaction(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "chain_reaction: initial_n, k, steps, absorption".to_string(),
        })?;

        let mut neutrons = arr[0].as_f64().unwrap_or(1.0);
        let k = arr[1].as_f64().unwrap_or(2.5);
        let steps = arr[2].as_i64().unwrap_or(10) as usize;
        let absorption = arr[3].as_f64().unwrap_or(0.3).clamp(0.0, 1.0);

        let mut result = Vec::new();
        let mut cumulative = 0.0;

        for step in 0..=steps {
            let fissions = neutrons * (1.0 - absorption);
            cumulative += fissions;
            result.push(json!([step as i64, neutrons, cumulative]));
            neutrons = neutrons * k * (1.0 - absorption);
            // Limitar para evitar overflow
            if neutrons > 1e15 { break; }
        }

        Ok(json!(result))
    }

    /// Energía de fisión: E = Δm · c²
    ///
    /// # Params
    /// - mass_defect: defecto de masa (kg)
    ///
    /// # Returns
    /// [energy_joules, energy_MeV, energy_kwh]
    fn radiation_fission_energy(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "fission_energy: mass_defect_kg".to_string(),
        })?;

        let dm = arr[0].as_f64().unwrap_or(3.56e-28); // ~200 MeV por fisión U-235
        let c = 299_792_458.0; // m/s

        let joules = dm * c * c;
        let mev = joules / 1.602e-13; // 1 eV = 1.602×10⁻¹⁹ J
        let kwh = joules / 3.6e6;

        Ok(json!([joules, mev, kwh]))
    }

    /// Energía de fusión: H + H → He + energía
    ///
    /// # Params
    /// - num_reactions: número de reacciones de fusión
    ///
    /// # Returns
    /// [energy_joules, energy_MeV, mass_defect_kg]
    fn radiation_fusion_energy(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "fusion_energy: num_reactions".to_string(),
        })?;

        let n = arr[0].as_f64().unwrap_or(1.0);
        // Fusión D-T: ~17.6 MeV por reacción
        let mev_per_reaction = 17.6;
        let joules_per_reaction = 17.6 * 1.602e-13;
        let c = 299_792_458.0;

        let total_joules = n * joules_per_reaction;
        let total_mev = n * mev_per_reaction;
        let mass_defect = total_joules / (c * c);

        Ok(json!([total_joules, total_mev, mass_defect]))
    }

    /// Tasa de clicks del contador Geiger: f = f₀ + k · radiación
    ///
    /// # Params
    /// - radiation_level: nivel de radiación (Bq/m³)
    /// - base_rate: tasa base sin radiación (clicks/s)
    /// - sensitivity: sensibilidad del detector (clicks por Bq)
    ///
    /// # Returns
    /// [click_rate, clicks_per_minute, is_dangerous]
    fn radiation_geiger_rate(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "geiger_rate: radiation, base_rate, sensitivity".to_string(),
        })?;

        let radiation = arr[0].as_f64().unwrap_or(0.0);
        let base = arr[1].as_f64().unwrap_or(0.5); // ~30 CPM background
        let sensitivity = arr[2].as_f64().unwrap_or(1.0);

        let rate = base + sensitivity * radiation;
        let cpm = rate * 60.0;
        let dangerous = radiation > 100.0; // >100 Bq/m³ = peligro

        Ok(json!([rate, cpm, dangerous]))
    }

    /// Dosis de exposición: Sieverts (Sv)
    ///
    /// # Params
    /// - energy_deposited: energía depositada (J/kg)
    /// - radiation_type: 1=alpha, 2=beta, 3=gamma, 4=neutron
    ///
    /// # Returns
    /// [dose_sieverts, dose_millisieverts, risk_level]
    fn radiation_exposure(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "exposure: energy_deposited, radiation_type".to_string(),
        })?;

        let energy = arr[0].as_f64().unwrap_or(0.001); // J/kg = Gray
        let rtype = arr[1].as_i64().unwrap_or(3) as u8; // default gamma

        // Factor de ponderación por tipo de radiación
        let wr = match rtype {
            1 => 20.0, // Alpha
            2 => 1.0,  // Beta
            3 => 1.0,  // Gamma
            4 => 10.0, // Neutron
            _ => 1.0,
        };

        let sieverts = energy * wr;
        let millisieverts = sieverts * 1000.0;

        // Nivel de riesgo
        let risk = if millisieverts < 1.0 {
            "safe"
        } else if millisieverts < 50.0 {
            "moderate"
        } else if millisieverts < 1000.0 {
            "dangerous"
        } else {
            "lethal"
        };

        Ok(json!([sieverts, millisieverts, risk]))
    }

    // ========================================================================
    // SISTEMA SOLAR — Leyes de Kepler
    // ========================================================================

    /// Posición en órbita elíptica en tiempo t (Ley de Kepler)
    ///
    /// Ecuación de Kepler: M = E - e·sin(E), donde M = anomalía media
    /// Se resuelve iterativamente para E (anomalía excéntrica).
    ///
    /// # Params
    /// - a: semieje mayor (distancia media al sol)
    /// - e: excentricidad (0=circular, 0<e<1=elíptica)
    /// - period: período orbital (tiempo para una vuelta completa)
    /// - time: tiempo actual
    ///
    /// # Returns
    /// [x, y, true_anomaly, distance_from_focus]
    fn kepler_orbit(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "kepler::orbit: a, e, period, time".to_string(),
        })?;

        if arr.len() != 4 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "kepler::orbit requiere 4 params".to_string(),
            });
        }

        let a = arr[0].as_f64().unwrap_or(1.0);
        let e = arr[1].as_f64().unwrap_or(0.0).clamp(0.0, 0.999);
        let period = arr[2].as_f64().unwrap_or(1.0);
        let time = arr[3].as_f64().unwrap_or(0.0);

        // Anomalía media
        let m = (2.0 * std::f64::consts::PI * time / period) % (2.0 * std::f64::consts::PI);

        // Resolver ecuación de Kepler: E - e*sin(E) = M (Newton-Raphson)
        let mut e_anom = m;
        for _ in 0..20 {
            let d_e = (e_anom - e * e_anom.sin() - m) / (1.0 - e * e_anom.cos());
            e_anom -= d_e;
            if d_e.abs() < 1e-10 { break; }
        }

        // Anomalía verdadera
        let cos_v = (e_anom.cos() - e) / (1.0 - e * e_anom.cos());
        let sin_v = (1.0 - e * e).sqrt() * e_anom.sin() / (1.0 - e * e_anom.cos());
        let true_anomaly = sin_v.atan2(cos_v);

        // Distancia al foco (sol)
        let r = a * (1.0 - e * e) / (1.0 + e * true_anomaly.cos());

        // Posición
        let x = r * true_anomaly.cos();
        let y = r * true_anomaly.sin();

        Ok(json!([x, y, true_anomaly, r]))
    }

    /// Período orbital: T² = 4π²a³/(GM)
    ///
    /// # Params
    /// - a: semieje mayor
    /// - m_central: masa del cuerpo central
    /// - G: constante gravitacional (default: 6.674e-11)
    ///
    /// # Returns
    /// [period, frequency, angular_velocity]
    fn kepler_period(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "kepler::period: a, m_central, G".to_string(),
        })?;

        let a = arr[0].as_f64().unwrap_or(1.0);
        let m = arr[1].as_f64().unwrap_or(1.0);
        let g = arr[2].as_f64().unwrap_or(6.674e-11);

        if a <= 0.0 || m <= 0.0 {
            return Ok(json!([0.0, 0.0, 0.0]));
        }

        // T = 2π · sqrt(a³/(GM))
        let period = 2.0 * std::f64::consts::PI * (a * a * a / (g * m)).sqrt();
        let freq = 1.0 / period;
        let angular = 2.0 * std::f64::consts::PI * freq;

        Ok(json!([period, freq, angular]))
    }

    /// Velocidad orbital en un punto (vis-viva: v² = GM(2/r - 1/a))
    ///
    /// # Params
    /// - a: semieje mayor
    /// - r: distancia actual al cuerpo central
    /// - m_central: masa del cuerpo central
    /// - G: constante gravitacional (default: 6.674e-11)
    ///
    /// # Returns
    /// [velocity, kinetic_energy_per_unit_mass, potential_energy_per_unit_mass]
    fn kepler_velocity(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "kepler::velocity: a, r, m_central, G".to_string(),
        })?;

        let a = arr[0].as_f64().unwrap_or(1.0);
        let r = arr[1].as_f64().unwrap_or(1.0);
        let m = arr[2].as_f64().unwrap_or(1.0);
        let g = arr[3].as_f64().unwrap_or(6.674e-11);

        if r < 0.001 || a <= 0.0 {
            return Ok(json!([0.0, 0.0, 0.0]));
        }

        // Vis-viva: v² = GM(2/r - 1/a)
        let v_sq = g * m * (2.0 / r - 1.0 / a);
        let v = if v_sq > 0.0 { v_sq.sqrt() } else { 0.0 };
        let ke = 0.5 * v * v;
        let pe = -g * m / r;

        Ok(json!([v, ke, pe]))
    }

    /// Generar puntos de una elipse completa para dibujar la órbita
    ///
    /// # Params
    /// - a: semieje mayor
    /// - e: excentricidad
    /// - focus_x, focus_y: posición del foco (sol)
    /// - num_points: número de puntos (default: 64)
    ///
    /// # Returns
    /// Array de puntos [[x, y], ...] de la elipse
    fn kepler_ellipse_points(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "kepler::ellipse_points: a, e, fx, fy, num_points".to_string(),
        })?;

        let a = arr[0].as_f64().unwrap_or(100.0);
        let e = arr[1].as_f64().unwrap_or(0.0).clamp(0.0, 0.999);
        let fx = arr[2].as_f64().unwrap_or(400.0);
        let fy = arr[3].as_f64().unwrap_or(300.0);
        let n = arr[4].as_i64().unwrap_or(64) as usize;

        let b = a * (1.0 - e * e).sqrt(); // semieje menor
        let c = a * e; // distancia foco a centro

        // Elipse centrada en (fx + c, fy) con foco en (fx, fy)
        let cx = fx + c;
        let cy = fy;

        let points: Vec<Value> = (0..n).map(|i| {
            let angle = (i as f64) / (n as f64) * 2.0 * std::f64::consts::PI;
            let x = cx + a * angle.cos();
            let y = cy + b * angle.sin();
            json!([x, y])
        }).collect();

        Ok(json!({ "points": points, "count": n, "semi_major": a, "semi_minor": b, "eccentricity": e }))
    }

    /// Simular paso a paso un sistema solar con múltiples planetas
    ///
    /// # Params
    /// - planets: array de {name, a, e, period, angle_start, color}
    /// - sun_x, sun_y: posición del sol
    /// - dt: delta tiempo
    ///
    /// # Returns
    /// Array de planetas con nueva posición {name, x, y, angle, distance}
    fn solar_system_step(&self, params: Value) -> ModuleResult {
        let arr = params.as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "solar_system::step: planets, sun_x, sun_y, dt".to_string(),
        })?;

        if arr.len() < 4 {
            return Err(ModuleError {
                code: "INVALID_PARAMS".to_string(),
                message: "solar_system::step requiere 4 params".to_string(),
            });
        }

        let planets_arr = arr[0].as_array().ok_or_else(|| ModuleError {
            code: "INVALID_PARAMS".to_string(),
            message: "planets: array de planetas".to_string(),
        })?;

        let sun_x = arr[1].as_f64().unwrap_or(400.0);
        let sun_y = arr[2].as_f64().unwrap_or(300.0);
        let dt = arr[3].as_f64().unwrap_or(0.016);

        let mut time = 0.0; // Se podría pasar como parámetro opcional

        let result: Vec<Value> = planets_arr.iter().filter_map(|p| {
            let obj = p.as_object()?;
            let name = obj.get("name")?.as_str()?.to_string();
            let a = obj.get("a")?.as_f64()?;
            let e = obj.get("e")?.as_f64()?;
            let period = obj.get("period")?.as_f64()?;
            let angle_start = obj.get("angle_start").and_then(|v| v.as_f64()).unwrap_or(0.0);

            // Calcular posición orbital
            let t = angle_start / (2.0 * std::f64::consts::PI) * period + time + dt;
            let m = (2.0 * std::f64::consts::PI * t / period) % (2.0 * std::f64::consts::PI);

            // Kepler: E - e*sin(E) = M
            let mut e_anom = m;
            for _ in 0..15 {
                let d_e = (e_anom - e * e_anom.sin() - m) / (1.0 - e * e_anom.cos());
                e_anom -= d_e;
                if d_e.abs() < 1e-8 { break; }
            }

            let cos_v = (e_anom.cos() - e) / (1.0 - e * e_anom.cos());
            let sin_v = (1.0 - e * e).sqrt() * e_anom.sin() / (1.0 - e * e_anom.cos());
            let true_anomaly = sin_v.atan2(cos_v);
            let r = a * (1.0 - e * e) / (1.0 + e * true_anomaly.cos());

            let x = sun_x + r * true_anomaly.cos();
            let y = sun_y + r * true_anomaly.sin();

            let new_angle = (true_anomaly + angle_start) % (2.0 * std::f64::consts::PI);

            Some(json!({
                "name": name,
                "x": x,
                "y": y,
                "angle": new_angle,
                "distance": r,
            }))
        }).collect();

        Ok(json!(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_module_name() {
        let module = PhysicsModule;
        assert_eq!(module.name(), "physics");
        assert_eq!(module.version(), "0.7.3");
    }

    #[test]
    fn test_physics_register() {
        let module = PhysicsModule;
        let cmds = module.register();

        assert!(cmds.contains_key("projectile"));
        assert!(cmds.contains_key("nbody_2"));
    }

    #[test]
    fn test_projectile() {
        let module = PhysicsModule;
        // x0=0, y0=0, v0=10, angle=45
        let params = json!([0.0, 0.0, 10.0, 45.0]);
        let result = module.execute("projectile", params).unwrap();

        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 5);
        // flight_time ≈ 2*vy/g = 2*(10*sin(45))/9.81 ≈ 1.44s
        let flight_time = arr[2].as_f64().unwrap();
        assert!(flight_time > 1.4 && flight_time < 1.5);
    }

    #[test]
    fn test_nbody_2() {
        let module = PhysicsModule;
        // m1=100, m2=200, x1=0, y1=0, x2=10, y2=0, G=1.0
        let params = json!([100.0, 200.0, 0.0, 0.0, 10.0, 0.0, 1.0]);
        let result = module.execute("nbody_2", params).unwrap();

        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 5);
        // fx = G*m1*m2/dist^2 * dx/dist = 1*100*200/100 * 10/10 = 200
        let fx = arr[0].as_f64().unwrap();
        assert!((fx - 200.0).abs() < 0.01);
    }

    #[test]
    fn test_nbody_2_close() {
        let module = PhysicsModule;
        // Cuerpos muy cercanos (dist < 0.001)
        let params = json!([100.0, 200.0, 0.0, 0.0, 0.0001, 0.0, 1.0]);
        let result = module.execute("nbody_2", params).unwrap();

        let arr = result.as_array().unwrap();
        assert_eq!(arr[0].as_f64().unwrap(), 0.0);
        assert_eq!(arr[1].as_f64().unwrap(), 0.0);
    }

    #[test]
    fn test_unknown_command() {
        let module = PhysicsModule;
        let result = module.execute("unknown", json!([]));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "UNKNOWN_COMMAND");
    }

    // ========================================================================
    // TESTS — FASE 1: Física avanzada (colisiones, flocking, arrastre, viento)
    // ========================================================================

    #[test]
    fn test_elastic_collision_2_head_on() {
        let module = PhysicsModule;
        // Dos masas iguales, una moviéndose hacia la otra estacionaria
        // m1=1, v1=10 → m2=1, v2=0 → después: m1=0, m2=10
        let params = json!([1.0, 0.0, 0.0, 10.0, 0.0, 1.0, 10.0, 0.0, 0.0, 0.0]);
        let result = module.execute("elastic_collision_2", params).unwrap();
        let arr = result.as_array().unwrap();

        let vx1_new = arr[0].as_f64().unwrap();
        let vx2_new = arr[2].as_f64().unwrap();

        // Masas iguales: intercambian velocidades
        assert!((vx1_new - 0.0).abs() < 0.01);
        assert!((vx2_new - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_elastic_collision_2_different_masses() {
        let module = PhysicsModule;
        // Masa ligera golpea masa pesada estacionaria
        let params = json!([1.0, 0.0, 0.0, 10.0, 0.0, 10.0, 10.0, 0.0, 0.0, 0.0]);
        let result = module.execute("elastic_collision_2", params).unwrap();
        let arr = result.as_array().unwrap();

        let vx1_new = arr[0].as_f64().unwrap();
        let vx2_new = arr[2].as_f64().unwrap();

        // Masa ligera rebota, pesada avanza lento
        assert!(vx1_new < 0.0); // rebota
        assert!(vx2_new > 0.0); // avanza
        assert!(vx2_new < 2.0); // lento
    }

    #[test]
    fn test_elastic_collision_no_collision() {
        let module = PhysicsModule;
        // Cuerpos que se separan (dvn < 0)
        let params = json!([1.0, 0.0, 0.0, -5.0, 0.0, 1.0, 10.0, 0.0, 5.0, 0.0]);
        let result = module.execute("elastic_collision_2", params).unwrap();
        let arr = result.as_array().unwrap();

        assert_eq!(arr[0].as_f64().unwrap(), -5.0); // sin cambio
        assert_eq!(arr[2].as_f64().unwrap(), 5.0);
    }

    #[test]
    fn test_elastic_collision_multi() {
        let module = PhysicsModule;
        // 3 bolas de billar en línea
        let bodies = json!([
            [1.0, 0.0, 0.0, 5.0, 0.0, 2.0],
            [1.0, 5.0, 0.0, 0.0, 0.0, 2.0],
            [1.0, 10.0, 0.0, 0.0, 0.0, 2.0]
        ]);
        let params = json!([bodies, 0.016]);
        let result = module.execute("elastic_collision_multi", params).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_flocking_step() {
        let module = PhysicsModule;
        let boids = json!([
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 0.0, 1.0, 0.0],
            [5.0, 5.0, 0.0, 1.0]
        ]);
        let params = json!([boids, 50.0, 25.0, 1.0, 1.0, 1.5, 5.0]);
        let result = module.execute("flocking_step", params).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_flocking_cohesion() {
        let module = PhysicsModule;
        // Boids dispersos con alta cohesión deberían converger
        let boids = json!([
            [0.0, 0.0, 0.0, 0.0],
            [100.0, 0.0, 0.0, 0.0],
            [50.0, 86.6, 0.0, 0.0] // triángulo equilátero
        ]);
        let params = json!([boids, 200.0, 10.0, 5.0, 0.0, 0.0, 5.0]);
        let result = module.execute("flocking_step", params).unwrap();
        let arr = result.as_array().unwrap();

        // Todos deberían moverse hacia el centro (~50, 29)
        let b1_x = arr[0].as_array().unwrap()[0].as_f64().unwrap();
        assert!(b1_x > 0.0); // se mueve hacia el centro
    }

    #[test]
    fn test_drag_force() {
        let module = PhysicsModule;
        // Esfera rápida en aire
        let params = json!([10.0, 0.0, 1.225, 0.47, 1.0]);
        let result = module.execute("drag_force", params).unwrap();
        let arr = result.as_array().unwrap();

        let fx = arr[0].as_f64().unwrap();
        let fy = arr[1].as_f64().unwrap();
        let mag = arr[2].as_f64().unwrap();

        assert!(fx < 0.0); // opuesta al movimiento
        assert_eq!(fy, 0.0);
        assert!(mag > 0.0);
        // F = 0.5 * 1.225 * 100 * 0.47 * 1.0 ≈ 28.8
        assert!((mag - 28.7875).abs() < 0.1);
    }

    #[test]
    fn test_drag_force_stationary() {
        let module = PhysicsModule;
        let params = json!([0.0, 0.0, 1.225, 0.47, 1.0]);
        let result = module.execute("drag_force", params).unwrap();
        let arr = result.as_array().unwrap();

        assert_eq!(arr[0].as_f64().unwrap(), 0.0);
        assert_eq!(arr[1].as_f64().unwrap(), 0.0);
    }

    #[test]
    fn test_wind_force() {
        let module = PhysicsModule;
        let params = json!([0.0, 0.0, 5.0, 0.0, 0.0, 0.0]);
        let result = module.execute("wind_force", params).unwrap();
        let arr = result.as_array().unwrap();

        // Sin turbulencia: viento base
        assert!((arr[0].as_f64().unwrap() - 5.0).abs() < 0.01);
        assert_eq!(arr[1].as_f64().unwrap(), 0.0);
    }

    #[test]
    fn test_wind_force_turbulent() {
        let module = PhysicsModule;
        let params = json!([0.0, 0.0, 5.0, 0.0, 0.5, 1.0]);
        let result = module.execute("wind_force", params).unwrap();
        let arr = result.as_array().unwrap();

        // Con turbulencia: diferente de base
        let wx = arr[0].as_f64().unwrap();
        let wy = arr[1].as_f64().unwrap();
        // turbulencia afecta wx y wy
        assert!(wx != 5.0 || wy != 0.0);
    }

    #[test]
    fn test_pendulum_period() {
        let module = PhysicsModule;
        // Péndulo de 1m en Tierra: T ≈ 2.006s
        let params = json!([1.0, 9.81]);
        let result = module.execute("pendulum_period", params).unwrap();
        let arr = result.as_array().unwrap();

        let period = arr[0].as_f64().unwrap();
        assert!(period > 2.0 && period < 2.1);
    }

    #[test]
    fn test_spring_force() {
        let module = PhysicsModule;
        // Resorte k=100, desplazado 0.5m, sin damping
        let params = json!([100.0, 0.5, 0.0, 0.0]);
        let result = module.execute("spring_force", params).unwrap();
        let arr = result.as_array().unwrap();

        let force = arr[0].as_f64().unwrap();
        let pe = arr[1].as_f64().unwrap();

        assert!((force - (-50.0)).abs() < 0.01); // F = -kx = -50
        assert!((pe - 12.5).abs() < 0.01); // PE = 0.5 * 100 * 0.25 = 12.5
    }

    #[test]
    fn test_spring_force_with_damping() {
        let module = PhysicsModule;
        let params = json!([100.0, 0.5, 5.0, 2.0]);
        let result = module.execute("spring_force", params).unwrap();
        let arr = result.as_array().unwrap();

        let force = arr[0].as_f64().unwrap();
        // F = -kx - damping*v = -50 - 10 = -60
        assert!((force - (-60.0)).abs() < 0.01);
    }

    #[test]
    fn test_physics_register_complete() {
        let module = PhysicsModule;
        let cmds = module.register();

        assert!(cmds.contains_key("elastic_collision_2"));
        assert!(cmds.contains_key("elastic_collision_multi"));
        assert!(cmds.contains_key("flocking_step"));
        assert!(cmds.contains_key("drag_force"));
        assert!(cmds.contains_key("wind_force"));
        assert!(cmds.contains_key("pendulum_period"));
        assert!(cmds.contains_key("spring_force"));
        assert_eq!(cmds.len(), 23); // 10 original + 8 radiation + 5 kepler
    }

    // ========================================================================
    // TESTS — FASE 2: Radiación
    // ========================================================================

    #[test]
    fn test_radiation_decay() {
        let module = PhysicsModule;
        // N0=1000, lambda=0.1, t=1 → N ≈ 904.8
        let params = json!([1000.0, 0.1, 1.0]);
        let result = module.execute("radiation::decay", params).unwrap();
        let arr = result.as_array().unwrap();

        let remaining = arr[0].as_f64().unwrap();
        let decayed = arr[1].as_f64().unwrap();
        let fraction = arr[2].as_f64().unwrap();

        assert!((remaining - 904.8).abs() < 1.0);
        assert!((decayed - 95.2).abs() < 1.0);
        assert!((fraction - 0.9048).abs() < 0.01);
    }

    #[test]
    fn test_radiation_half_life() {
        let module = PhysicsModule;
        // lambda=0.1 → T½ = ln(2)/0.1 ≈ 6.93
        let params = json!([0.1]);
        let result = module.execute("radiation::half_life", params).unwrap();
        let arr = result.as_array().unwrap();

        let half_life = arr[0].as_f64().unwrap();
        let mean_lifetime = arr[1].as_f64().unwrap();

        assert!((half_life - 6.93).abs() < 0.1);
        assert!((mean_lifetime - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_radiation_activity() {
        let module = PhysicsModule;
        // lambda=0.1, N=1000 → A=100 Bq
        let params = json!([0.1, 1000.0]);
        let result = module.execute("radiation::activity", params).unwrap();
        let arr = result.as_array().unwrap();

        let bq = arr[0].as_f64().unwrap();
        let ci = arr[1].as_f64().unwrap();

        assert!((bq - 100.0).abs() < 0.1);
        assert!(ci < 1.0e-8); // very small in Curies
    }

    #[test]
    fn test_radiation_chain_reaction() {
        let module = PhysicsModule;
        let params = json!([1.0, 2.5, 5, 0.3]);
        let result = module.execute("radiation::chain_reaction", params).unwrap();
        let arr = result.as_array().unwrap();

        assert!(arr.len() >= 5); // steps + 1
        // Each step should have 3 elements
        for step in arr {
            let s = step.as_array().unwrap();
            assert_eq!(s.len(), 3);
        }
    }

    #[test]
    fn test_radiation_fission_energy() {
        let module = PhysicsModule;
        // ~200 MeV per fission
        let params = json!([3.56e-28]);
        let result = module.execute("radiation::fission_energy", params).unwrap();
        let arr = result.as_array().unwrap();

        let mev = arr[1].as_f64().unwrap();
        assert!((mev - 200.0).abs() < 5.0);
    }

    #[test]
    fn test_radiation_fusion_energy() {
        let module = PhysicsModule;
        // D-T fusion: 17.6 MeV per reaction
        let params = json!([1.0]);
        let result = module.execute("radiation::fusion_energy", params).unwrap();
        let arr = result.as_array().unwrap();

        let mev = arr[1].as_f64().unwrap();
        assert!((mev - 17.6).abs() < 0.1);
    }

    #[test]
    fn test_radiation_geiger_rate() {
        let module = PhysicsModule;
        let params = json!([50.0, 0.5, 1.0]);
        let result = module.execute("radiation::geiger_rate", params).unwrap();
        let arr = result.as_array().unwrap();

        let rate = arr[0].as_f64().unwrap();
        let cpm = arr[1].as_f64().unwrap();
        let dangerous = arr[2].as_bool().unwrap();

        assert!((rate - 50.5).abs() < 0.1);
        assert!((cpm - 3030.0).abs() < 1.0);
        assert!(!dangerous); // 50 Bq/m³ < 100 threshold
    }

    #[test]
    fn test_radiation_geiger_dangerous() {
        let module = PhysicsModule;
        let params = json!([200.0, 0.5, 1.0]);
        let result = module.execute("radiation::geiger_rate", params).unwrap();
        let arr = result.as_array().unwrap();

        let dangerous = arr[2].as_bool().unwrap();
        assert!(dangerous); // 200 > 100
    }

    #[test]
    fn test_radiation_exposure_safe() {
        let module = PhysicsModule;
        // 0.0005 J/kg gamma → 0.5 mSv
        let params = json!([0.0005, 3]);
        let result = module.execute("radiation::exposure", params).unwrap();
        let arr = result.as_array().unwrap();

        let msv = arr[1].as_f64().unwrap();
        let risk = arr[2].as_str().unwrap();

        assert!((msv - 0.5).abs() < 0.01);
        assert_eq!(risk, "safe");
    }

    #[test]
    fn test_radiation_exposure_alpha_dangerous() {
        let module = PhysicsModule;
        // 0.001 J/kg alpha → 20 mSv (alpha has wr=20)
        let params = json!([0.001, 1]);
        let result = module.execute("radiation::exposure", params).unwrap();
        let arr = result.as_array().unwrap();

        let msv = arr[1].as_f64().unwrap();
        let risk = arr[2].as_str().unwrap();

        assert!((msv - 20.0).abs() < 0.1);
        assert_eq!(risk, "moderate");
    }

    #[test]
    fn test_physics_register_with_radiation() {
        let module = PhysicsModule;
        let cmds = module.register();
        assert!(cmds.contains_key("radiation::decay"));
        assert!(cmds.contains_key("radiation::half_life"));
        assert!(cmds.contains_key("radiation::chain_reaction"));
        assert!(cmds.contains_key("radiation::fission_energy"));
        assert!(cmds.contains_key("radiation::fusion_energy"));
        assert!(cmds.contains_key("radiation::geiger_rate"));
        assert!(cmds.contains_key("radiation::exposure"));
        assert_eq!(cmds.len(), 23);
        assert!(cmds.contains_key("kepler::orbit"));
        assert!(cmds.contains_key("kepler::period"));
        assert!(cmds.contains_key("kepler::velocity"));
        assert!(cmds.contains_key("kepler::ellipse_points"));
        assert!(cmds.contains_key("solar_system::step"));
    }

    #[test]
    fn test_kepler_circular_orbit() {
        let module = PhysicsModule;
        // Órbita circular: e=0, a=1, period=1, t=0
        let params = json!([1.0, 0.0, 1.0, 0.0]);
        let result = module.execute("kepler::orbit", params).unwrap();
        let arr = result.as_array().unwrap();
        // En t=0, debe estar en (1, 0)
        assert!((arr[0].as_f64().unwrap() - 1.0).abs() < 0.01);
        assert!(arr[1].as_f64().unwrap().abs() < 0.01);
    }

    #[test]
    fn test_kepler_ellipse_points() {
        let module = PhysicsModule;
        let params = json!([100.0, 0.5, 400.0, 300.0, 32]);
        let result = module.execute("kepler::ellipse_points", params).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj["count"].as_i64().unwrap(), 32);
        assert!((obj["semi_major"].as_f64().unwrap() - 100.0).abs() < 0.01);
        let points = obj["points"].as_array().unwrap();
        assert_eq!(points.len(), 32);
    }

    #[test]
    fn test_kepler_period() {
        let module = PhysicsModule;
        // T² = 4π²a³/(GM), con G=1, M=1, a=1 → T = 2π
        let params = json!([1.0, 1.0, 1.0]);
        let result = module.execute("kepler::period", params).unwrap();
        let arr = result.as_array().unwrap();
        assert!((arr[0].as_f64().unwrap() - 2.0 * std::f64::consts::PI).abs() < 0.01);
    }
}
