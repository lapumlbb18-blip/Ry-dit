# Ry-Dit - ROADMAP v0.23.0 → v1.0.0

**Última actualización**: 2026-05-10
**Versión actual**: v0.23.0 ✅ Fusión SDL2 + RLGL (Arquitectura Híbrida)
**Próxima versión**: v0.24.0 — Migración de Demos + Renderizado de Texturas Unificado
**Análisis estratégico**: Transición a un motor multiplataforma basado en RLGL para GPU.

---

## Estado Actual (v0.23.0)

| Métrica | Valor |
|---------|-------|
| **Arquitectura** | Híbrida (SDL2 + RLGL) |
| **Crates** | 25 |
| **Compilación** | 0 errores (Núcleo consolidado) ✅ |
| **Tests** | ~260/260 pasando |
| **Demos Legacy** | 24 (Reubicados para migración) |
| **Repositorio** | `github.com/lapumlbb18-blip/Ry-dit` |

---

## 🧭 VISIÓN v0.23.0 — La Gran Fusión

El motor ha superado la dependencia de los componentes de escritorio antiguos (`Canvas` de SDL2) y ha adoptado una base híbrida:
1.  **SDL2 (Sistema)**: Manejo de ventana, contexto OpenGL y eventos de entrada multiplataforma.
2.  **RLGL (Gráficos)**: Renderizado de bajo nivel mediante `raylib-ffi`, permitiendo 2D y 3D unificado con alto rendimiento en dispositivos Low-End.

---

## Versiones Completadas (Reciente)

### v0.23.0 — Fusión SDL2 + RLGL ✅

**Fecha**: 2026-05-10

| Feature | Estado | Detalle |
|---------|--------|---------|
| Eliminación de SDL_Canvas | ✅ | El renderizado ahora es controlado por RLGL. |
| Shims de Compatibilidad | ✅ | `draw_rect`, `fill_rect`, `clear` redirigidos a RLGL. |
| Limpieza de Demos Legacy | ✅ | Movidos a `pendientes_importantes` para evitar ruido. |
| Estabilización de Núcleo | ✅ | El proyecto compila 100% sin errores en crates principales. |

```
Progreso: ████████████████████ 100%
```

---

## 🧭 ROADMAP PRÓXIMO

| Versión | Foco Principal | Estado |
|---------|----------------|--------|
| **v0.23.0** | Consolidación SDL2 + RLGL | ✅ Completado |
| **v0.24.0** | Migración de Demos a la nueva arquitectura | ⏳ Siguiente paso |
| **v0.25.0** | Unificación de carga de Assets (Texture2D) | ⏳ Pendiente |
| **v1.0.0**  | Motor Completo + GitHub Actions CI/CD | ⏳ Pendiente |

---

## Progreso General

```
v0.19.2   ████████████████████ 100%
v0.23.0   ████████████████████ 100% (ACTUAL)
v0.24.0   ░░░░░░░░░░░░░░░░░░░░   0%
v1.0.0    ░░░░░░░░░░░░░░░░░░░░   0%
```

---

<div align="center">

**Ry-Dit v0.23.0 - ROADMAP ACTUALIZADO**

*Fusión exitosa: SDL2 para el sistema, RLGL para el arte.*

*Próximo: v0.24.0 — Recuperación de demos y renderizado de texturas.*

</div>
