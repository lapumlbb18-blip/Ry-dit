# Plan de Refactorización: ry-god v0.22.0 (Supervisor de Integridad)

## Visión General
Refactorizar `ry-god` para eliminar la validación estática (Lexer/Parser) en tiempo de ejecución, delegando esa responsabilidad al `AssetPipeline`. Convertir a `ry-god` en un *watchdog* industrial enfocado exclusivamente en auditoría, límites de recursos y monitoreo de rendimiento (benchmarking) en tiempo real.

## Plan de Acción

### 1. Desacoplamiento de la Validación (Performance)
- **Eliminar** la lógica que importa `ry_lexer` y `ry_parser` dentro de `RyGod::validate_script`.
- **Delegar**: La validación sintáctica será pre-condición del `AssetServer`. Si el archivo no parsea, el asset no entra en caché.
- **Resultado**: `ry-god` será extremadamente ligero (nanosegundos de overhead).

### 2. Monitor de Recursos (Watchdog)
- Mantener y optimizar los límites:
    - `max_memory_mb`
    - `max_instructions` (vía contador en la VM)
    - `max_loop_iterations`
- Implementar un *callback* que el motor pueda disparar si el script excede estos límites.

### 3. Benchmark de Debugging Avanzado
- Implementar un sistema de **"Debug Trace"** dentro de `ry-god`.
- **Características del reporte**:
    - Tiempo de ejecución acumulado por instrucción.
    - Consumo de memoria peak detectado.
    - Historial de llamadas a funciones (Call Stack Trace) en caso de crash o abort.
- **Salida**: Formato JSON/TTY optimizado para análisis rápido durante el desarrollo.

### 4. Objetivos de Verificación
- [ ] Lograr que `RyGod::run` tenga una latencia < 1ms para scripts ya cargados.
- [ ] Validar que el reporte de Debug contenga: *Time, Memory, Instruction Count*.
- [ ] Asegurar que el sistema de auditoría sea capaz de escribir logs sin bloquear el hilo principal (Async logging).

## Filosofía
`ry-god` es el supervisor de que Ry-Dit es un motor "decente" en cualquier hardware. Su misión no es compilar, es **observar**.

---
*Plan generado para el crate ry-god.*
