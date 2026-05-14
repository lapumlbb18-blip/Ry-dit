# Plan de Integración: Asset Pipeline (Ry-Dit v0.20.0)

## Visión General
El objetivo es conectar la nueva arquitectura de **Action Assets** (basada en tipos fuertes) con el **AssetServer** existente, convirtiendo el sistema en un pipeline de datos tipado y eficiente.

## Plan de Acción (Fase 2)

### 1. Extensión de AssetType
Actualizar `AssetType` en `crates/ry-core/src/assets.rs` para incluir:
- `SpriteAnimation`: Para animaciones de cuadros (Action Assets).
- `SpriteSheet`: Para la configuración de hojas de sprites.
- `Config`: Para configuraciones generales del motor.

### 2. Lógica de Deserialización Tipada
Modificar `AssetServer` para que, tras la lectura binaria (`fs::read`), realice el parseo automático hacia nuestros structs tipados definidos en `ry-anim`.

- **Proceso**:
  1. `AssetServer` detecta el tipo (`AssetType`).
  2. Si es `SpriteAnimation`, deserializa el JSON/SAZ automáticamente.
  3. Almacena en caché el objeto tipado, no solo los bytes crudos.

### 3. Integración en el Game Loop
Refactorizar los módulos y demos para utilizar el `AssetServer` como fuente de verdad:

```rust
// Ejemplo de integración esperada:
let anim: SpriteAnimation = asset_server.get_typed::<SpriteAnimation>(id)?;
let result = anim.update(time);
```

### 4. Objetivos de Verificación
- [ ] Eliminar carga manual de archivos en las demos (`demo_war_spacio.rydit`).
- [ ] Validar que la deserialización tipada falla de forma segura (fail-fast) si el asset está corrupto.
- [ ] Mantener la compatibilidad con el sistema de módulos `RyditModule` existente.

## Beneficios Esperados
- **Integridad de Datos**: Garantizar que el motor no intente procesar datos mal formados.
- **Eficiencia**: Acceso directo a structs, eliminando el parseo repetitivo durante el frame loop.
- **Preparación para el Editor**: La capacidad de cargar structs tipados es la base indispensable para el futuro inspector de propiedades en el Editor Visual.

---
*Plan actualizado para la sesión v0.21.0.*
