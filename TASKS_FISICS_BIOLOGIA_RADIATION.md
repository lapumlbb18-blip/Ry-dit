# 🧲 TASKS — Física, Biología, Radiación y Ciencia en Ry-Dit

**Fecha**: 2026-04-14
**Versión**: v0.20.0
**Contexto**: Mapa completo de físicas, biología, ciencia y radiación — implementadas y pendientes
**Visión**: Ry-Dit como simulador de escenas con físicas + animaciones Disney + procesos científicos/biológicos

---

## ✅ CONSOLIDACIÓN DE DUPLICACIONES — 2026-04-14

### Fuego/Humo consolidado en `ry-gfx/particles.rs`

| Cambio | Antes | Después | Razón |
|--------|-------|---------|-------|
| `fire(x,y)` | Duplicado con parámetros propios | → Wrapper de `fire_convection(x,y,0.35)` | Elimina duplicación, unifica API |
| `fire_convection(x,y,intensity)` | Solo versión avanzada | → Método principal (único) | intensity: 0.1=brasa → 1.2+=incendio |
| `smoke(x,y)` | Independiente | → Se mantiene independiente | Útil sin fuego (granadas, chimeneas, niebla) |
| Humo en fuego | Color end oscuro implícito | → Documentado como **fase final** de enfriamiento | Humo = parte natural del fuego, no duplicado |

### L-System implementado en `ry-science/src/lsystem.rs`

| Feature | Detalle | Tests |
|---------|---------|-------|
| `rewrite()` | Axioma + reglas iterativas → string final | ✅ 3 tests |
| `interpret()` | String → segmentos de dibujo con turtle graphics | ✅ 3 tests |
| 8 presets | binary_tree, bush, fern, coral, koch_plant, fractal_tree, dragon_curve, hilbert_curve | ✅ 8 tests |
| 4 commands | lsystem::rewrite, interpret, preset, presets | Integrados al ScienceModule |

**Tests totales**: 198 passing (crates no-gráficos). ry-gfx requiere DISPLAY para tests visuales.
**Compilación**: 0 errores.

---

## 📊 RESUMEN EJECUTIVO

| Categoría | Implementadas | Pendientes | Total | % Listo |
|-----------|--------------|-----------|-------|---------|
| **Física** | 6 | 10 | 16 | 38% |
| **Ciencia** | 10 | 8 | 18 | 56% |
| **Animación/Efectos** | 7 | 4 | 11 | 64% |
| **Biología** | 1 | 7 | 8 | 13% |
| **Radiación/Atómico** | 0 | 6 | 6 | 0% |
| **TOTAL** | **24** | **35** | **59** | **41%** |

---

## ✅ FÍSICA — Ya Implementada

### ry-physics (296 líneas)

| # | Feature | Fórmula | Dónde | Estado | Uso en Demo |
|---|---------|---------|-------|--------|-------------|
| 1 | **Proyectil 2D** | x = v₀cos(θ)t, y = v₀sin(θ)t - ½gt² | `physics::projectile` | ✅ | Granadas militar, torreta |
| 2 | **Gravitación N-cuerpos** | F = G·m₁·m₂/r², O(n²) | `physics::nbody_simulate` | ✅ | War Spacio, meteoros |
| 3 | **Gravedad 2 cuerpos** | F = G·m₁·m₂/r², fx/fy | `physics::nbody_2` | ✅ | Simulación orbital simple |
| 4 | **Color por velocidad** | 6 zonas: azul→amarillo→rojo→blanco | `ry-gfx/particles.rs` | ✅ | War Spacio, explosiones |
| 5 | **Blend aditivo** | Suma de colores en superposición | `ry-gfx/particles.rs` | ✅ | Explosiones brillantes |
| 6 | **Audio procedural** | Ondas seno + noise + exponencial | `ry-gfx/audio_*.rs` | ✅ | Impact, explosion, rumble |
| 7 | **Física en game loop** | Update dt cada frame | `rybot/physics_subsystem` | ✅ | War Spacio |
| 8 | **Energía cinética** | E = ½mv² | `ry-rs/physics.rs` | ✅ | Audio reactivo |
| 9 | **Fuego con convección** | Partículas suben + turbulencia térmica | `ry-gfx/particles.rs` | ✅ | Antorcha, hoguera, incendio |

### sdl2_helpers (ry-gfx)

| # | Feature | Detalle | Estado |
|---|---------|---------|--------|
| 9 | **velocity_color_sdl2** | Misma rampa 6 zonas para SDL2 | ✅ |
| 10 | **blend_additive_sdl2** | BlendMode::Add en SDL2 | ✅ |
| 11 | **newtonian_gravity_2d** | Gravitación frame a frame | ✅ |
| 12 | **audio_procedural_sdl2** | Shoot, explosion, powerup | ✅ |

---

## ✅ CIENCIA — Ya Implementada

### ry-science (531 + 457 líneas)

| # | Feature | Fórmula | Dónde | Estado |
|---|---------|---------|-------|--------|
| S1 | **Bezier lineal** | P(t) = (1-t)P₀ + tP₁ | `science::bezier::linear` | ✅ |
| S2 | **Bezier cuadrática** | (1-t)²P₀ + 2(1-t)tP₁ + t²P₂ | `science::bezier::quadratic` | ✅ |
| S3 | **Bezier cúbica** | (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃ | `science::bezier::cubic` | ✅ |
| S4 | **Media aritmética** | x̄ = Σx/n | `science::stats::mean` | ✅ |
| S5 | **Mediana** | Valor central ordenado | `science::stats::median` | ✅ |
| S6 | **Mínimo/Máximo** | min/max de array | `science::stats::min, max` | ✅ |
| S7 | **Geometría: Ilusiones ópticas** | Penrose, Cubo imposible, Espiral, Müller-Lyer, Ponzo | `science::geometry` | ✅ |
| S8 | **Simulaciones científicas** | 8 simulaciones en ry-science | ry-science demos | ✅ |
| S9 | **L-System (Lindenmayer)** | Axioma → reglas iterativas → árboles, helechos, coral | `science::lsystem` | ✅ v0.20.0 |

---

## ✅ ANIMACIÓN/EFFECTS — Ya Implementada

### ry-anim/effects.rs (300+ líneas)

| # | Feature | Detalle | Estado | Uso |
|---|---------|---------|--------|-----|
| A1 | **Neon Glow** | Capas de resplandor con pulso sinusoidal | ✅ | Personajes neón |
| A2 | **Motion Blur** | Estelas por posiciones anteriores | ✅ | Movimiento rápido |
| A3 | **Chromatic Aberration** | Separación RGB animada | ✅ | Efecto glitch/retro |
| A4 | **Bloom effect** | Halos de brillo por fuente de luz | ✅ | Explosiones, magia |
| A5 | **Particle Trails** | Estelas detrás de partículas | ✅ | Cometas, meteoros |
| A6 | **Morphing** | Interpolación entre formas con easing | ✅ | Transformaciones |
| A7 | **12 Disney principles** | Squash/stretch, anticipation, etc. | ✅ ry-anim core | Animación personajes |
| A8 | **Action assets** | Frame animation, sprite sheets, state machine | ✅ ry-anim | Sprites animados |
| A9 | **Transitions** | 19 transiciones (fade, slide, wipe, zoom, spiral) | ✅ ry-gfx | Cambio de escenas |
| A10 | **Fire presets** | fire(), fire_convection(), torch(), bonfire(), wildfire() | ✅ ry-gfx | Fuego unificado sin duplicación |

---

## ❌ FÍSICA — Evaluada pero NO Implementada

### Prioridad ALTA 🔴

| # | Feature | Fórmula | Impacto Visual | Esfuerzo | Demos que beneficia |
|---|---------|---------|----------------|----------|---------------------|
| F1 | **Colisiones elásticas** | p = m·v, conservación momentum: m₁v₁ + m₂v₂ = m₁v₁' + m₂v₂' | Choque de bolas, billar, vehículos rebotando | 6-8h | Billar, fútbol, pinball, asteroides, War Spacio |
| F2 | **Flocking (Reynolds)** | Cohesión + separación + alineación (3 reglas) | Bandadas de pájaros, cardúmenes, enjambres | 4-6h | Escenas naturaleza, zombies horda, naves enemigas |
| F3 | **Arrastre/fricción aire** | F_d = ½·ρ·v²·C_d·A | Paracaídas, hojas cayendo, plumas flotando | 3-4h | Paracaidista, tormenta, hojas de otoño |
| F4 | **Viento/turbulencia** | Vector fuerza variable con noise Perlin | Banderas ondeando, humo, polvo, arena | 3-4h | Desierto, tormenta de arena, humo de chimenea |

### Prioridad MEDIA 🟡

| # | Feature | Fórmula | Impacto Visual | Esfuerzo | Demos que beneficia |
|---|---------|---------|----------------|----------|---------------------|
| F5 | **Meteor shower completo** | Gravitación + fricción + explosión al impacto | Lluvia de meteoros con estela y explosión | 4-6h | Escena espacial,灾难, apocalipsis |
| F6 | **Choque de vehículos** | Colisión elástica + deformación + partículas | Car crash con chispas y escombros | 8-12h | Racing game, GTA-style, destrucción |
| F7 | ~~**Fuego con convección**~~ | ~~Partículas suben + viento térmico + turbulencia~~ | ~~Hoguera realista con humo~~ | ~~4-6h~~ | ~~Campamento, antorcha, incendio~~ | ✅ **Implementado v0.20.0** |
| F8 | **Explosión onda choque** | Onda expansiva esférica + escombros con gravedad | Explosión tipo bomba nuclear (onda + debris) | 4-6h | Acción, guerra, sci-fi |
| F9 | **Efecto Doppler real** | f' = f·(v±v_o)/(v±v_s) | Sirena que cambia tono al pasar | 3-4h | Carreras, ambulancia, tren |
| F10 | **Péndulo + resorte** | T = 2π√(L/g), F = -kx (Hooke) | Reloj de péndulo, resorte rebotando | 3-4h | Reloj, trampolín, juguete |

### Prioridad BAJA 🟢 (complejo o nicho)

| # | Feature | Fórmula | Impacto | Esfuerzo | Demos que beneficia |
|---|---------|---------|---------|----------|---------------------|
| F11 | **Agua SPH** | Smoothed Particle Hydrodynamics | Río, lluvia, olas realistas | 10-15h | Ocean game, simulador agua |
| F12 | **Barnes-Hut** | O(n log n) para N-cuerpos | 100+ cuerpos gravitacionales | 8-10h | Galaxia, sistema solar masivo |
| F13 | **Cuerdas/telas** | Verlet integration + constraints | Bandera, capa, puente colgante | 6-8h | Capa de superhéroe, tela |

---

## ❌ CIENCIA/BIOLOGÍA — Evaluada pero NO Implementada

### Prioridad ALTA 🔴

| # | Feature | Fórmula/Detalle | Impacto Visual | Esfuerzo | Demos que beneficia |
|---|---------|---------------|----------------|----------|---------------------|
| B1 | **Funciones trigonométricas animadas** | seno(t), coseno(t), tangente(t) como curvas en movimiento | Ondas, péndulos, osciladores, mareas | 3-4h | Péndulo, ola, radar, sonar, latido |
| B2 | ~~**L-System avanzado**~~ | ~~Lindenmayer: Axioma → Reglas de reescritura iterativa~~ | ~~Árboles, plantas, coral, helechos procedurales~~ | ~~6-8h~~ | ~~Bosque, jungla, jardín, naturaleza~~ | ✅ **Implementado v0.20.0** |
| B3 | **Autómatas celulares** | Game of Life (Conway), reglas custom 2D/3D | Patrones emergentes, vida artificial | 4-6h | Simulación biológica, arte generativo |
| B4 | **Fractales** | Mandelbrot: zₙ₊₁ = zₙ² + c, Julia, Koch, Sierpinski | Zoom infinito, patrones recursivos | 4-6h | Fondo psicodélico, transición, arte |
| B5 | **Sistema solar orbital** | Leyes de Kepler: T² ∝ a³, órbitas elípticas | Planetas orbitando sol con velocidades reales | 6-8h | Educativo, sci-fi, espacio |
| B6 | **Crecimiento poblacional** | Logistic map: xₙ₊₁ = rxₙ(1-xₙ), bifurcaciones | Caos vs estabilidad, explosión demográfica | 3-4h | Simulación ecología, epidemia |

### Prioridad MEDIA 🟡

| # | Feature | Fórmula/Detalle | Impacto Visual | Esfuerzo | Demos que beneficia |
|---|---------|---------------|----------------|----------|---------------------|
| B7 | **Reacción-difusión (Turing)** | ∂u/∂t = Dᵤ∇²u + f(u,v), ∂v/∂t = Dᵥ∇²v + g(u,v) | Manchas de leopardo, rayas de cebra, coral | 8-10h | Texturas animales, piel, piedra |
| B8 | **Ondas estacionarias** | Interferencia: y = 2A·sin(kx)·cos(ωt) | Cuerda de guitarra, agua en vaso | 4-6h | Música, física, experimento |
| B9 | **Diagramas de fase** | Transiciones: sólido ↔ líquido ↔ gas con calor | Hielo derritiéndose, agua hirviendo, vapor | 6-8h | Educativo, cocina, laboratorio |
| B10 | **Mareas gravitacionales** | F_marea = 2GMmR/d³ | Mareas oceánicas Luna-Tierra | 4-6h | Playa, navegación, astronomía |

---

## ☢️ RADIACIÓN — NUEVO (Atómico, Gen, Cinemático)

### Concepto: Radiación como sistema unificado

La **radiación** en Ry-Dit no es solo un efecto visual — es un **sistema de simulación** que conecta física atómica, genética y drama cinemático:

```
Núcleo atómico → Radiación → Mutación genética → Evolución → Drama cinemático
```

### Prioridad ALTA 🔴

| # | Feature | Fórmula/Detalle | Impacto Visual | Esfuerzo | Demos que beneficia |
|---|---------|---------------|----------------|----------|---------------------|
| R1 | **Decaimiento radiactivo** | N(t) = N₀·e^(-λt), vida media T½ = ln(2)/λ | Material brillante que se apaga con el tiempo, partículas alfa/beta/gamma | 4-6h | Central nuclear, Chernobyl, Fallout, sci-fi |
| R2 | **Fisión nuclear** | U-235 + n → Ba + Kr + 3n + energía | Reacción en cadena con flash + onda expansiva | 6-8h | Bomba atómica, reactor, drama histórico |
| R3 | **Fusión nuclear** | H + H → He + energía (estrella) | Sol brillando, explosión termonuclear | 6-8h | Estrella, supernova, sol del futuro |
| R4 | **Radiación Cherenkov** | Luz azul cuando partícula > velocidad luz en medio | Resplandor azul de reactor nuclear | 3-4h | Escena nuclear, underwater reactor |
| R5 | **Mutación genética** | ADN modificado por radiación: bases A,T,C,G alteradas | Criatura que muta visualmente (color, forma, tamaño) | 8-10h | Godzilla, mutants, sci-fi horror, evolución |
| R6 | **Drama cinemático por radiación** | Tensión dramática = f(energía, distancia, tiempo exposición) | Escena con niveles de peligro, urgencia, suspense | 4-6h | Chernobyl drama, thriller nuclear, película |

### Prioridad MEDIA 🟡

| # | Feature | Fórmula/Detalle | Impacto Visual | Esfuerzo | Demos que beneficia |
|---|---------|---------------|----------------|----------|---------------------|
| R7 | **Geiger counter audio** | Click rate ∝ radiación: f = f₀ + k·radiación | Sonido de contador Geiger reactivo | 3-4h | Detector, exploración, suspense |
| R8 | **Radiación electromagnética** | Espectro completo: radio → micro → IR → visible → UV → X → gamma | Visualización del espectro EM con colores | 4-6h | Educativo, telecomunicaciones, medicina |
| R9 | **Cadena de desintegración** | U-238 → Th-234 → Pa-234 → ... → Pb-206 (14 pasos) | Árbol de desintegración animado | 6-8h | Educativo, tabla periódica interactiva |
| R10 | **Evolución por radiación** | Radiación → mutación → selección natural → nueva especie | Timeline evolutivo acelerado | 8-10h | Documental, origen de la vida, futuro |

---

## 🎬 DEMOS PLANIFICADAS — Simulación por Categoría

### 🎮 JUEGOS Beneficiados

| Demo | Features Necesarias | Tipo | Descripción |
|------|--------------------|------|-------------|
| **demo_billar** | Colisiones elásticas, fricción | Juego | Mesa de billar con física real |
| **demo_futbol** | Colisiones elásticas, flocking (IA equipo) | Juego | Fútbol 2D con IA de equipo |
| **demo_asteroids** | Colisiones, proyectil, N-cuerpos | Juego | Asteroides que se rompen y atraen |
| **demo_zombie_horde** | Flocking, wind, fire | Juego | Zombies persiguiendo con fuego |
| **demo_racing** | Colisiones vehículos, Doppler | Juego | Carreras con choques y sonido Doppler |
| **demo_pinball** | Colisiones elásticas, resorte, gravedad | Juego | Pinball completo |
| **demo_galaxy** | N-cuerpos Barnes-Hut, sistema solar, fisión | Juego | Simulador galáctico |
| **demo_fallout** | Decaimiento, fisión, Geiger, mutación | Juego | Survival post-nuclear |
| **demo_godzilla** | Mutación genética, radiación, fusión | Juego | Monstruo que crece y muta |
| **demo_chernobyl** | Fisión, Cherenkov, drama cinemático, Geiger | Juego | Thriller histórico |

### 🎬 ANIMACIONES Beneficiadas

| Demo | Features Necesarias | Tipo | Descripción |
|------|--------------------|------|-------------|
| **demo_birds** | Flocking, wind, skybox | Animación | Bandada de pájaros al atardecer |
| **demo_forest** | L-System, fuego convección, wind | Animación | Bosque procedural con incendio |
| **demo_ocean** | SPH agua, mareas, ondas estacionarias | Animación | Océano con olas y mareas |
| **demo_solar_system** | Kepler, N-cuerpos, fusión (sol) | Animación | Sistema solar educativo |
| **demo_supernova** | Fusión, fisión, onda choque, bloom | Animación | Explosión de estrella |
| **demo_evolution** | Mutación genética, selección natural, radiación | Animación | Origen de la vida hasta humano |
| **demo_fractals** | Mandelbrot, Julia, color grading | Animación | Zoom infinito fractal con postfx |
| **demo_cherenkov** | Cherenkov blue glow, decaimiento | Animación | Reactor nuclear underwater |
| **demo_pendulum** | Péndulo, resorte, ondas, trig | Animación | Reloj + experimentos físicos |
| **demo_cellular_life** | Autómatas celulares + postfx bloom | Animación | Vida artificial que evoluciona |

### 🔬 CIENCIA/EDUCATIVO

| Demo | Features Necesarias | Tipo | Descripción |
|------|--------------------|------|-------------|
| **demo_periodic_table** | Decaimiento, cadena desintegración | Educativo | Tabla periódica interactiva |
| **demo_spectrum** | Radiación EM completa | Educativo | Espectro electromagnético visual |
| **demo_turing_patterns** | Reacción-difusión | Educativo | Manchas animales generadas |
| **demo_chaos** | Logistic map, bifurcaciones | Educativo | Teoría del caos visual |
| **demo_phase_diagram** | Sólido/líquido/gas | Educativo | Cambios de fase del agua |

---

## 📋 PLAN DE IMPLEMENTACIÓN — Fases

### Fase 1: Fundamentos (15-20h) — Semanas 1-2

| # | Feature | Crate | Conecta con |
|---|---------|-------|-------------|
| 1 | **Colisiones elásticas** | ry-physics | War Spacio, demo_billar |
| 2 | **Flocking (Reynolds)** | ry-physics | demo_birds, demo_zombie_horde |
| 3 | **Fractales (Mandelbrot)** | ry-science | demo_fractals, postfx-ry |
| 4 | **Autómatas celulares** | ry-science | demo_cellular_life |
| 5 | **Funciones trig animadas** | ry-science | demo_pendulum |

### Fase 2: Naturaleza y Espacio (20-25h) — Semanas 3-4

| # | Feature | Crate | Conecta con |
|---|---------|-------|-------------|
| 6 | ~~**L-System avanzado**~~ | ~~ry-science~~ | ~~demo_forest, ry3d-gfx~~ | ✅ **Implementado v0.20.0** — 8 presets + 4 commands + 14 tests |
| 7 | **Sistema solar orbital** | ry-physics | demo_solar_system |
| 8 | **Fuego con convección** | ry-gfx/particles | demo_forest, antorcha |
| 9 | **Arrastre/fricción aire** | ry-physics | demo_parachute |
| 10 | **Viento/turbulencia** | ry-physics | demo_forest, banderas |

### Fase 3: Radiación Atómica (25-35h) — Semanas 5-7

| # | Feature | Crate | Conecta con |
|---|---------|-------|-------------|
| 11 | **Decaimiento radiactivo** | ry-physics (nuevo: radiation) | demo_chernobyl, demo_fallout |
| 12 | **Fisión nuclear** | ry-physics | demo_fallout, demo_supernova |
| 13 | **Radiación Cherenkov** | postfx-ry + ry-gfx | demo_cherenkov |
| 14 | **Geiger counter audio** | ry-gfx/audio | demo_chernobyl |
| 15 | **Fusión nuclear** | ry-physics | demo_supernova, demo_solar_system |

### Fase 4: Biología + Genética (20-30h) — Semanas 8-10

| # | Feature | Crate | Conecta con |
|---|---------|-------|-------------|
| 16 | **Mutación genética** | ry-science (nuevo: genetics) | demo_godzilla, demo_evolution |
| 17 | **Reacción-difusión (Turing)** | ry-science | demo_turing_patterns |
| 18 | **Crecimiento poblacional** | ry-science | demo_chaos |
| 19 | **Evolución por radiación** | ry-science + ry-physics | demo_evolution |
| 20 | **Diagramas de fase** | ry-science | demo_phase_diagram |

### Fase 5: Drama Cinemático (15-20h) — Semanas 11-12

| # | Feature | Crate | Conecta con |
|---|---------|-------|-------------|
| 21 | **Drama cinemático por radiación** | rybot (nuevo subsystem: drama) | demo_chernobyl |
| 22 | **Ondas estacionarias** | ry-science | demo_pendulum |
| 23 | **Cadena de desintegración** | ry-science | demo_periodic_table |
| 24 | **Espectro electromagnético** | ry-science | demo_spectrum |
| 25 | **Meteor shower completo** | ry-physics + ry-gfx | demo_meteors |

---

## 📊 MATRIZ DE IMPACTO — Qué demo usa qué feature

### Leyenda: ✅ = esencial · ⚡ = mejora · — = no aplica

| Feature → Demo ↓ | Colisiones | Flocking | Fractales | L-System | Fisión | Fusión | Mutación | Cherenkov | Drama | Fuego | Viento | SPH |
|------------------|-----------|----------|-----------|----------|--------|--------|----------|-----------|-------|-------|--------|-----|
| demo_billar | ✅ | — | — | — | — | — | — | — | — | — | — | — |
| demo_futbol | ✅ | ⚡ | — | — | — | — | — | — | — | — | — | — |
| demo_asteroids | ✅ | — | — | — | — | — | — | — | — | — | — | — |
| demo_zombie_horde | — | ✅ | — | — | — | — | — | — | — | ⚡ | ⚡ | — |
| demo_racing | ✅ | — | — | — | — | — | — | — | — | — | — | — |
| demo_pinball | ✅ | — | — | — | — | — | — | — | — | — | — | — |
| demo_galaxy | ⚡ | — | ⚡ | — | ⚡ | ⚡ | — | — | — | — | — | — |
| demo_fallout | ⚡ | — | — | — | ✅ | — | ⚡ | ⚡ | ⚡ | — | — | — |
| demo_godzilla | — | — | — | — | ⚡ | — | ✅ | ⚡ | ⚡ | — | — | — |
| demo_chernobyl | — | — | — | — | ✅ | — | — | ✅ | ✅ | — | — | — |
| demo_birds | — | ✅ | — | — | — | — | — | — | — | — | ⚡ | — |
| demo_forest | — | — | — | ✅ | — | — | — | — | — | ✅ | ⚡ | — |
| demo_ocean | — | — | — | — | — | — | — | — | — | — | — | ✅ |
| demo_solar_system | ⚡ | — | — | — | — | ✅ | — | — | — | — | — | — |
| demo_supernova | — | — | ⚡ | — | ⚡ | ✅ | — | — | — | — | — | — |
| demo_evolution | — | — | — | — | — | — | ✅ | ⚡ | ⚡ | — | — | — |
| demo_fractals | — | — | ✅ | — | — | — | — | — | — | — | — | — |
| demo_cherenkov | — | — | — | — | — | — | — | ✅ | ⚡ | — | — | — |
| demo_pendulum | — | — | — | — | — | — | — | — | — | — | — | — |
| demo_cellular_life | — | — | ⚡ | — | — | — | — | — | — | — | — | — |

---

## 🗺️ ARQUITECTURA — Dónde vive cada cosa

### Filosofía: NO crates nuevos — todo en los existentes

**Ry-Dit ya tiene la estructura correcta:**
- `main.rs` / `rybot` → Orquestador, game loop, lógica de escenas
- `ry-physics` → **Toda la física** (newtoniana + radiación atómica)
- `ry-science` → **Toda la ciencia** (Bezier + biología/genética + fractales)
- `ry-gfx` → Render, partículas, audio, efectos visuales
- `postfx-ry` → Post-processing GPU (bloom, blur, color grading)
- `ry3d-gfx` → 3D, cámaras, skybox, primitivas
- `ry-anim` → Disney principles + action assets + effects (neon, morph, trails)
- `toolkit-ry` → Themes y estilos visuales

### Expansión de crates existentes

| Crate | Qué agregar |
|-------|------------|
| **ry-physics** | Colisiones elásticas, flocking, arrastre, viento, sistema solar, **decaimiento radiactivo**, **fisión nuclear**, **fusión nuclear** |
| **ry-science** | Fractales, autómatas, trig animada, L-System, Turing, caos, fase, espectro EM, **mutación genética**, **ADN**, **selección natural**, **evolución** |
| **ry-gfx/particles** | Fuego convección, Cherenkov glow, meteor trail, onda choque |
| **ry-gfx/audio** | Geiger counter, Doppler real |
| **postfx-ry** | Presets de **drama cinemático** (suspense, épico, documental, horror) + bloom Cherenkov + color grade mutación |
| **toolkit-ry** | **Estilos de drama**: cyberpunk, steampunk, anime, caricatura, pixel art, low-poly, mid-poly, high-poly, **drama nuclear**, **drama sci-fi** |
| **ry3d-gfx** | Skybox + primitives para escenas 3D |
| **rybot / main.rs** | Orquestación de escenas dramáticas, game loop con física extendida |

### Drama Cinemático — Es un ESTILO, no un crate

El **drama** en Ry-Dit es un **preset cinematográfico** que complementa los estilos de arte existentes. Se implementa como:

```
postfx-ry presets + toolkit-ry themes + ry-gfx/audio = Drama Cinemático
```

| Estilo de Drama | Postfx-ry Config | toolkit-ry Theme | Audio | Uso |
|----------------|-----------------|------------------|-------|-----|
| **Suspense nuclear** | Vignette alto + color frío + chromatic bajo | Azul Cherenkov + gris acero | Geiger lento + bass drone | Chernobyl, thriller |
| **Épico/heroico** | Bloom alto + color cálido + sharpen | Dorado + naranja | Orquestal + impacto | Godzilla, supernova |
| **Documental** | Natural + vignette suave | Neutro, realista | Ambiente + narración | Sistema solar, evolución |
| **Horror sci-fi** | Chromatic alto + bloom rojo + blur | Negro + rojo sangre | Disonancia + silencio | Mutación, fallout |
| **Anime dramático** | Bloom + saturación alta + vignette | Colores vivos, contrastados | Emotion J-style | Evolución épica |
| **Caricatura** | Sin postfx + colores planos | Paleta cartoon, sin gradientes | Sonidos exagerados | Juegos infantiles |
| **Pixel art** | Sharpen alto + sin blur + paleta limitada | 8-bit / 16-bit palette | Chiptune | Retro games |
| **Low-poly** | Color grade plano + sin bloom | Colores facetados, sin suavizar | Minimalista | Juegos indie |

### Flujo de Drama Cinemático

```
1. Usuario elige estilo: "drama_nuclear"
2. toolkit-ry aplica theme (colores, UI, fonts)
3. postfx-ry aplica chain (bloom, vignette, color grade)
4. ry-gfx/audio aplica mixer (Geiger, bass, silence)
5. ry-physics activa simulación (fisión, decaimiento)
6. ry-science activa datos (cadena desintegración)
7. main.rs / rybot orquesta todo en el game loop
```

### Crates existentes — Sin crear nada nuevo

| Feature | Dónde va | Por qué |
|---------|----------|---------|
| Decaimiento radiactivo | `ry-physics::radiation` | Es física nuclear |
| Fisión/fusión | `ry-physics::radiation` | Es física nuclear |
| Mutación genética | `ry-science::genetics` | Es biología |
| ADN/evolución | `ry-science::genetics` | Es biología |
| Reacción-difusión | `ry-science::turing` | Es química/matemáticas |
| Fractales | `ry-science::fractals` | Es matemáticas |
| Autómatas celulares | `ry-science::cellular` | Es computación |
| Drama cinemático | `postfx-ry presets` + `toolkit-ry themes` | Es estilo visual |
| Geiger counter | `ry-gfx/audio` | Es audio |
| Cherenkov glow | `ry-gfx/particles` + `postfx-ry` | Es visual |

---

## 🎯 PRIORIZACIÓN FINAL — Orden de ejecución

### 🔴 Semanas 1-2: Fundamentos (impacto inmediato en demos existentes)

1. **Colisiones elásticas** → demo_billar, War Spacio mejorado
2. **Flocking** → demo_birds (visual impactante)
3. **Fractales** → demo_fractals + postfx-ry = arte
4. **Autómatas celulares** → demo_cellular_life
5. **Trig animada** → demo_pendulum

### 🟡 Semanas 3-4: Naturaleza

6. **L-System** → demo_forest
7. **Sistema solar** → demo_solar_system
8. **Fuego convección** → antorcha, campamento
9. **Arrastre + Viento** → paracaídas, banderas

### 🟠 Semanas 5-7: Radiación Atómica

10. **Decaimiento radiactivo** → demo_chernobyl
11. **Fisión nuclear** → demo_fallout
12. **Cherenkov** → postfx-ry bloom + azul
13. **Fusión nuclear** → demo_supernova
14. **Geiger counter** → audio reactivo

### 🔵 Semanas 8-10: Biología + Genética

15. **Mutación genética** → demo_godzilla
16. **Reacción-difusión** → demo_turing_patterns
17. **Crecimiento poblacional** → demo_chaos
18. **Evolución** → demo_evolution

### 🟣 Semanas 11-12: Drama Cinemático

19. **Drama cinemático** → demo_chernobyl completo
20. **Ondas estacionarias** → demo_pendulum completo
21. **Cadena desintegración** → demo_periodic_table

---

## 📈 MÉTRICAS DE PROGRESO

| Meta | Actual | Semana 4 | Semana 7 | Semana 10 | Semana 12 |
|------|--------|----------|----------|-----------|-----------|
| **Features físicas** | 8 | 13 | 18 | 22 | 25 |
| **Features ciencia** | 8 | 12 | 12 | 18 | 22 |
| **Features biología** | 0 | 0 | 2 | 6 | 8 |
| **Features radiación** | 0 | 0 | 3 | 6 | 6 |
| **Features drama** | 0 | 0 | 0 | 0 | 3 |
| **Demos funcionales** | 24+ | 29+ | 34+ | 39+ | 44+ |
| **Tests totales** | ~292 | ~350 | ~400 | ~450 | ~500 |

---

## 🔗 RELACIÓN CON PIPELINE.TXT

El `pipeline.txt` propone:
> *"Ry-dit podrá no solo tener su Pipeline de textura sino crearlas, adaptarlas... con físicas, animaciones y procesos científicos/biológicos... se acerca mucho más al simulador de escenas"*

Este documento es el **mapa de implementación** de esa visión:

| Visión pipeline.txt | Feature correspondiente |
|---------------------|------------------------|
| "Físicas ceno arco ceno ángulos raíz" | Proyectil + Trig animada + Colisiones |
| "Física newtoniana avanzada + biología" | N-cuerpos + Flocking + Mutación genética |
| "Texturas moldeadas con código" | Fractales + L-System + Reacción-difusión |
| "Simulador de escenas" | TODOS los demos de simulación |
| "Estilos de textura como mods" | Postfx-ry presets (cyberpunk, natural, retro) |
| "Fusión de físicas + animaciones + ciencia" | Rybot drama subsystem |

---

<div align="center">

**🧲 Física, Biología, Radiación — Ry-Dit v0.20.0+**

*19 implementadas · 38 pendientes · 57 total · 33% listo*

*24+ demos actuales → 44+ demos planificados*

*~292 tests → ~500 tests planificados*

*0 crates nuevos — todo en ry-physics, ry-science, ry-gfx, postfx-ry, toolkit-ry*

*🎬 Drama Cinemático = postfx-ry presets + toolkit-ry themes (cyberpunk, steampunk, anime, nuclear, sci-fi)*

</div>
