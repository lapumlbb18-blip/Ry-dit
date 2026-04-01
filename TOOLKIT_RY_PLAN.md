# 🛡️ toolkit-ry - UI Toolkit para RyDit

**Fecha**: 2026-04-01
**Estado**: 📋 Diseño simplificado
**Ubicación**: `crates/toolkit-ry/` (nuevo crate)

---

## 🎯 **DECISIÓN ARQUITECTÓNICA**

### **Problema Detectado**
- ❌ MiGUI tiene su propio backend (`MiguiSdl2Backend`)
- ❌ rydit-gfx tiene otro backend (`Sdl2Backend`)
- ❌ Hay choque de responsabilidades

### **Solución**
Crear `toolkit-ry` como crate **separado** que:
- ✅ Usa `rydit-gfx` directamente (ya tiene SDL2)
- ✅ Es agnóstico (puede usarse en juegos O en MiGUI)
- ✅ Se fusiona después si tiene sentido

---

## 📁 **ESTRUCTURA SIMPLIFICADA**

```
crates/toolkit-ry/
├── Cargo.toml
└── src/
    ├── lib.rs           # API principal
    ├── widgets/
    │   ├── button.rs    # Botones
    │   └── label.rs     # Texto
    └── theme.rs         # Colores
```

---

## 🔧 **API MÍNIMA VIABLE**

```rust
use toolkit_ry::{Button, Label, Theme};
use rydit_gfx::backend_sdl2::Sdl2Backend;

// En tu game loop
fn render(backend: &mut Sdl2Backend) {
    let theme = Theme::dark();
    
    // Botón
    let btn = Button::new("Jugar")
        .position(100, 100)
        .size(150, 40);
    btn.render(backend, &theme);
    
    // Label
    let label = Label::new("Mi Juego")
        .position(100, 50)
        .size(24);
    label.render(backend, &theme);
}
```

---

## 📋 **PRÓXIMOS PASOS**

1. ✅ Crear `crates/toolkit-ry/`
2. ✅ Mover widgets allí (Button, Label)
3. ✅ Usar `rydit-gfx::Sdl2Backend`
4. ✅ Testear con demo simple
5. ⏸️ Decidir si integrar con MiGUI o dejar separado

---

<div align="center">

**🛡️ toolkit-ry - Simplificado y Funcional**

*Sin choques de backend ✅ | Independiente ✅ | Integrable 🔮*

</div>
