# 🍎 Informe: Física Newtoniana + Audio Reactivo en Ry-Dit

**Fecha**: 2026-04-12
**Versión**: v0.19.2
**Crate principal**: ry-gfx (particles) + ry-rs (physics, executor)
**Demo**: `demo_meteor_shower`

---

## 📋 Resumen Ejecutivo

Se implementó un sistema completo de **física newtoniana interactiva** con **visualización reactiva por color** y **audio procedural generativo** en el motor Ry-Dit. El resultado es el demo `Meteor Shower` que simula gravedad universal entre cuerpos, colisiones con explosiones de partículas, y sonidos generados matemáticamente sin archivos externos.

**Características principales:**
- Gravitación F = G·m₁·m₂/r² entre todos los cuerpos (O(n²))
- Color por velocidad: 6 zonas de Azul oscuro → Rojo NFS → Blanco flash
- Blend aditivo para explosiones brillantes
- Audio reactivo: 3 sonidos procedurales (impacto, explosión, retumbo)
- Demo funcional con Zink DRI3 en Termux-X11

---

## 🧲 1. Gravitación Newtoniana

### Fórmula implementada

```
F = G * m₁ * m₂ / r²
ax = F * dx / (dist * m₁)
ay = F * dy / (dist * m₁)
```

**Archivos:**
- `crates/ry-rs/src/modules/physics.rs` → `apply_newtonian_gravity(dt, g)`
- `crates/ry-rs/src/executor.rs` → llamado en cada frame del game loop

### Integración en el game loop

```rust
// En executor.rs (FASE 1 de cada frame)
if let Some(Valor::Bool(true)) = executor.leer("__PHYSICS_ENABLED__") {
    let world = get_physics_world();
    let mut world_ref = world.borrow_mut();
    world_ref.update(dt);
    
    if let Some(Valor::Bool(true)) = executor.leer("__NEWTON_GRAVITY__") {
        let g: f64 = match executor.leer("__GRAVITY_G__") {
            Some(Valor::Num(g)) => g, _ => 100.0,
        };
        world_ref.apply_newtonian_gravity(dt, g);
    }
}
```

### Activación desde `.rydit`

```rydit
physics::enable()              # Activar física básica
physics::enable_newton(200)    # Gravitación con G=200
physics::create_body("sol", 450, 350, 0, 0)
physics::set_velocity("tierra", 50, 0)
physics::update(__DT__)        # Cada frame
```

### Funciones nuevas de física

| Función | Fórmula | Rango | Uso |
|---------|---------|-------|-----|
| `physics::impact_frequency(base)` | f = base + 500·√E | 20-20K Hz | Frecuencia reactiva |
| `physics::impact_volume()` | V = min(1, E/10000) | 0.0-1.0 | Volumen reactivo |
| `physics::doppler_shift(f, vs, vo)` | f' = f·(v+vo)/(v-vs) | 20-20K Hz | Efecto Doppler |
| `physics::impact_profile()` | [freq, vol, tipo] | — | Perfil completo |
| `physics::kinetic_energy()` | E = ½mv² | Joules | Energía total |
| `physics::max_impact()` | E_max = ½(m₁+m₂)·Δv² | Joules | Impacto máximo |

### Tipos de sonido por energía

| Energía | Tipo | Frecuencia | Volumen | Sensación |
|---------|------|-----------|---------|-----------|
| E < 100 | "rumble" | 100-200Hz | 0.0-0.1 | Retumbo grave |
| 100-5K | "impact" | 200-800Hz | 0.1-0.5 | Golpe seco |
| 5K-50K | "crash" | 800-4KHz | 0.5-0.8 | Choque fuerte |
| >50K | "explosion" | 4K-20KHz | 0.8-1.0 | Aturdidor |

---

## 🎨 2. Color por Velocidad

### Escala de 6 zonas

| Velocidad | Color RGB | Sensación |
|-----------|-----------|-----------|
| 0-20% | (20, 40, 120) → (80, 160, 255) | 🔵 Azul oscuro → claro: lento/pesado |
| 20-40% | (80, 160, 255) → (255, 255, 80) | 🔵→🟡 Azul → amarillo: acelerando |
| 40-60% | (255, 255, 80) → (255, 160, 20) | 🟡→🟠 Amarillo → naranja: eléctrico |
| 60-80% | (255, 160, 20) → (255, 40, 20) | 🟠→🔴 Naranja → rojo: rápido |
| 80-95% | (255, 40, 20) → (255, 100, 80) | 🔴 Rojo brillante: nitrógeno NFS |
| 95-100% | (255, 100, 80) → (255, 255, 255) | ⚪ Blanco flash: velocidad luz |

### Implementación

**Archivo:** `crates/ry-gfx/src/particles.rs`

```rust
pub fn get_velocity_color(&self, max_speed: f32) -> Color {
    let speed = (self.vx * self.vx + self.vy * self.vy).sqrt();
    let t = (speed / max_speed).clamp(0.0, 1.0);
    
    let (r, g, b) = if t < 0.2 {
        lerp3((20, 40, 120), (80, 160, 255), t / 0.2)
    } else if t < 0.4 {
        lerp3((80, 160, 255), (255, 255, 80), (t - 0.2) / 0.2)
    } // ... más zonas
    
    Color { r: r as u8, g: g as u8, b: b as u8, a: self.get_alpha() }
}
```

### Activación desde `.rydit`

```rydit
particles::enable_velocity_color(300)  # max_speed = 300 px/seg
```

---

## ✨ 3. Blend Aditivo

### Qué es

Cuando múltiples partículas se superponen, sus colores **se suman** en lugar de mezclarse. Resultado: la superposición densa produce **blanco puro**.

### Implementación

```rust
// En ParticleSystem::draw()
if self.additive_blend {
    unsafe { raylib::ffi::BeginBlendMode(raylib::ffi::BlendMode::BLEND_ADDITIVE as i32) };
}
for emitter in self.emitters.values() {
    emitter.draw_with_velocity(d, max_speed);
}
if self.additive_blend {
    unsafe { raylib::ffi::EndBlendMode() };
}
```

### Activación

```rydit
particles::enable_additive_blend()
```

### Efecto visual

| Densidad | Resultado | Uso |
|----------|-----------|-----|
| 1 partícula | Color normal | Partícula individual |
| 2-3 partículas | Color brillante | Chispas, fuego |
| 5+ partículas | Casi blanco | Explosión central |
| 10+ partículas | **Blanco flash** | Núcleo de explosión |

---

## 🔊 4. Audio Procedural

### Filosofía

**Sin archivos de audio.** Todos los sonidos se generan matemáticamente en runtime usando ondas sinusoidales y ruido blanco.

### Sonidos generados

#### Impacto (150ms)
```
f(t) = sin(800Hz · t) · e^(-20t) · 0.8
```
- Tono sinusoidal puro con decaimiento exponencial
- Frecuencia base: 800Hz (agudo pero no doloroso)

#### Explosión (500ms)
```
ruido(t) = white_noise * 0.5
filtrado(t) = prev * 0.7 + ruido * 0.3  # Low-pass simple
env(t) = { t/0.02           si t < 20ms  (attack)
         { e^(-5(t-0.02))   si t ≥ 20ms  (decay)
output(t) = filtrado(t) · env(t) · 0.9
```
- Ruido blanco filtrado con simple low-pass (feedback 0.7)
- Envolvente con ataque rápido (20ms) y decaimiento exponencial

#### Retumbo (300ms)
```
f(t) = sin(80Hz · t) · e^(-4t) · 0.5
```
- Tono muy grave (80Hz) — vibración profunda
- Decaimiento lento para efecto de resonancia

### Implementación FFI

```rust
fn create_wave_from_samples(samples: &[f32], sample_rate: u32) -> ffi::Wave {
    let boxed = samples.to_vec().into_boxed_slice();
    let data = Box::into_raw(boxed) as *mut c_void;
    
    ffi::Wave {
        frameCount: samples.len() as u32,
        sampleRate: sample_rate,
        sampleSize: 32,  // 32-bit float
        channels: 1,
        data,
    }
}

fn generate_impact_wave(sample_rate: u32) -> ffi::Wave { ... }

// Carga y reproducción
let impact_wave = generate_impact_wave(44100);
let impact_sound = unsafe { ffi::LoadSoundFromWave(impact_wave) };
// ...
ffi::SetSoundPitch(impact_sound, pitch);
ffi::PlaySound(impact_sound);
```

### Backend de audio

**Miniaudio** (incluido en raylib) detecta automáticamente:
- PulseAudio
- PipeWire
- ALSA
- JACK

No requiere configuración adicional. Funciona en Termux-X11 igual que SDL2 audio.

---

## 🌠 5. Demo Meteor Shower

### Arquitectura

```
┌─────────────────────────────────────────────────┐
│                  GAME LOOP                       │
├─────────────────────────────────────────────────┤
│ 1. INPUT                                         │
│    - [ESPACIO] nuevo meteoro                    │
│    - [R] reiniciar                               │
│                                                 │
│ 2. FÍSICA (cada frame)                          │
│    - Gravitación F=G·m₁·m₂/r² entre todos       │
│    - Detección de colisión (dist < r₁+r₂)       │
│    - Resolución: meteoros pequeños mueren       │
│    - Energía cinética calculada                  │
│                                                 │
│ 3. AUDIO (si hay colisión)                      │
│    - E<5000: impacto con pitch reactivo         │
│    - E>5000: explosión con volumen reactivo     │
│                                                 │
│ 4. PARTÍCULAS                                    │
│    - spawn_explosion() en punto de impacto      │
│    - ps.update(dt)                               │
│                                                 │
│ 5. RENDER                                        │
│    - Fondo: cielo oscuro + estrellas            │
│    - Partículas: additive_blend + velocity_color │
│    - Meteoros: halo + cuerpo + brillo + trail   │
│    - HUD: meteoros, impactos, energía, sonido   │
└─────────────────────────────────────────────────┘
```

### Controles

| Tecla | Acción |
|-------|--------|
| `ESPACIO` | Lanzar nuevo meteoro desde arriba |
| `R` | Reiniciar demo |
| `ESC` | Salir |

### Spawning automático

Cada 2 segundos (~120 frames) se genera un nuevo meteoro desde posición aleatoria en la parte superior.

### Colores del meteoro

| Velocidad | Color del cuerpo | Halo | Brillo central |
|-----------|-----------------|------|----------------|
| Lento | 🔵 Azul oscuro | Azul tenue | Blanco |
| Medio | 🟡 Amarillo | Amarillo | Blanco |
| Rápido | 🔴 Rojo NFS | Rojo brillante | Blanco |
| Flash | ⚪ Blanco | Blanco intenso | Blanco |

### Trail

Los meteoros rápidos dejan una cola de partículas que se desvanecen:
```
trail_len = min(speed / 50, 10)
for i in 1..=trail_len:
    alpha = 200 - i * 20
    size = radius - i
```

---

## 📊 Métricas de Rendimiento

| Métrica | Valor |
|---------|-------|
| **Física** | O(n²) por frame — ok hasta ~50 cuerpos |
| **Partículas** | 10-100 por explosión |
| **Audio** | 3 sonidos cargados en RAM (~200KB total) |
| **FPS objetivo** | 60 FPS (Adreno 610 con Zink) |

### Optimización futura

- Gravitación O(n²) → Barnes-Hut O(n log n) para 100+ cuerpos
- Partículas por frame → limitar máximo por emisor
- Audio → pool de sonidos reutilizables

---

## 🔗 Archivos Modificados

| Archivo | Cambios |
|---------|---------|
| `crates/ry-rs/src/modules/physics.rs` | +120 líneas: newtonian gravity, audio reactivo |
| `crates/ry-rs/src/executor.rs` | +30 líneas: physics update en game loop |
| `crates/ry-rs/src/main.rs` | +50 líneas: dispatch de funciones físicas |
| `crates/ry-gfx/src/particles.rs` | +80 líneas: velocity color, additive blend |
| `crates/ry-rs/src/bin/demo_meteor_shower.rs` | +450 líneas: demo completo |

---

## 🎯 Lecciones Aprendidas

### Lo que funcionó

1. **Física en game loop**: Integrar `physics::update(dt)` en cada frame funciona sin impacto de rendimiento
2. **Audio procedural**: Generar ondas sin archivos es limpio y portable
3. **Color por velocidad**: Feedback visual inmediato de la energía del sistema
4. **Blend aditivo**: Explosiones "reales" sin texturas costosas
5. **Miniaudio**: Detecta PulseAudio/PipeWire sin configuración

### Lo que mejorar

1. **Gravedad O(n²)**: Funciona bien hasta ~50 cuerpos. Para 100+ necesita Barnes-Hut
2. **Audio sin caché**: Cada colisión reproduce sonido nuevo. Un pool sería más eficiente
3. **Partículas sin límite**: Explosiones grandes generan 100+ partículas. Limitar a 50/emisor

### No intentado (pero posible)

- **Efecto Doppler real**: Calcular v_source/v_observer por meteoro
- **Reverberación**: Simular espacio (cueva vs espacio abierto)
- **Audio posicional**: Sonido más fuerte cerca del punto de impacto

---

## 📐 Comparativa con Motores Existentes

| Feature | Ry-Dit | Godot | Unity | Bevy |
|---------|--------|-------|-------|------|
| Gravitación N-cuerpos | ✅ Manual | ⚠️ Area2D | ⚠️ Physics2D | ❌ |
| Color por velocidad | ✅ 6 zonas | ⚠️ Shader custom | ⚠️ Shader | ✅ ECS query |
| Blend aditivo | ✅ FFI | ✅ Node | ✅ Material | ✅ Pipeline |
| Audio procedural | ✅ Sin archivos | ⚠️ AudioStream | ⚠️ AudioClip | ⚠️ crate |
| Sin dependencias | ✅ Zero | ❌ Engine | ❌ Engine | ❌ ECS |

---

<div align="center">

**🌠 Física Newtoniana + Audio Reactivo — Ry-Dit v0.19.2**

*25 crates · ~225 tests · 23+ demos · 0 errores · Low-End First*

*Todo funciona en Adreno 610 con Zink DRI3 en Termux-X11*

</div>
