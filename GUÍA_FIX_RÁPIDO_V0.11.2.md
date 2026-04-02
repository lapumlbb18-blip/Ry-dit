# 🛡️ GUÍA RÁPIDA DE FIX MANUAL - v0.11.2

**Para usar DURANTE el fix - Tener abierta mientras editas**

---

## 🔥 RESUMEN ULTRA-RÁPIDO

| Error | Líneas | Fix | Tiempo |
|-------|--------|-----|--------|
| **E0425** (name/func_name) | 6 líneas | Cambiar nombre | 5 min |
| **E0277** (&str vs str) | 13 líneas | Agregar `*` o `&` | 10 min |
| **E0433** (Lizer) | 4 líneas | Agregar import | 5 min |
| **E0658** (as_str) | 3 líneas | Quitar `.as_str()` | 5 min |
| **E0308** (&str/String) | ~20 líneas | `.to_string()` | 30 min |
| **E0308** (parser.parse) | 6 líneas | Tupla en vez de Result | 30 min |
| **E0599** (into_string) | 3 líneas | `.expect().into_string()` | 20 min |
| **E0026/E0027** (Stmt::Call) | 4 patrones | `callee` en vez de `name` | 2 horas |

**Total**: 76 errores en ~4 horas (con esta guía)

---

## 📝 COMANDOS DE VERIFICACIÓN

### Antes de empezar:
```bash
# Contar errores actuales
cargo build -p rydit-rs --bin rydit-rs 2>&1 | grep -E "^error\[E[0-9]+\]" | wc -l
# Debería decir: 76
```

### Después de cada fix:
```bash
# Verificar progreso
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep -E "^error\[E[0-9]+\]" | wc -l
# Debería disminuir
```

### Ver errores específicos:
```bash
# Ver solo E0425
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0425"

# Ver solo E0308
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "E0308"

# Ver errores en archivo específico
cargo check -p rydit-rs --bin rydit-rs 2>&1 | grep "main.rs"
```

---

## 🟢 FASE 1: FIXES SEGUROS (30 minutos)

### 1.1 Imports faltantes (5 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Agregar en línea ~50**:
```rust
use rydit_lexer::Lizer;
```

**Archivo**: `crates/rydit-rs/src/repl.rs`

**Agregar en línea ~9**:
```rust
use rydit_lexer::Lizer;
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0433"  # Debería ser 0
```

---

### 1.2 name vs func_name (10 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Cambiar `name` → `func_name`**:
```bash
# Línea 309
# Línea 1739
# Línea 3057
```

**Cambiar `func_name` → `name`**:
```bash
# Línea 2187
# Línea 3168
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0425"  # Debería ser 0
```

---

### 1.3 Comparaciones &str (10 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Líneas 1268-1278** (10 comparaciones):
```rust
// Cambiar TODOS los `name ==` por `*name ==`
if *name == "x"
    || *name == "y"
    || *name == "velocidad"
    // ... etc (10 en total)
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0277"  # Debería bajar a ~3
```

---

### 1.4 as_str() inestable (5 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Líneas 345, 1789, 4483**:
```rust
// ANTES
alias_name.as_str()

// DESPUÉS
alias_name  // (ya es &str)
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0658"  # Debería ser 0
```

---

## 🟡 FASE 2: FIXES MEDIOS (1.5 horas)

### 2.1 Type mismatch simples (30 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Líneas**: 334, 392, 411, 434, 1849, 4525

```rust
// Cambiar
original_funcs.push(name);

// Por
original_funcs.push(name.to_string());
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0308" | wc -l  # Debería bajar a ~25
```

---

### 2.2 parser.parse() API (30 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Líneas**: 399-400, 1837-1838, 4514-4515

```rust
// ANTES
let program = match parser.parse() {
    Ok(p) => p,
    Err(e) => {
        println!("[ERROR] {}", e);

// DESPUÉS
let (program, errors) = parser.parse();
if !errors.is_empty() {
    for e in &errors {
        println!("[WARNING] {}", e);
    }
}
// Usar program directamente
```

**Archivo**: `crates/rydit-rs/src/repl.rs`

**Líneas 72, 85**:
```rust
// ANTES
match parser.parse() {
    Ok(program) => { ... }
    Err(e) => { ... }

// DESPUÉS
let (program, errors) = parser.parse();
if !errors.is_empty() {
    for e in &errors {
        println!("[ERROR] {}", e);
    }
}
// Usar program directamente
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0308" | wc -l  # Debería bajar a ~15
```

---

### 2.3 HTTP functions (30 min)

**Archivo**: `crates/rydit-rs/src/eval/mod.rs`

**Línea 1399** (http::get):
```rust
// ANTES
return match ureq::get(&url).call().into_string() {
    Ok(response) => Valor::Texto(response),
    Err(e) => Valor::Error(e),

// DESPUÉS
return match ureq::get(&url).call() {
    Ok(response) => match response.into_string() {
        Ok(text) => Valor::Texto(text),
        Err(e) => Valor::Error(e.to_string()),
    },
    Err(e) => Valor::Error(e.to_string()),
};
```

**Líneas 1423-1427** (http::post):
```rust
// ANTES
return match ureq::post(&url).send_string(&data)
    .map(|r| r.into_string())
    .unwrap_or(Err("POST error".to_string())) {
    Ok(response) => Valor::Texto(response),
    Err(e) => Valor::Error(e),

// DESPUÉS
return match ureq::post(&url).send_string(&data) {
    Ok(r) => match r.into_string() {
        Ok(text) => Valor::Texto(text),
        Err(e) => Valor::Error(e.to_string()),
    },
    Err(e) => Valor::Error(e.to_string()),
};
```

**Líneas 1445-1449** (http::put) - MISMO FIX que post

**Línea 1459** (http::delete) - MISMO FIX que get

**Verificar**:
```bash
cargo check 2>&1 | grep "E0599"  # Debería ser 0
```

---

### 2.4 Function registration (30 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Líneas**: 244-245, 1448, 4457

```rust
// ANTES
funcs.insert(name.clone(), (params.clone(), body.clone()));

// DESPUÉS
funcs.insert(
    name.to_string(),
    (params.iter().map(|s| s.to_string()).collect(), body.clone())
);
```

**Archivo**: `crates/rydit-rs/src/executor.rs`

**Línea 424** - MISMO FIX

**Verificar**:
```bash
cargo check 2>&1 | grep "E0308" | wc -l  # Debería bajar a ~10
```

---

## 🔴 FASE 3: FIXES CRÍTICOS (2 horas)

### 3.1 Stmt::Call pattern (1.5 horas) 🔴

**ENTENDER ESTO PRIMERO**:

El AST cambió:
```rust
// AST VIEJO (ROTO)
Stmt::Call {
    callee: Box<Expr<'a>>,  // Era una expresión
    args: Vec<Expr<'a>>,
}

// AST NUEVO (CORRECTO)
Stmt::Call {
    callee: &'a str,  // AHORA ES &str DIRECTO
    args: Vec<Expr<'a>>,
}
```

**Archivo**: `crates/rydit-rs/src/main.rs`

**Línea 246-254** (`ejecutar_stmt()`):
```rust
// ANTES (ROTO)
Stmt::Call { callee, args } => {
    let func_name = if let Expr::Var(name) = callee.as_ref() {
        *name
    } else {
        println!("[WARNING] Call requiere función válida");
        return (false, None);
    };

// DESPUÉS (CORRECTO)
Stmt::Call { callee, args } => {
    let func_name = callee;  // ¡DIRECTO! callee YA ES &str
    
    // Continuar con el resto del código...
```

**Línea 1448-1454** (`ejecutar_stmt_gfx()`):
```rust
// ANTES
Stmt::Call { callee, args } => {
    let func_name = if let Expr::Var(name) = callee.as_ref() {
        *name

// DESPUÉS
Stmt::Call { callee, args } => {
    let func_name = callee;  // Directo
```

**Línea 4459-4473** (`ejecutar_stmt_migui()`):
```rust
// ANTES
Stmt::Call { name, args } => {  // ← ERROR: campo es callee, no name
    let _ = evaluar_expr_migui(
        &Expr::Call { callee: Box::new(Expr::Var(func_name)),

// DESPUÉS
Stmt::Call { callee, args } => {
    let func_name = callee;  // Usar callee
    let _ = evaluar_expr_migui(
        &Expr::Call { callee: Box::new(Expr::Var(func_name)),
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0026\|E0027"  # Debería ser 0
```

---

### 3.2 Expr::BinOp → Expr::Binary (15 min)

**Archivo**: `crates/rydit-rs/src/main.rs`

**Línea 4075**:
```rust
// ANTES
Expr::BinOp { left, op, right } => {

// DESPUÉS
Expr::Binary { left, op, right } => {
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0599"  # Debería ser 0
```

---

### 3.3 Expr::Texto type mismatch (15 min)

**Archivo**: `crates/rydit-rs/src/eval/mod.rs`

**Línea 54**:
```rust
// ANTES
Expr::Texto(s) => Valor::Texto(s.clone()),

// DESPUÉS
Expr::Texto(s) => Valor::Texto(s.to_string()),
```

**Línea 57**:
```rust
// ANTES
if name == "__INPUT__" {

// DESPUÉS
if *name == "__INPUT__" {
```

**Verificar**:
```bash
cargo check 2>&1 | grep "E0308" | wc -l  # Debería ser 0
```

---

## ✅ VERIFICACIÓN FINAL

### Compilación:
```bash
cargo build -p rydit-rs --bin rydit-rs
```

**Si funciona**:
```
Finished release [optimized] target(s)
```

**Si hay errores restantes**:
```bash
# Ver cuáles quedan
cargo build -p rydit-rs --bin rydit-rs 2>&1 | grep "^error"
```

### Ejecución:
```bash
# Probar demo simple
./target/release/rydit-rs --gfx demos/test_simple.rydit

# O REPL
./target/release/rydit-rs --repl
```

---

## 🚨 EMERGENCIA

### Si compilas y crashea:
```bash
# Verificar que SDL2 esté bien
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Ejecutar con logging
RUST_LOG=debug ./target/release/rydit-rs --gfx demos/test_simple.rydit
```

### Si hay pánico:
```bash
# Backtrace completo
export RUST_BACKTRACE=full
./target/release/rydit-rs --gfx demos/test_simple.rydit
```

### Revertir todo:
```bash
git checkout crates/rydit-rs/src/
cargo build -p rydit-rs --bin rydit-rs
# Vuelve a 76 errores
```

---

## 📊 PROGRESO ESPERADO

| Después de | Errores | % Completado |
|------------|---------|--------------|
| Inicio | 76 | 0% |
| FASE 1 | ~30 | 60% |
| FASE 2 | ~10 | 87% |
| FASE 3 | 0 | 100% ✅ |

---

<div align="center">

**🛡️ RyDit v0.11.2 - GUÍA RÁPIDA**

*76 → 0 errores en ~4 horas*

**¡TÚ PUEDES! Fix manual es más seguro**

</div>
