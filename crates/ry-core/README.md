# ry-core

**Core trait and module registry for Ry-Dit game engine**

[![Crates.io](https://img.shields.io/crates/v/ry-core.svg)](https://crates.io/crates/ry-core)
[![Documentation](https://docs.rs/ry-core/badge.svg)](https://docs.rs/ry-core)
[![License](https://img.shields.io/crates/l/ry-core.svg)](https://github.com/lapumlbb18-blip/Ry-dit/blob/main/LICENSE)

## Overview

`ry-core` provides the foundational `RyditModule` trait and `ModuleRegistry` system for the Ry-Dit game engine. All modules in the ecosystem implement this trait, enabling dynamic loading, execution, and metadata discovery.

## Features

- **RyditModule trait** - Common interface for all engine modules
- **ModuleRegistry** - Dynamic module loading and management
- **ModuleMetadata** - Descriptive metadata for plugin systems
- **ModuleResult/ModuleError** - Standardized error handling
- **JSON-based execution** - Universal command interface via `serde_json::Value`

## Installation

```toml
[dependencies]
ry-core = "0.8.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage

### Implementing a Module

```rust
use ry_core::{RyditModule, ModuleResult, ModuleError};
use serde_json::{json, Value};
use std::collections::HashMap;

struct MyMathModule;

impl RyditModule for MyMathModule {
    fn name(&self) -> &'static str { "math" }
    fn version(&self) -> &'static str { "1.0.0" }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("add", "Add two numbers");
        cmds.insert("multiply", "Multiply two numbers");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "add" => {
                let arr = params.as_array().unwrap();
                let a = arr[0].as_f64().unwrap();
                let b = arr[1].as_f64().unwrap();
                Ok(json!(a + b))
            }
            "multiply" => {
                let arr = params.as_array().unwrap();
                let a = arr[0].as_f64().unwrap();
                let b = arr[1].as_f64().unwrap();
                Ok(json!(a * b))
            }
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Unknown command: {}", command),
            }),
        }
    }
}
```

### Using ModuleRegistry

```rust
use ry_core::{ModuleRegistry, RyditModule};

let mut registry = ModuleRegistry::new();

// Register modules
registry.register(MyMathModule)?;

// Execute commands
let result = registry.execute("math", "add", json!([5.0, 3.0]))?;
assert_eq!(result.as_f64().unwrap(), 8.0);
```

### Module Metadata

```rust
use ry_core::ModuleMetadata;

let metadata = ModuleMetadata::new()
    .with_name("my_module")
    .with_version("1.0.0")
    .with_description("My custom game module")
    .with_license("MIT")
    .with_authors(vec!["Alice", "Bob"]);
```

## API Reference

### RyditModule Trait

```rust
pub trait RyditModule {
    /// Module name (identifier)
    fn name(&self) -> &'static str;
    
    /// Module version
    fn version(&self) -> &'static str;
    
    /// Register available commands
    fn register(&self) -> HashMap<&'static str, &'static str>;
    
    /// Execute a command with JSON parameters
    fn execute(&self, command: &str, params: Value) -> ModuleResult;
    
    /// Optional: Module metadata
    fn metadata(&self) -> ModuleMetadata { ModuleMetadata::default() }
    
    /// Optional: Hot reload hook
    fn on_hot_reload(&self) { }
}
```

### ModuleResult

```rust
pub type ModuleResult = Result<Value, ModuleError>;

pub struct ModuleError {
    pub code: String,
    pub message: String,
}
```

### ModuleRegistry

```rust
pub struct ModuleRegistry {
    // ...
}

impl ModuleRegistry {
    pub fn new() -> Self;
    pub fn register<M: RyditModule>(&mut self, module: M);
    pub fn execute(&self, module: &str, command: &str, params: Value) -> ModuleResult;
    pub fn list_modules(&self) -> Vec<(&str, &str)>;
}
```

## Design Principles

1. **JSON Interface** - All modules use `serde_json::Value` for maximum flexibility
2. **Error Standardization** - `ModuleError` with code + message for consistent error handling
3. **Dynamic Loading** - Registry pattern enables runtime module discovery
4. **Hot Reload Ready** - Optional hooks for live code updates
5. **Zero Dependencies** - Only `serde` and `serde_json` as external crates

## Implementing Modules

The following crates implement `RyditModule`:

| Crate | Module | Description |
|-------|--------|-------------|
| [ry-anim](https://crates.io/crates/ry-anim) | `AnimModule` | Animation system (Disney principles) |
| [ry-physics](https://crates.io/crates/ry-physics) | `PhysicsModule` | Physics simulation |
| [ry-gfx](https://crates.io/crates/ry-gfx) | `GfxModule` | Graphics rendering |
| [ry-lexer](https://crates.io/crates/ry-lexer) | `LexerModule` | Tokenizer for scripting |
| [ry-parser](https://crates.io/crates/ry-parser) | `ParserModule` | AST parser for scripting |

## Performance

- **Zero allocations** in trait methods (static dispatch)
- **9 unit tests** ensuring correctness
- **Minimal dependencies** - only serde + serde_json
- **429 lines** of focused, production-ready code

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `serde` | 1.0 (derive) | Serialization macros |
| `serde_json` | 1.0 | JSON value type |

## Roadmap

- [ ] Async module execution
- [ ] Module dependency resolution
- [ ] Version compatibility checking
- [ ] Module lifecycle management (init, shutdown)
- [ ] Sandbox execution with resource limits

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

**ry-core** - The foundation of Ry-Dit's modular game engine 🔧

*9 tests · 429 lines · RyditModule trait · ModuleRegistry*

</div>
