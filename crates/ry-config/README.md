# ry-config

**Config parser for Ry-Dit — Parse `.rydit` config files for entities, levels, and checkpoints**

[![Crates.io](https://img.shields.io/crates/v/ry-config.svg)](https://crates.io/crates/ry-config)
[![Documentation](https://docs.rs/ry-config/badge.svg)](https://docs.rs/ry-config)
[![License](https://img.shields.io/crates/l/ry-config.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

## Overview

`ry-config` is a zero-dependency configuration parser for the Ry-Dit game engine. It reads custom `.rydit` config files and produces typed structures for entities, levels, and checkpoints — all in pure Rust with no external dependencies.

## Features

- **Zero dependencies** — pure Rust, no external crates
- **Entity parsing** — type, sprite, position, size, properties (vida, daño, velocidad)
- **Level parsing** — name, gravity, background, music, checkpoints
- **Typed values** — `ValorConfig` enum (Texto, Numero, Bool, Array)
- **Error handling** — descriptive error messages for malformed configs
- **Fast** — single-pass line-by-line parsing

## Installation

```toml
[dependencies]
ry-config = "0.1.0"
```

## Quick Start

### Parse a `.rydit` config file

```rust
use ry_config::ConfigParser;

let config = ConfigParser::parse("nivel1.rydit")?;

println!("Level: {}", config.nombre);
println!("Gravity: {}", config.gravedad);
println!("Background: {}", config.fondo);

for entity in &config.entidades {
    println!("  Entity: {} (type: {}, sprite: {})", entity.id, entity.tipo, entity.sprite);
    println!("    Position: ({}, {})", entity.x, entity.y);
    println!("    Size: {}x{}", entity.ancho, entity.alto);
}
```

### Parse from string

```rust
use ry_config::ConfigParser;

let contenido = r#"
@nombre "Nivel 1 - Bosque"
gravedad: 9.8
fondo: "bosque.png"

entidad "jugador" {
    tipo: "hero"
    sprite: "hero.png"
    x: 100
    y: 200
    vida: 100
}

entidad "enemigo" {
    tipo: "goblin"
    sprite: "goblin.png"
    x: 400
    y: 200
    vida: 50
    daño: 10
}
"#;

let config = ConfigParser::parse_contenido(contenido)?;
assert_eq!(config.nombre, "Nivel 1 - Bosque");
assert_eq!(config.entidades.len(), 2);
```

### Access entity properties

```rust
use ry_config::{ConfigParser, ValorConfig};

let config = ConfigParser::parse_contenido(contenido)?;

for entity in &config.entidades {
    if let Some(ValorConfig::Numero(vida)) = entity.propiedades.get("vida") {
        println!("{} has {} HP", entity.id, vida);
    }
}
```

## `.rydit` Config Format

### Level definition

```
@nombre "Nombre del Nivel"
gravedad: 9.8
fondo: "background.png"
musica: "level1.ogg"
```

### Entity definition

```
entidad "id" {
    tipo: "hero"
    sprite: "hero.png"
    x: 100
    y: 200
    ancho: 32
    alto: 32
    vida: 100
    daño: 10
    velocidad: 200
    estatica: false
}
```

### Checkpoint definition

```
checkpoint "inicio" {
    x: 50
    y: 100
}
```

### Comments and blank lines

Lines starting with `#` or blank lines are ignored:

```
# This is a comment
@nombre "Test"

# Blank lines are also ignored
```

## API Reference

### ConfigParser

```rust
pub struct ConfigParser;

impl ConfigParser {
    /// Parse a .rydit file from disk
    pub fn parse(ruta: &str) -> Result<NivelConfig, String>;

    /// Parse .rydit content from a string
    pub fn parse_contenido(contenido: &str) -> Result<NivelConfig, String>;

    /// Helpers
    pub fn extraer_texto(linea: &str, clave: &str) -> Result<String, String>;
    pub fn extraer_numero(linea: &str, clave: &str) -> Result<f32, String>;
    pub fn extraer_id(linea: &str) -> Result<String, String>;
}
```

### NivelConfig

```rust
pub struct NivelConfig {
    pub nombre: String,
    pub gravedad: f32,
    pub fondo: String,
    pub musica: String,
    pub entidades: Vec<EntityConfig>,
    pub checkpoints: HashMap<String, (f32, f32)>,
}
```

### EntityConfig

```rust
pub struct EntityConfig {
    pub id: String,
    pub tipo: String,
    pub sprite: String,
    pub x: f32,
    pub y: f32,
    pub ancho: f32,
    pub alto: f32,
    pub propiedades: HashMap<String, ValorConfig>,
}
```

### ValorConfig

```rust
pub enum ValorConfig {
    Texto(String),
    Numero(f32),
    Bool(bool),
    Array(Vec<f32>),
}
```

## Examples

### Complete level config file

```
@nombre "Bosque Encantado"
@descripcion "Un bosque mágico lleno de criaturas"
@autor "Tu Nombre"
@version "1.0"

gravedad: 12.0
fondo: "bosque_bg.png"
musica: "bosque_ambient.ogg"

entidad "jugador" {
    tipo: "hero"
    sprite: "hero_idle.png"
    x: 50
    y: 300
    ancho: 32
    alto: 48
    vida: 100
    velocidad: 250
}

entidad "enemigo_1" {
    tipo: "goblin"
    sprite: "goblin.png"
    x: 400
    y: 300
    ancho: 32
    alto: 32
    vida: 30
    daño: 5
    velocidad: 80
}

entidad "enemigo_2" {
    tipo: "slime"
    sprite: "slime.png"
    x: 600
    y: 300
    ancho: 24
    alto: 24
    vida: 20
    daño: 3
    velocidad: 40
    estatica: false
}

checkpoint "inicio" {
    x: 50
    y: 300
}

checkpoint "medio" {
    x: 500
    y: 200
}
```

### Iterating over entities by type

```rust
use ry_config::ConfigParser;

let config = ConfigParser::parse("nivel1.rydit")?;

let enemigos: Vec<_> = config.entidades
    .iter()
    .filter(|e| e.tipo == "goblin" || e.tipo == "slime")
    .collect();

println!("Found {} enemies", enemigos.len());
```

### Creating entities from config

```rust
use ry_config::ConfigParser;

let config = ConfigParser::parse("nivel1.rydit")?;

for entity in &config.entidades {
    let pos = (entity.x, entity.y);
    let size = (entity.ancho, entity.alto);

    // Spawn entity in your game engine
    spawn_entity(&entity.tipo, &entity.sprite, pos, size);

    // Apply custom properties
    if let Some(ry_config::ValorConfig::Numero(vida)) = entity.propiedades.get("vida") {
        set_hp(entity.id.as_str(), *vida as i32);
    }
}
```

## Performance

- **Single pass** — reads file line by line, no backtracking
- **Zero allocations** for ignored lines (comments, blanks)
- **HashMap-based** properties for O(1) lookup
- **No external dependencies** — minimal binary size

## Limitations

- Property values are limited to: `f32`, `String`, `bool`, `Vec<f32>`
- No nested entity definitions (flat structure only)
- No expression evaluation (values are literal)
- No include/import mechanism

These limitations are by design — the parser is meant to be simple and fast. For complex configurations, use TOML, JSON, or the full Ry-Dit scripting engine.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| *(none)* | — | **Zero external dependencies** |

## Roadmap

- [ ] Integer type (`ValorConfig::Entero(i32)`)
- [ ] Nested entity properties
- [ ] Include/import mechanism
- [ ] Expression evaluation in values
- [ ] Schema validation

## Contributing

Contributions are welcome! This crate is part of the **Ry-Dit** game engine project.

- **Repository**: https://github.com/lapumlbb18-blip/Ry-dit
- **Issues**: https://github.com/lapumlbb18-blip/Ry-dit/issues
- **Pull Requests**: Welcome!

Please read [CONTRIBUTING.md](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE) for details.

---

<div align="center">

**ry-config** — Zero-dependency config parser for Ry-Dit game engine 📝✨

*3 tests · 230 lines · Zero dependencies · Pure Rust*

</div>
