# 🛡️ RyDit v0.11.6 - Guía de Usuario

**Última actualización**: 2026-04-03
**Versión**: v0.11.6
**Estado**: ✅ Input + TTF + Sprites + Rigid Body + Audio — Todo funcional

---

## 🆕 ¿Qué hay de nuevo en v0.11.6?

La versión v0.11.6 es la primera versión completamente funcional con el stack gráfico SDL2:

| Feature | Descripción |
|---------|-------------|
| **Input SDL2** | Teclado funcional con patrón `repeat: false` (cada pulsación = acción individual) |
| **Texto TTF** | Texto real con fuente del sistema (`DroidSans.ttf`), sin parpadeo, texturas cacheadas |
| **Sprites PNG** | 4 sprites cargados con SDL2_image: tank, helicopter, crate, platform |
| **Rigid Body** | 4 cuerpos con gravedad independiente + colisiones AABB + empuje del jugador |
| **Audio SDL2** | Sonidos generados dinámicamente (salto 600Hz, colisión 300Hz) con SDL2_mixer |

---

## 🎮 Controles

Todos los demos usan el mismo patrón de input. Las teclas funcionan en el teclado virtual de Android (Termux-X11) y en teclado físico.

| Tecla | Acción |
|-------|--------|
| **← / A** | Mover izquierda |
| **→ / D** | Mover derecha |
| **↑ / W** | Mover arriba / Saltar |
| **↓ / S** | Mover abajo |
| **SPACE** | Saltar (en platformer) |
| **R** | Reset (reiniciar posiciones) |
| **G** | Toggle (activar/desactivar feature) |
| **ESC** | Salir del demo |

> **Nota importante**: El teclado virtual de Android solo envía pulsaciones individuales (`repeat: false`). Cada toque = una acción. Mantener presionado NO produce movimiento continuo.

---

## 📦 Demos disponibles

### 1. demo_colisiones — Input básico + colisiones

**Qué verás**: Un cuadro rojo que controlas sobre 6 plataformas grises. Gravedad + salto.

| Elemento | Color | Función |
|----------|-------|---------|
| Jugador | 🔴 Rojo | Controlable con ← → SPACE |
| Plataformas | ⬜ Gris | Colisiones AABB |
| Fondo | ⬛ Negro | Sin decoración |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin demo_colisiones --release
DISPLAY=:0 ./target/release/demo_colisiones
```

**Qué probar**:
- Toca ← → para mover el cuadro
- Toca SPACE para saltar
- Observa cómo el cuadro cae por gravedad y aterriza en las plataformas

---

### 2. demo_rigidbody — Demo completo (⭐ recomendado)

**Qué verás**: Un cuadro rojo (jugador) + 4 sprites con físicas independientes + texto TTF en tiempo real mostrando FPS, posiciones y estado.

| Elemento | Color/Sprite | Función |
|----------|-------------|---------|
| Jugador | 🔴 Rojo | Controlable con ← → ↑ ↓ WASD SPACE |
| Tank | 🟢 Verde | Sprite PNG con gravedad |
| Helicopter | 🔵 Cyan | Sprite PNG con gravedad |
| Crate | 🟤 Marrón | Sprite PNG con gravedad |
| Platform | ⚫ Gris | Sprite PNG con gravedad |
| Plataformas | ⬜ Gris | 7 plataformas estáticas |
| Texto TTF | ⬜ Blanco | Info en tiempo real (FPS, posiciones) |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin demo_rigidbody --release
DISPLAY=:0 ./target/release/demo_rigidbody
```

**Qué probar**:
- Mueve el jugador con ← → ↑ ↓
- Salta con SPACE
- Empuja los sprites: el jugador transfiere velocidad a los rigid bodies
- Observa cómo cada sprite cae, rebota y aterriza independientemente
- Lee el texto TTF en la esquina superior con info en tiempo real

---

### 3. test_audio_minimal — Sonidos SDL2_mixer

**Qué verás**: Una pantalla que genera archivos WAV con tonos puros y los reproduce.

| Sonido | Frecuencia | Cuándo suena |
|--------|-----------|--------------|
| Salto | 600 Hz | Al presionar SPACE |
| Colisión | 300 Hz | Al chocar con plataforma |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin test_audio_minimal --release
DISPLAY=:0 ./target/release/test_audio_minimal
```

**Qué probar**:
- Presiona SPACE para escuchar el tono de salto (600Hz)
- Observa la pantalla que confirma la reproducción del sonido

---

### 4. demo_50k_particulas — Sistema de partículas masivo

**Qué verás**: Miles de partículas animadas con diferentes efectos (fuego, humo, explosión, lluvia, chispas).

| Tecla | Efecto |
|-------|--------|
| **F** | Fuego |
| **H** | Humo |
| **E** | Explosión |
| **S** | Chispas |
| **L** | Lluvia |

**Cómo ejecutar**:
```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

cargo build -p rydit-rs --bin demo_50k_particulas --release
DISPLAY=:0 ./target/release/demo_50k_particulas
```

**Qué probar**:
- Presiona F para fuego, H para humo, E para explosión
- Observa el rendimiento con miles de partículas simultáneas

---

## 🖼️ Sprites disponibles

Todos los sprites están en `logo_icon_asst/sprites/`:

| # | Nombre | Archivo | Tamaño | Color representativo |
|---|--------|---------|--------|---------------------|
| 1 | **Tank** | `tank_16x16.png` | 16×16 | 🟢 Verde |
| 2 | **Helicopter** | `helicopter_16x16.png` | 16×16 | 🔵 Cyan |
| 3 | **Crate** | `crate_8x8.png` | 8×8 | 🟤 Marrón |
| 4 | **Platform** | `platform_16x16.png` | 16×16 | ⚫ Gris |
| 5 | **Cube** | `cube_8x8.png` | 8×8 | — |

Los sprites se cargan con `SDL2_image` y se dibujan como texturas SDL2. En `demo_rigidbody` los 4 primeros se ven como objetos con gravedad que rebotan y aterrizan.

---

## 📝 Texto TTF

El texto TTF se renderiza usando `FontFFI` (FFI interno de `rydit-gfx`), **no** usando `sdl2::ttf` directamente.

**Qué se ve en pantalla** (en `demo_rigidbody`):
```
═══════════════════════════════════════
  RyDit v0.11.6 - Demo Rigid Body
═══════════════════════════════════════
  FPS: 60
  Jugador: x=400 y=300
  Rigid Bodies: 4
  Plataformas: 7
  ← → ↑ ↓ WASD SPACE = Mover
  R = Reset | ESC = Salir
═══════════════════════════════════════
```

**Características del texto TTF**:
- ✅ Fuente real del sistema (`/system/fonts/DroidSans.ttf` en Android)
- ✅ Sin parpadeo (texturas cacheadas cada 30 frames)
- ✅ Múltiples tamaños (14px, 18px, 20px)
- ✅ Colores RGB (blanco, verde, amarillo)

**Cómo se carga** (patrón para tus propios demos):
```rust
let mut backend = Sdl2Backend::new("Mi Demo", 800, 600)?;

// Probar rutas de fuente
for path in &["/system/fonts/DroidSans.ttf", "/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
    if std::path::Path::new(path).exists() {
        let _ = backend.load_font(path, 18);
        break;
    }
}

// Dibujar texto
backend.draw_text("Título", 15, 15, 18, 255, 255, 255);
backend.draw_text(&format!("FPS: {}", fps), 15, 45, 14, 0, 255, 0);
```

---

## 🔊 Audio

El audio funciona con `SDL2_mixer` a través de `Sdl2Backend`. Los sonidos se generan dinámicamente como archivos WAV con tonos puros.

| Sonido | Frecuencia | Duración | Uso |
|--------|-----------|----------|-----|
| Salto | 600 Hz | 0.2s | Cuando el jugador salta |
| Colisión | 300 Hz | 0.15s | Cuando hay impacto con plataforma |

**Cómo se genera un sonido**:
```rust
// Generar WAV con tono puro
fn generar_wav(ruta: &str, frecuencia: f32, duracion: f32) {
    let sample_rate = 44100u32;
    let num_samples = (sample_rate as f32 * duracion) as usize;
    let mut datos = Vec::with_capacity(num_samples * 2);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let valor = (2.0 * std::f32::consts::PI * frecuencia * t).sin();
        let sample = (valor * 16000.0) as i16;
        datos.extend_from_slice(&sample.to_le_bytes());
    }

    // Escribir header WAV + datos...
}

// Reproducir con SDL2_mixer
let chunk = Mix_Chunk::from_file("sonido_salto.wav")?;
Mix_PlayChannel(-1, &chunk, 0)?;
```

---

## 💥 Colisiones

Las colisiones usan el algoritmo **AABB** (Axis-Aligned Bounding Box) con `Rect::has_intersection()`.

### Cómo funcionan

1. **Cada frame** se calcula la nueva posición del jugador/rigid body
2. **Se verifica** si el rect del objeto intersecta con alguna plataforma
3. **Si hay intersección** y el objeto cae (`vy > 0`):
   - Se reposiciona el objeto encima de la plataforma
   - Se setea `vy = 0` y `en_suelo = true`
4. **Jugador ↔ Rigid Body**: Si intersectan, se transfiere velocidad (empuje)

```rust
// Colisión con plataforma
for plat in plataformas {
    if self.rect().has_intersection(*plat) {
        if self.rect().bottom() as i32 <= plat.y + 10 && self.vy > 0.0 {
            self.y = plat.y as f32 - self.h as f32;
            self.vy = 0.0;
            self.en_suelo = true;
        }
    }
}

// Colisión jugador ↔ rigid body (empuje)
if self.rect().has_intersection(jugador_rect) {
    if jugador_rect.y < self.rect().y as i32 {
        self.vy = -200.0;  // Empujar arriba
    }
    if jugador_rect.x < self.rect().x as i32 {
        self.vx -= 100.0;  // Empujar izquierda
    } else {
        self.vx += 100.0;  // Empujar derecha
    }
}
```

### En demo_rigidbody

- **4 rigid bodies** con gravedad independiente
- **7 plataformas** estáticas
- **Jugador** empuja rigid bodies al colisionar
- **Cada rigid body** también colisiona con plataformas

---

## 📋 Requisitos

### Software necesario

| Dependencia | Versión | Cómo verificar |
|-------------|---------|----------------|
| **SDL2** | 0.37 | `pkg list-installed \| grep sdl2` |
| **SDL2_image** | — | `ls /data/data/com.termux/files/usr/lib/libSDL2_image.so` |
| **SDL2_ttf** | — | `ls /data/data/com.termux/files/usr/lib/libSDL2_ttf.so` |
| **SDL2_mixer** | — | `ls /data/data/com.termux/files/usr/lib/libSDL2_mixer.so` |
| **Rust** | 1.70+ | `rustc --version` |
| **Termux-X11** | — | Ejecutar `xinit` |

### Instalar dependencias

```bash
pkg install sdl2 sdl2_image sdl2_ttf sdl2_mixer
```

### Variables de entorno

```bash
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1
```

> **Importante**: Termux-X11 debe estar corriendo antes de ejecutar cualquier demo. En una sesión aparte ejecuta `xinit`.

---

## 🎬 Videos Demo

Los 3 videos están en `screenshots/` y son reproducibles directamente en GitHub:

### Video 1: Rigid Body + TTF + Sprites
- **Archivo**: `screenshots/Rydit_demo1.mp4`
- **Qué muestra**: Input SDL2 + texto TTF + 4 sprites PNG + gravedad + colisiones + audio
- **Duración**: ~30 segundos

### Video 2: Test Audio SDL2_mixer
- **Archivo**: `screenshots/test_demo.mp4`
- **Qué muestra**: Generación de tonos WAV + reproducción con SDL2_mixer
- **Duración**: ~15 segundos

### Video 3: Sistema de Partículas
- **Archivo**: `screenshots/particulas.mp4`
- **Qué muestra**: 5 efectos (fuego, humo, explosión, lluvia, chispas) a 60 FPS
- **Duración**: ~30 segundos

---

## 🚀 Inicio rápido

```bash
# 1. Iniciar Termux-X11 (en otra sesión)
xinit

# 2. Configurar entorno
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink

# 3. Compilar y ejecutar el demo recomendado
cargo build -p rydit-rs --bin demo_rigidbody --release
DISPLAY=:0 ./target/release/demo_rigidbody
```

**Controles en pantalla**:
- ← → para mover el jugador
- ↑ o SPACE para saltar
- R para reset
- ESC para salir

---

## ⚠️ Solución de problemas

| Problema | Causa | Solución |
|----------|-------|----------|
| "no SDL2 video device" | Termux-X11 no iniciado | Ejecutar `xinit` primero |
| "failed to create window" | DISPLAY no configurado | `export DISPLAY=:0` |
| "undefined symbol: TTF_Init" | Usando `sdl2::ttf` directo | Usar `backend.draw_text()` |
| Input no responde | Usando `repeat: true` | Usar SOLO `repeat: false` |
| Sprites no aparecen | Ruta incorrecta | Verificar que el archivo existe en `logo_icon_asst/sprites/` |
| Audio no suena | SDL2_mixer no linkado | Usar `Sdl2Backend` que ya tiene mixer init |

---

<div align="center">

**🛡️ RyDit v0.11.6 - Guía de Usuario**

*Input SDL2 ✅ | TTF ✅ | Sprites PNG ✅ | Rigid Body ✅ | Audio ✅*

**Próximo: v0.11.7 — Audio.rs migrado a SDL2_mixer + Demo .rydit con audio**

</div>
