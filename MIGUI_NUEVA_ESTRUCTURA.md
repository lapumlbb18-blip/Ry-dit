# 🛡️ MiGUI v2.0 - Nueva Estructura + Toolkit + RyBot

**Fecha**: 2026-04-01
**Versión**: v2.0 (Planificada)
**Estado**: 📋 En diseño

---

## 🎯 **VISIÓN**

MiGUI evoluciona de un backend SDL2 a un **toolkit UI completo** + **inspector RyBot** para el Sistema Universal Ry.

### **Dualidad MiGUI**

| Modo | Función | Uso |
|------|---------|-----|
| **Runtime** | UI del juego (menús, HUD, botones) | `migui::Button::new("Jugar")` |
| **Editor** | Inspector RyBot (debug, assets, entities) | `rybot::inspector::view(entity)` |

---

## 📁 **ESTRUCTURA DE ARCHIVOS**

```
crates/migui/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs                  # API principal
    │
    ├── backend_sdl2.rs         # ✅ SDL2 Backend (existente)
    │   ├── Sdl2Backend
    │   ├── InputState
    │   └── FontFFI (SDL2_ttf)
    │
    ├── toolkit/                # 🆕 UI Toolkit
    │   ├── mod.rs
    │   │
    │   ├── widgets/            # Componentes UI
    │   │   ├── mod.rs
    │   │   ├── button.rs       # Botones clickeables
    │   │   ├── label.rs        # Texto estático/dinámico
    │   │   ├── panel.rs        # Contenedores/ventanas
    │   │   ├── progress.rs     # Barras (vida, energía, XP)
    │   │   ├── slider.rs       # Deslizadores (volumen, brillo)
    │   │   ├── checkbox.rs     # Casillas (on/off)
    │   │   └── image.rs        # Imágenes/sprites UI
    │   │
    │   ├── layout/             # Distribución automática
    │   │   ├── mod.rs
    │   │   ├── vbox.rs         # Vertical
    │   │   ├── hbox.rs         # Horizontal
    │   │   ├── grid.rs         # Grilla
    │   │   └── anchor.rs       # Anclado (top-left, center, etc.)
    │   │
    │   ├── theme/              # Estilos personalizables
    │   │   ├── mod.rs
    │   │   ├── colors.rs       # Paletas de colores
    │   │   ├── fonts.rs        # Fuentes TTF
    │   │   ├── style.rs        # Estilos predefinidos
    │   │   └── animations.rs   # Animaciones UI (hover, click)
    │   │
    │   └── input/              # Input UI
    │       ├── mod.rs
    │       ├── navigation.rs   # Navegación con gamepad/teclado
    │       ├── focus.rs        # Sistema de focus/selection
    │       └── events.rs       # Click, hover, drag
    │
    └── rybot/                  # 🆕 Inspector RyBot
        ├── mod.rs
        │
        ├── inspector.rs        # Ver/editar propiedades de entidades
        │   ├── EntityInspector
        │   ├── PropertyEditor
        │   └── ComponentViewer
        │
        ├── console.rs          # Logs en tiempo real
        │   ├── LogEntry
        │   ├── LogLevel (Info, Warn, Error)
        │   └── Filter
        │
        ├── profiler.rs         # FPS, memoria, entities
        │   ├── FpsCounter
        │   ├── MemoryTracker
        │   └── EntityGraph
        │
        ├── assets_browser.rs   # Navegador de assets
        │   ├── AssetTree
        │   ├── PreviewPanel
        │   └── ImportDialog
        │
        └── registry.rs         # Registro de eventos Ry
            ├── EventLog
            ├── ScriptTracker
            └── AutoSave
```

---

## 🔧 **FEATURES PRINCIPALES**

### **1. Toolkit UI**

```rust
use migui::toolkit::{App, Button, Label, VBox, Theme};

fn main() {
    let mut app = App::new("Mi Juego", 800, 600);
    app.set_theme(Theme::dark());

    // Menú principal
    let menu = VBox::new(vec![
        Label::new("MI JUEGO").size(48),
        Button::new("Jugar").on_click(start_game),
        Button::new("Opciones").on_click(show_options),
        Button::new("Salir").on_click(quit),
    ]);

    app.add_widget(menu);
    app.run();
}
```

---

### **2. RyBot Inspector**

```rust
use migui::rybot::{Inspector, Console, Profiler};

// En el game loop
fn game_loop(entity: &Entity) {
    // Inspector muestra propiedades en tiempo real
    Inspector::view(entity);
    
    // Console muestra logs
    Console::info("Jugador saltó");
    Console::warn("Colisión detectada");
    
    // Profiler muestra FPS
    Profiler::show_fps();
}
```

---

### **3. HUD (Heads-Up Display)**

```rust
use migui::toolkit::{ProgressBar, Label, Anchor};

// Barra de vida
let health_bar = ProgressBar::new(0.0..100.0)
    .value(80.0)
    .color(Color::Red)
    .anchor(Anchor::TopLeft);

// Contador de monedas
let coins = Label::new("💰 42")
    .size(24)
    .anchor(Anchor::TopRight);

// Aviso temporal
let notification = Label::new("¡Nivel completado!")
    .blink(true)
    .duration(3.0); // Desaparece en 3 segundos
```

---

## 📊 **INTEGRACIÓN CON SISTEMA RY**

### **Registro Automático**

```rydit
# nivel_1.rydit
entidad "jugador" {
    sprite: "player.png"
    x: 400
    y: 300
    vida: 100
}
```

**RyBot registra**:
```rust
Registry::log("entidad 'jugador' creada");
Registry::track_property("jugador.vida", 100);
Inspector::add_entity("jugador");
```

---

### **Inspector en Tiempo Real**

```
┌─────────────────────────────────┐
│  RyBot Inspector                │
├─────────────────────────────────┤
│  Entidad: jugador               │
│  ├─ sprite: "player.png"  [✏️] │
│  ├─ x: 400                  [✏️] │
│  ├─ y: 300                  [✏️] │
│  ├─ vida: 100               [✏️] │
│  └─ script: "player_control.rydit" │
├─────────────────────────────────┤
│  [Aplicar] [Cancelar]           │
└─────────────────────────────────┘
```

---

## 🎯 **PLAN DE IMPLEMENTACIÓN**

### **Fase 1: MiGUI Toolkit Básico** (2-3 días)
- [ ] `Button` - Botones clickeables
- [ ] `Label` - Texto estático
- [ ] `Panel` - Contenedores
- [ ] `Theme` - Estilos claro/oscuro
- [ ] `VBox`/`HBox` - Layout básico

### **Fase 2: RyBot Inspector** (2-3 días)
- [ ] `Inspector` - Ver propiedades
- [ ] `Console` - Logs en tiempo real
- [ ] `Profiler` - FPS, memoria
- [ ] Integración con `entity.rs`

### **Fase 3: Widgets Avanzados** (2-3 días)
- [ ] `ProgressBar` - Barras de vida
- [ ] `Slider` - Deslizadores
- [ ] `Checkbox` - Casillas
- [ ] `Image` - Sprites UI

### **Fase 4: RyBot Registry** (2-3 días)
- [ ] `Registry` - Registro de eventos
- [ ] `AssetsBrowser` - Navegador de assets
- [ ] `AutoSave` - Guardado automático
- [ ] Integración con parser `.rydit`

### **Fase 5: Plantillas y Proyectos** (2 días)
- [ ] Crear proyecto desde plantilla
- [ ] Guardar/cargar proyectos
- [ ] Exportar configuración
- [ ] MiGUI como editor visual

---

## 📋 **ESTADO ACTUAL**

| Componente | Estado | Notas |
|------------|--------|-------|
| **backend_sdl2.rs** | ✅ 100% | SDL2 + input + SDL2_ttf |
| **toolkit/widgets** | ⏸️ Pendiente | Empezar por Button + Label |
| **toolkit/layout** | ⏸️ Pendiente | VBox, HBox básicos |
| **toolkit/theme** | ⏸️ Pendiente | Theme claro/oscuro |
| **rybot/inspector** | ⏸️ Pendiente | Ver propiedades de entities |
| **rybot/console** | ⏸️ Pendiente | Logs simples |
| **rybot/profiler** | ⏸️ Pendiente | FPS counter |

---

## 🚀 **PRÓXIMOS PASOS**

1. ✅ **HOY**: Empezar Fase 1 (toolkit básico)
2. ⏸️ **Mañana**: Continuar con widgets
3. ⏸️ **Día 3**: RyBot inspector básico
4. ⏸️ **Día 4-5**: Integración con Sistema Ry

---

## 📊 **MÉTRICAS ESPERADAS**

| Métrica | Actual | Esperado v2.0 |
|---------|--------|---------------|
| **Líneas MiGUI** | ~200 | ~2000+ |
| **Widgets** | 0 | 10+ |
| **Tests** | 0 | 50+ |
| **Demos** | 0 | 5+ (menús, HUD, inspector) |

---

<div align="center">

**🛡️ MiGUI v2.0 - Toolkit UI + RyBot Inspector**

*Backend ✅ | Toolkit 🔮 | RyBot 🔮 | Integración 🔮*

**Próximo: Fase 1 - Toolkit Básico (Button + Label)**

</div>
