# 🛠️ Guía de Compilación en Termux-X11

**Fecha**: 2026-04-05
**Entorno**: Android/Termux (NO Linux completo)
**Arquitectura**: ARM aarch64

---

## 📊 Estado del Entorno

| Componente | Versión | Estado |
|------------|---------|--------|
| **rustc** | Latest | ✅ |
| **cargo** | Latest | ✅ |
| **raylib** | 5.5.1 (sistema) | ✅ `/data/data/com.termux/files/usr/lib/libraylib.so` |
| **SDL2** | 0.37 + features | ✅ image, ttf, mixer |
| **Linker** | lld (NDK r29) | ✅ |

---

## 🔧 El Problema y la Solución

### Problema Original

`ry-gfx/Cargo.toml` tiene `links = "raylib"` que dice a Cargo que este crate linkea raylib.
Pero `build.rs` **solo emitía libs SDL2**, no raylib.

Con el feature `nobuild` en la dependencia de raylib:
```toml
raylib = { version = "5.5.1", default-features = false, features = ["nobuild"] }
```

raylib **NO compila su propia librería C** — espera `libraylib.so` del sistema.
Pero Cargo nunca fue informado de que debe linkear `-lraylib`.

### Solución Aplicada

**Archivo**: `crates/ry-gfx/build.rs`

```rust
fn main() {
    // SDL2 libs (ya existían)
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
    println!("cargo:rustc-link-lib=SDL2_ttf");
    println!("cargo:rustc-link-lib=SDL2_mixer");

    // ✅ NUEVO: raylib del sistema
    println!("cargo:rustc-link-lib=raylib");

    // pkg-config para SDL2
    if let Ok(libs) = pkg_config::Config::new().probe("sdl2") {
        println!("cargo:rustc-link-search=native={}", ...);
    }

    // ✅ NUEVO: pkg-config para raylib también
    if let Ok(libs) = pkg_config::Config::new().probe("raylib") {
        println!("cargo:rustc-link-search=native={}", ...);
    }
}
```

---

## 📋 Pasos para Compilar Cualquier Demo

### 1. Verificar que raylib esté instalado

```bash
pkg-config --libs raylib
# Debe mostrar: -L/data/data/com.termux/files/usr/lib -lraylib

ls /data/data/com.termux/files/usr/lib/libraylib.so
# Debe existir
```

### 2. Compilar en modo debug (rápido, para desarrollo)

```bash
cargo build --bin demo_anime_ry
# Resultado: target/debug/demo_anime_ry (~779K, no stripped)
```

### 3. Compilar en modo release (optimizado, para distribución)

```bash
cargo build --bin demo_anime_ry --release
# Resultado: target/release/demo_anime_ry (~341K, stripped, LTO)
```

### 4. Verificar el ELF

```bash
file target/release/demo_anime_ry
# ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV)
# dynamically linked, interpreter /system/bin/linker64
# for Android 24, built by NDK r29, stripped
```

### 5. Ejecutar

```bash
./target/release/demo_anime_ry
# Necesita Termux-X11 corriendo con DISPLAY=:0
```

---

## 🏗️ Arquitectura de Compilación

```
demo_anime_ry.rs
    ↓ usa
ry-gfx (crate)
    ├── usa rylib (Rust bindings) → nobuild → libraylib.so (sistema)
    └── usa sdl2 (Rust bindings) → libsdl2.so (sistema)
    ↓
Cargo build.rs emite:
    -lSDL2 -lSDL2_image -lSDL2_ttf -lSDL2_mixer -lraylib
    ↓
Linker (lld) resuelve símbolos de:
    /data/data/com.termux/files/usr/lib/libSDL2.so
    /data/data/com.termux/files/usr/lib/libraylib.so
    ↓
ELF ARM aarch64
```

### Por qué otros demos funcionaban antes

Los demos existentes (`demo_50k_particulas`, `demo_colisiones`, etc.) **NO llaman**
`RyditGfx::new()` — usan SDL2 directamente. Como nunca ejecutan código raylib FFI,
el linker nunca necesita resolver los símbolos de `libraylib.so`.

`demo_anime_ry` es diferente porque llama `RyditGfx::new()` que internamente llama
`raylib::init()`, lo que tira de los símbolos FFI de raylib.

---

## ⚠️ Problemas Comunes y Soluciones

### Error: `undefined symbol: InitWindow`

**Causa**: `build.rs` no emite `-lraylib`.
**Solución**: Verificar que `crates/ry-gfx/build.rs` tenga:
```rust
println!("cargo:rustc-link-lib=raylib");
```

### Error: `cannot find -lraylib`

**Causa**: `libraylib.so` no está en el path del linker.
**Solución**:
```bash
pkg install raylib
# o
export LDFLAGS="-L/data/data/com.termux/files/usr/lib"
```

### Error: `pkg-config not found`

**Causa**: pkg-config no está instalado.
**Solución**:
```bash
pkg install pkg-config
```

### Compilación lenta en release

**Causa**: LTO + opt-level = "z" son agresivos.
**Solución**: Para desarrollo rápido:
```bash
cargo build --bin demo_anime_ry  # debug, ~30s
```
Para distribución:
```bash
cargo build --bin demo_anime_ry --release  # release, ~40s, 341K
```

---

## 📦 Perfil de Release (Cargo.toml)

```toml
[profile.release]
opt-level = "z"      # Optimizar por tamaño (binarios más pequeños)
lto = true           # Link Time Optimization (más rápido, más lento compilar)
panic = "abort"      # Panic = abort (binario más pequeño)
strip = true         # Strip symbols (341K vs 779K)
```

---

## 🎯 Resumen de Flags Implícitas

| Variable | Valor | Dónde |
|----------|-------|-------|
| `links` | `"raylib"` | `crates/ry-gfx/Cargo.toml` |
| `raylib feature` | `nobuild` | `crates/ry-gfx/Cargo.toml` |
| `rustc-link-lib` | SDL2, SDL2_image, SDL2_ttf, SDL2_mixer, **raylib** | `crates/ry-gfx/build.rs` |
| `opt-level` | `"z"` | `Cargo.toml` workspace |
| `lto` | `true` | `Cargo.toml` workspace |
| `strip` | `true` | `Cargo.toml` workspace |

---

<div align="center">

**🛡️ Guía de Compilación Termux — Ry-Dit v0.12.0**

*ELF ARM aarch64 | SDL2 + raylib | 341K release*

*`cargo build --bin demo_anime_ry --release`*

</div>
