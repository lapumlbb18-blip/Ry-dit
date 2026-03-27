# 📋 PLANIFICACIÓN v0.5.1 - AUDIO + HTTP + DATA SCIENCE

**Fecha**: 2026-03-26 (Próxima sesión)
**Versión actual**: v0.5.0 ✅ ESTABLE
**Versión objetivo**: v0.5.1

---

## 🎯 OBJETIVOS

### 1. Audio - Sonidos Básicos ⭐⭐⭐
**Prioridad**: ALTA

#### Features a Implementar
- `audio::beep(frecuencia, duracion)` - Sonido tipo beep
- `audio::click()` - Sonido de click UI
- `audio::play_sound("path")` - Reproducir archivo WAV/MP3

---

### 2. HTTP Request - GET Sencillo ⭐⭐
**Prioridad**: MEDIA

#### Features a Implementar
- `http::get(url)` - GET request sencillo
- `http::post(url, data)` - POST request (opcional)

---

### 3. Data Science - CSV ⭐⭐
**Prioridad**: MEDIA

#### Features a Implementar
**Lo que falta:**
- ❌ `csv::read("file.csv")` - Leer archivos CSV
- ❌ `csv::write(data, "file.csv")` - Escribir CSV
- ❌ `stats::std_dev([1,2,3,4,5])` - Desviación estándar
- ❌ `stats::variance([1,2,3,4,5])` - Varianza
- ❌ `plot::ascii(data)` - Gráficos ASCII en consola

---

### 4. Documentación ⭐
**Prioridad**: BAJA

#### Tasks
- [ ] Actualizar README con ejemplos de audio
- [ ] Actualizar README con ejemplos de HTTP
- [ ] Actualizar README con ejemplos de CSV
- [ ] Crear demo de audio (beep + sonidos)
- [ ] Crear demo de HTTP (API call simple)
- [ ] Crear demo de CSV (leer/escribir datos)

---

## 📦 CRATES INVOLUCRADOS

### Nuevos (a crear)
- `crates/rydit-audio/` - Audio (beep, sonidos, música)
- `crates/rydit-http/` - HTTP requests (GET, POST)

### Existentes (a modificar)
- `crates/rydit-rs/src/main.rs` - Exponer funciones `audio::`, `http::`, `csv::`
- `crates/rydit-gfx/` - Posible integración con audio
- `crates/rydit-science/` - Agregar CSV + stats avanzados + plot

---

## 🔧 IMPLEMENTACIÓN PASO A PASO

### Sesión 1: Audio Básico
1. Crear `crates/rydit-audio/Cargo.toml`
2. Implementar `beep()` y `click()` con raylib
3. Exponer en `main.rs` como `audio::beep()`, `audio::click()`
4. Crear demo `demo_audio.rydit`
5. Tests

### Sesión 2: HTTP GET
1. Agregar `ureq` dependency a `rydit-rs/Cargo.toml`
2. Implementar `http_get()` function
3. Exponer en `main.rs` como `http::get()`
4. Crear demo `demo_http.rydit`
5. Tests

### Sesión 3: CSV + Stats Avanzados
1. Crear `crates/rydit-science/src/csv.rs`
2. Implementar `csv::read()`, `csv::write()`
3. Agregar `stats::std_dev()`, `stats::variance()` a `rydit-science`
4. Implementar `plot::ascii()`
5. Crear demo `demo_csv.rydit`
6. Tests

### Sesión 4: Integración + Docs
1. Demo combinado (audio + HTTP + CSV)
2. Actualizar README
3. Actualizar QWEN.md
4. Release v0.5.1

---

## 📊 METAS

| Feature | Líneas | Tests | Demo |
|---------|--------|-------|------|
| Audio beep/click | ~100 | 5+ | ✅ |
| HTTP GET | ~50 | 3+ | ✅ |
| CSV read/write | ~150 | 5+ | ✅ |
| Stats avanzados | ~50 | 4+ | ✅ |
| Plot ASCII | ~80 | 2+ | ✅ |

**Total estimado**: ~430 líneas nuevas, 19+ tests, 3-4 demos

---

## ⚠️ RIESGOS

### Audio
- raylib audio puede no estar disponible en Termux
- Solución: Usar `miniaudio` o `rodio` como fallback

### HTTP
- Requiere TLS/SSL para HTTPS
- Solución: `ureq` con `native-tls` o `rustls`

### CSV
- Parsing de CSV con comas, quotes, escapes
- Solución: Usar crate `csv` de Rust

---

## ✅ CRITERIOS DE ACEPTACIÓN

- [ ] `audio::beep()` funciona en Termux-X11
- [ ] `audio::click()` suena al hacer click en UI
- [ ] `http::get()` retorna datos de API pública
- [ ] `csv::read()` lee archivos CSV correctamente
- [ ] `csv::write()` escribe archivos CSV
- [ ] `stats::std_dev()` calcula desviación estándar
- [ ] `stats::variance()` calcula varianza
- [ ] `plot::ascii()` imprime gráfico en consola
- [ ] 19+ tests passing
- [ ] 3-4 demos funcionales
- [ ] README actualizado

---

<div align="center">

**🛡️ RyDit v0.5.1 - Audio + HTTP + Data Science**

*~430 líneas | 19+ tests | 3-4 demos | 0 dependencias críticas*

</div>
