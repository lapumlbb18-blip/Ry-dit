# 🛡️ Ry-Dit - Guía del Parser

**Versión**: v0.12.0
**Última actualización**: 2026-04-04

---

## 📋 Tabla de Contenidos

1. [Arquitectura del Parser](#arquitectura-del-parser)
2. [Flujo Completo de Parsing](#flujo-completo-de-parsing)
3. [TokenKind Disponibles](#tokenkind-disponibles)
4. [Statements Soportados](#statements-soportados)
5. [Expresiones Soportadas](#expresiones-soportadas)
6. [Sintaxis de .rydit](#sintaxis-de-rydit)
7. [Patrones Correctos de Parsing](#patrones-correctos-de-parsing)
8. [Errores Comunes y Cómo Evitarlos](#errores-comunes-y-cómo-evitarlos)
9. [Testing del Parser](#testing-del-parser)

---

## Arquitectura del Parser

El sistema de parsing de Ry-Dit consta de 3 componentes:

```
crates/ry-lexer/     → Tokeniza source → Vec<Token>
crates/ry-parser/    → Parsea tokens → (Program, Errors)
crates/ry-rs/eval/   → Ejecuta AST → Valores + DrawCommands
```

### Zero-Copy con Lifetimes

```rust
// Los tokens referencian el source original, NO copian
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,  // ← Referencia al source
    pub span: Span,
}
```

---

## Flujo Completo de Parsing

```
# demo.rydit
dark.slot x = 400
dibujar.circulo(x, 300, 30, "rojo")
```

### Paso 1: Lexer

```rust
let source = fs::read_to_string("demo.rydit")?;
let tokens = ry_lexer::Lexer::new(&source).scan();
// tokens: Vec<Token> [DarkSlot, Ident("x"), Asignar, Num(400), ...]
```

### Paso 2: Parser

```rust
let (program, errors) = ry_parser::Parser::new(tokens).parse();
// program: Program { statements: [Assign, DrawCircle] }
// errors: Vec<RyDitError> (vacío si no hay errores)
```

### Paso 3: Evaluación

```rust
for stmt in program.statements {
    ejecutar_stmt(stmt, &mut executor);
}
```

---

## TokenKind Disponibles

### Keywords del Lenguaje

| TokenKind | Lexemas | Descripción |
|-----------|---------|-------------|
| `DarkSlot` | `dark.slot` | Declaración de variable |
| `Ryda` | `ryda` | Bucle while |
| `Onif` | `onif` | Condicional if |
| `Blelse` | `blelse` | Condicional else |
| `Cada` | `cada` | Foreach |
| `En` | `en` | Keyword "en" |
| `Rytmo` | `rytmo` | Definición de función |
| `Return` | `return` | Retorno |
| `Voz` | `voz` | Print |
| `Break` | `break`, `romper` | Romper bucle |
| `TextoKw` | `texto` | Keyword para draw text |

### Operadores

| TokenKind | Lexema | Descripción |
|-----------|--------|-------------|
| `Asignar` | `=` | Asignación |
| `Igual` | `==` | Comparación igualdad |
| `Mayor` | `>` | Mayor que |
| `Menor` | `<` | Menor que |
| `MayorIgual` | `>=` | Mayor o igual |
| `MenorIgual` | `<=` | Menor o igual |
| `Diferente` | `!=` | Diferente |
| `Mas` | `+` | Suma |
| `Menos` | `-` | Resta |
| `Por` | `*` | Multiplicación |
| `Div` | `/` | División |
| `MasIgual` | `+=` | Suma y asigna |
| `MenosIgual` | `-=` | Resta y asigna |
| `PorIgual` | `*=` | Multiplica y asigna |
| `DivIgual` | `/=` | Divide y asigna |
| `And` | `and` | AND lógico |
| `Or` | `or` | OR lógico |
| `Not` | `not` | NOT lógico |

### Delimitadores

| TokenKind | Lexema |
|-----------|--------|
| `LlaveIzq` | `{` |
| `LlaveDer` | `}` |
| `ParentIzq` | `(` |
| `ParentDer` | `)` |
| `CorcheteIzq` | `[` |
| `CorcheteDer` | `]` |
| `Punto` | `.` |
| `Coma` | `,` |

### Literales

| TokenKind | Ejemplo |
|-----------|---------|
| `Ident` | `x`, `frame`, `mi_var` |
| `Num` | `42`, `3.14`, `-7` |
| `Texto` | `"hola"`, `'mundo'` |

### Draw Commands

| TokenKind | Lexemas |
|-----------|---------|
| `DrawCircle` | `draw.circle`, `dibujar.circulo` |
| `DrawRect` | `draw.rect`, `dibujar.rect` |
| `DrawLine` | `draw.line`, `dibujar.linea` |
| `DrawText` | `draw.text`, `dibujar.texto` |
| `DrawTriangle` | `draw.triangle`, `dibujar.triangulo` |
| `DrawRing` | `draw.ring`, `dibujar.anillo` |
| `DrawRectangleLines` | `draw.rectangle_lines`, `dibujar.rect_lineas` |
| `DrawEllipse` | `draw.ellipse`, `dibujar.elipse` |
| `DrawLineThick` | `draw.line_thick`, `dibujar.linea_gruesa` |

### Especiales

| TokenKind | Descripción |
|-----------|-------------|
| `Comentario` | `# comentario` |
| `Eof` | Fin de archivo |
| `Error` | Token inválido |

---

## Statements Soportados

### Declaración de Variables

```rydit
# Variable simple
dark.slot x = 400

# Array
dark.slot[] colores = ["rojo", "verde", "azul"]
```

### Bucle While

```rydit
# Con condición
ryda frame < 1000 {
    dibujar.circulo(x, y, 30, "rojo")
    frame = frame + 1
}

# Sin condición (loop infinito)
ryda {
    dibujar.circulo(x, y, 30, "rojo")
    onif tecla_presionada("escape") {
        romper
    }
}
```

### Condicional

```rydit
onif tecla_presionada("escape") {
    romper
}

onif x > 100 {
    dibujar.circulo(x, y, 30, "rojo")
} blelse {
    dibujar.rect(x, y, 60, 60, "azul")
}
```

### Draw Commands

```rydit
# Sintaxis con paréntesis
dibujar.circulo(400, 300, 30, "rojo")
dibujar.rect(100, 100, 80, 60, "verde")
dibujar.linea(0, 0, 800, 600, "blanco")
dibujar.texto("Hola", 60, 60, 20, "blanco")

# Sintaxis texto con 'en'
texto "Hola Ry-Dit" en 60, 60, tamano 20, color "blanco"
texto "FPS: " + fps() en 60, 80, tamano 14, color "gris"
```

### Asignación

```rydit
# Variable simple
x = 400

# Operación
frame = frame + 1
velocidad = velocidad * 2

# Indexación
colores[0] = "amarillo"
```

### Foreach

```rydit
cada color en colores {
    dibujar.circulo(x, y, 20, color)
    x = x + 50
}
```

### Funciones

```rydit
rytmo mi_funcion(a, b) {
    return a + b
}
```

### Import

```rydit
# Se maneja en el runtime
```

### Break

```rydit
ryda {
    onif condicion {
        romper
    }
}
```

---

## Expresiones Soportadas

### Literales

```rydit
42           # Num
3.14         # Num decimal
"hola"       # Texto
```

### Variables

```rydit
x            # Ident
mi_variable  # Ident con guion bajo
```

### Operaciones Binarias

```rydit
x + y        # Suma
x - y        # Resta
x * y        # Multiplicación
x / y        # División
x == y       # Igualdad
x != y       # Diferente
x > y        # Mayor que
x < y        # Menor que
x >= y       # Mayor o igual
x <= y       # Menor o igual
x and y      # AND lógico
x or y       # OR lógico
```

### Operaciones Unarias

```rydit
not x        # NOT lógico
-x           # Negación
```

### Concatenación de Strings

```rydit
"Hola " + nombre    # Concatenación
"FPS: " + fps()     # Con función builtin
```

### Array Literals

```rydit
[1, 2, 3]                    # Array de números
["rojo", "verde", "azul"]    # Array de strings
[x, y, z]                    # Array de variables
```

### Indexación

```rydit
colores[0]                   # Acceso por índice
colores[i]                   # Acceso por variable
```

### Llamadas a Función

```rydit
tecla_presionada("escape")   # Función de input
fps()                        # Función builtin (60.0)
sumar(1, 2, 3)               # Función aritmética
```

### Namespace matematica::

```rydit
matematica::sin(angulo)      # Seno
matematica::cos(angulo)      # Coseno
matematica::tan(angulo)      # Tangente
matematica::sqrt(x)          # Raíz cuadrada
matematica::floor(x)         # Redondear abajo
matematica::ceil(x)          # Redondear arriba
matematica::abs(x)           # Valor absoluto
```

---

## Sintaxis de .rydit

### Estructura Básica

```rydit
# Comentario (opcional)
# Los comentarios empiezan con # y van hasta el final de línea

# Declaraciones
dark.slot x = 400
dark.slot y = 300
dark.slot frame = 0
dark.slot[] colores = ["rojo", "verde", "azul"]

# Game loop
ryda {
    # Draw commands
    dibujar.circulo(x, y, 30, "rojo")
    texto "Frame: " + frame en 60, 60, tamano 16, color "blanco"

    # Input
    onif tecla_presionada("escape") {
        romper
    }

    # Lógica
    frame = frame + 1
}
```

### Reglas de Sintaxis

1. **Comentarios**: `# texto` → hasta fin de línea
2. **Variables**: `dark.slot nombre = valor` o `dark.slot[] nombre = [array]`
3. **Bloques**: `{ ... }` → siempre con llaves
4. **Indentación**: Opcional, no afecta parsing
5. **Punto y coma**: NO necesario, cada línea es un statement
6. **Case sensitive**: `ryda` ≠ `Ryda`

---

## Patrones Correctos de Parsing

### Patrón 1: Consumir antes de delegar

```rust
// ✅ CORRECTO
TokenKind::Ident => {
    if let Some(name) = self.current().as_ident() {
        self.advance(); // ← CONSUMIR antes
        return self.parse_call_or_ident(name);
    }
    None
}

// ❌ INCORRECTO (causó error persistente)
TokenKind::Ident => {
    if let Some(name) = self.current().as_ident() {
        return self.parse_call_or_ident(name); // ← NO consume
    }
    None
}
```

### Patrón 2: Usar parse_expression() para valores

```rust
// ✅ CORRECTO
let texto = self.parse_expression()?;  // Soporta "X" + var

// ❌ INCORRECTO (causó error)
let texto = if self.check(TokenKind::Texto) {
    let lexeme = self.current().lexeme.trim_matches('"');
    self.advance();
    Expr::Texto(lexeme)  // ← Solo string literal, ignora "+ var"
} else {
    self.parse_expression()?
};
```

### Patrón 3: Separar operadores diferentes

```rust
// ✅ CORRECTO
TokenKind::Asignar  // =
TokenKind::Igual    // ==

// ❌ INCORRECTO
TokenKind::Igual  // "=" y "==" (ambos)
```

### Patrón 4: Comas opcionales

```rust
// ✅ CORRECTO
fn parse_optional_comma(&mut self) {
    if self.check(TokenKind::Coma) {
        self.advance();
    }
}
```

---

## Errores Comunes y Cómo Evitarlos

### Error 1: Token no consumido antes de delegar

**Síntoma**: "Se esperaba X, se encontró Y" donde Y es el token actual.

**Causa**: No se hizo `self.advance()` antes de llamar otra función de parsing.

**Solución**: Siempre consumir el token antes de delegar.

### Error 2: Operadores compartiendo TokenKind

**Síntoma**: El parser confunde asignación con comparación.

**Causa**: `=` y `==` comparten `TokenKind::Igual`.

**Solución**: Crear tokens separados (`Asignar` vs `Igual`).

### Error 3: Shortcuts en parsing de expresiones

**Síntoma**: `"X" + var` falla, solo funciona `"X"`.

**Causa**: El parser toma un shortcut para literales y no parsea la expresión completa.

**Solución**: Siempre usar `parse_expression()`.

### Error 4: Tests con nombres de AST viejos

**Síntoma**: 151 errores de compilación en tests.

**Causa**: Los tests usan `Expr::BinOp` pero el AST usa `Expr::Binary`.

**Solución**: Mover tests a `docs/tests_referencia/` y crear nuevos.

---

## Testing del Parser

### Test Binario

```bash
cargo run --bin test_parser -- demos/demo.rydit
```

### Output esperado

```
🛡️ Ry-Dit v0.12.0 - Test de Parser
====================================

📄 Archivo: demos/demo.rydit

✅ Lectura exitosa (502 bytes)

🔍 Fase 1: Lexer...
✅ 85 tokens generados

🔍 Fase 2: Parser...
✅ Parsing exitoso
   - 5 statements en programa

🏁 Test completado
```

### Crear nuevo test

1. Crear archivo `.rydit` en `demos/`
2. Ejecutar: `cargo run --bin test_parser -- demos/test.rydit`
3. Verificar: ✅ Parsing exitoso

### Ejemplo de test mínimo

```rydit
# test_minimo.rydit
dark.slot x = 400
dark.slot y = 300

dibujar.circulo(x, y, 30, "rojo")
dibujar.rect(100, 100, 80, 60, "verde")
```

Resultado esperado: ✅ 4 statements

---

<div align="center">

**🛡️ Ry-Dit v0.12.0 - Guía del Parser**

*Parser funcional | 0 errores | 95% features completas*

</div>
