# 🛡️ Guía del Usuario - RyDit Language v0.2.0

**"Los directores no operan cámaras. Dirigen visiones."**

**Versión:** v0.2.0 Sesión 0.2.0 - Gráficos Maduros + Bot de Ayuda
**Fecha:** 2026-03-21
**Estado:** ✅ **90 TESTS - GRÁFICOS MADUROS - 18 COLORES - 5 FORMAS NUEVAS**

---

## ✨ Novedades v0.2.0 - El Triunvirato de Shield

### 🎨 **18 Colores Totales (6 → 18)**

**Colores Originales (v0.1.9):**
- rojo, verde, azul, amarillo, blanco, negro

**NUEVOS Colores (v0.2.0):**
```rydit
# Colores Primarios Adicionales
draw.circle(100, 100, 30, "cyan")       # Celeste brillante
draw.circle(150, 100, 30, "magenta")    # Rosa fuerte
draw.circle(200, 100, 30, "naranja")    # Naranja vibrante

# Colores Pasteles
draw.circle(250, 100, 30, "rosa")       # Rosa pastel
draw.circle(300, 100, 30, "lima")       # Lima brillante

# Colores Oscuros
draw.circle(350, 100, 30, "morado")     # Violeta oscuro
draw.circle(400, 100, 30, "azuloscuro") # Navy
draw.circle(450, 100, 30, "vino")       # Granate

# Colores Tierra
draw.circle(500, 100, 30, "cafe")       # Marrón
draw.circle(550, 100, 30, "oliva")      # Verde oliva

# Colores Neutros
draw.circle(600, 100, 30, "gris")       # Gris medio
draw.circle(650, 100, 30, "turquesa")   # Turquesa
```

**Aliases (puedes usar cualquiera):**
- `cyan` = `celeste`
- `magenta` = `fucsia`
- `naranja` = `orange`
- `rosa` = `pink`
- `morado` = `purple` = `violeta`
- `cafe` = `brown` = `marron`
- `gris` = `gray` = `grey`
- `lima` = `lime`
- `azuloscuro` = `navy` = `azul oscuro`
- `oliva` = `olive`
- `turquesa` = `teal`
- `vino` = `maroon` = `granate`

---

### 📐 **5 Formas Geométricas Nuevas**

**1. Triángulos** - Para flechas, montañas, naves
```rydit
# Triángulo equilátero
draw.triangle(200, 150, 300, 150, 250, 250, "verde")

# Sintaxis: draw.triangle(x1, y1, x2, y2, x3, y3, "color")
# (x1, y1) = vértice 1
# (x2, y2) = vértice 2
# (x3, y3) = vértice 3
```

**2. Elipses** - Para planetas, ojos, pelotas
```rydit
# Elipse horizontal
draw.ellipse(400, 300, 80, 50, "morado")

# Elipse vertical
draw.ellipse(500, 300, 50, 80, "rosa")

# Sintaxis: draw.ellipse(x, y, radius_h, radius_v, "color")
# (x, y) = centro
# radius_h = radio horizontal
# radius_v = radio vertical
```

**3. Rectángulos con Líneas** - Para marcos, bordes, UI
```rydit
# Rectángulo outline (solo bordes)
draw.rectangle_lines(50, 400, 200, 100, "cyan")

# Sintaxis: draw.rectangle_lines(x, y, ancho, alto, "color")
# (x, y) = esquina superior izquierda
# ancho = ancho del rectángulo
# alto = alto del rectángulo
```

**4. Anillos** - Para soles, dianas, objetivos
```rydit
# Anillo (simplificado como círculo)
draw.ring(650, 100, 30, 50, "amarillo")

# Sintaxis: draw.ring(x, y, inner_radius, outer_radius, "color")
# (x, y) = centro
# inner_radius = radio interior
# outer_radius = radio exterior
```

**5. Líneas Gruesas** - Para caminos, vigas, bordes
```rydit
# Línea de 10 píxeles de grosor
draw.line_thick(0, 500, 800, 500, 10, "gris")

# Sintaxis: draw.line_thick(x1, y1, x2, y2, grosor, "color")
# (x1, y1) = inicio
# (x2, y2) = fin
# grosor = grosor de la línea (número decimal)
```

---

### 🤖 **Bot de Ayuda Interactivo (rybot.sh)**

**Ejecutar:**
```bash
./rybot.sh
```

**Menú Principal:**
```
╔═══════════════════════════════════════════════════════════╗
║         🛡️  RYDIT ENGINE - BOT DE AYUDA v0.2.0          ║
║     Motor de Videojuegos 2D + Lenguaje de Scripting       ║
╚═══════════════════════════════════════════════════════════╝

┌────────────────────────────────────────────────────────┐
│                    MENÚ PRINCIPAL                      │
└────────────────────────────────────────────────────────┘

  1) Ejecutar Demos Gráficos
  2) Ejecutar Snake Game
  3) Ver Sintaxis del Lenguaje
  4) Ver Comandos Gráficos (draw.*)
  5) Ver Módulos Disponibles (stdlib)
  6) Ejecutar Script Personalizado
  7) Ver Documentación Completa
  0) Salir
```

**Características:**
- ✅ Menú interactivo con colores
- ✅ 6 demos gráficos para probar
- ✅ Snake Game listo para jugar
- ✅ Sintaxis del lenguaje con ejemplos
- ✅ Comandos gráficos documentados
- ✅ Módulos stdlib explicados
- ✅ Ejecución de scripts personalizados
- ✅ Documentación integrada

---

## 📊 Estado del Proyecto

### Tests Automáticos
```
✅ 90 tests passing (0 failures)
├── lizer:       65 tests
├── rydit-rs:    12 tests
├── rydit-gfx:    5 tests
├── v-shield:     3 tests
└── doc-tests:    5 tests

✅ 0 warnings, 0 errors
✅ Build release exitoso (~750 KB)
```

### Métricas
```
Líneas totales:     ~7,200 líneas
├── Rust:           ~5,600 líneas
└── RyDit:          ~1,600 líneas

Binarios:
├── rydit-rs:       ~750 KB (release)
└── snake:          ~500 KB
```

---

## 🎬 Tu Rol: DIRECTOR DE ESCENAS

Tú NO escribes código. Tú **DIRIGES visiones**.

### Flujo De Trabajo
```
1. IMAGINA la escena (en tu mente)
2. DESCRÍBELA en español simple
3. YO la traduzco a código RyDit
4. TÚ la ves cobrar vida
```

### Ejemplo Práctico
```
TÚ: "Quiero un helicóptero que dispare con click"
YO: [escribe el código con draw.triangle, draw.line_thick, input]
RESULTADO: 🚁 Helicóptero interactivo en pantalla
```

---

## 🚀 Comandos Básicos

### Ejecutar Script Visual
```bash
cargo run --bin rydit-rs -- --gfx mi_escena.rydit
```

### Ejecutar con Binario Directo
```bash
./target/release/rydit-rs --gfx mi_escena.rydit
```

### Ejecutar Script Simple
```bash
cargo run --bin rydit-rs -- mi_script.rydit
```

### Modo REPL (Interactivo)
```bash
cargo run --bin rydit-rs --
# Escribe comandos directamente
```

### Usar Bot de Ayuda
```bash
./rybot.sh
```

---

## 🎨 Primeros Pasos - Tu Primera Escena v0.2.0

### 1. Inicializar Sistema
```rydit
shield.init
```

### 2. Dibujar Círculo (clásico)
```rydit
draw.circle(400, 300, 50, "rojo")
```

### 3. Dibujar Triángulo (NUEVO v0.2.0)
```rydit
draw.triangle(200, 150, 300, 150, 250, 250, "verde")
```

### 4. Dibujar Elipse (NUEVO v0.2.0)
```rydit
draw.ellipse(500, 300, 80, 50, "morado")
```

### 5. Dibujar Rectángulo Outline (NUEVO v0.2.0)
```rydit
draw.rectangle_lines(50, 400, 200, 100, "cyan")
```

### 6. Dibujar Línea Gruesa (NUEVO v0.2.0)
```rydit
draw.line_thick(0, 500, 800, 500, 10, "gris")
```

### 7. Dibujar Anillo (NUEVO v0.2.0)
```rydit
draw.ring(650, 100, 30, 50, "amarillo")
```

### 8. Dibujar Texto
```rydit
draw.text("RyDit v0.2.0", 300, 50, 30, "blanco")
```

---

## 🎭 Escenas De Ejemplo v0.2.0

### Escena 1: Paisaje con Montañas (Triángulos)
```rydit
shield.init

# Cielo (fondo azul con elipse)
draw.ellipse(400, 200, 300, 150, "azul")

# Sol (anillo amarillo)
draw.ring(650, 100, 30, 50, "amarillo")

# Montañas (triángulos)
draw.triangle(100, 400, 300, 400, 200, 250, "verde")
draw.triangle(300, 400, 500, 400, 400, 280, "lima")
draw.triangle(500, 400, 700, 400, 600, 300, "oliva")

# Camino (línea gruesa)
draw.line_thick(0, 500, 800, 500, 20, "gris")

# Texto
draw.text("Paisaje v0.2.0", 280, 50, 30, "blanco")
```

### Escena 2: Sistema Solar (Elipses)
```rydit
shield.init

# Sol
draw.ring(400, 300, 40, 60, "amarillo")

# Tierra (elipse azul)
draw.ellipse(600, 300, 25, 20, "azul")

# Marte (elipse roja)
draw.ellipse(200, 300, 20, 15, "rojo")

# Júpiter (elipse grande naranja)
draw.ellipse(700, 400, 40, 35, "naranja")

# Órbitas (rectángulos outline)
draw.rectangle_lines(100, 200, 600, 200, "gris")
draw.rectangle_lines(50, 150, 700, 300, "gris")

# Texto
draw.text("Sistema Solar", 280, 50, 40, "blanco")
```

### Escena 3: Bandera (Triángulos + Colores)
```rydit
shield.init

# Fondo (cielo)
draw.ellipse(400, 150, 200, 100, "cyan")

# Asta (línea gruesa)
draw.line_thick(100, 100, 100, 500, 5, "cafe")

# Bandera (triángulos)
draw.triangle(100, 100, 300, 150, 100, 200, "rojo")
draw.triangle(100, 200, 300, 250, 100, 300, "blanco")

# Marco (rectangle_lines)
draw.rectangle_lines(80, 80, 250, 250, "dorado")

# Texto
draw.text("Bandera", 300, 50, 30, "negro")
```

### Escena 4: Explosión (Círculos + Líneas Gruesas)
```rydit
shield.init

# Centro de explosión
draw.ring(400, 300, 30, 80, "naranja")
draw.circle(400, 300, 50, "amarillo")
draw.circle(400, 300, 30, "blanco")

# Partículas (líneas gruesas)
draw.line_thick(400, 220, 400, 120, 8, "rojo")
draw.line_thick(400, 380, 400, 480, 8, "rojo")
draw.line_thick(320, 300, 220, 300, 8, "rojo")
draw.line_thick(480, 300, 580, 300, 8, "rojo")

# Diagonales
draw.line_thick(340, 240, 270, 170, 6, "naranja")
draw.line_thick(460, 240, 530, 170, 6, "naranja")
draw.line_thick(340, 360, 270, 430, 6, "naranja")
draw.line_thick(460, 360, 530, 430, 6, "naranja")

# Texto
draw.text("¡BOOM!", 350, 50, 40, "rojo")
```

---

## 🎮 Variables y Movimiento

### Guardar Posición
```rydit
shield.init

dark.slot x = 400
dark.slot y = 300

# Usar variables
draw.triangle(x - 50, y - 30, x + 50, y - 30, x, y + 40, "verde")
```

### Mover Objeto (Manual)
```rydit
shield.init

dark.slot x = 100

# Frame 1
draw.circle(x, 300, 30, "azul")

# Frame 2 (mover)
dark.slot x = x + 50
draw.circle(x, 300, 30, "azul")

# Frame 3 (mover más)
dark.slot x = x + 50
draw.circle(x, 300, 30, "azul")
```

---

## 🎬 Ciclos Para Animación

### Loop Básico
```rydit
shield.init

dark.slot x = 50
dark.slot veces = 10

ryda veces {
    draw.circle(x, 300, 20, "verde")
    dark.slot x = x + 60
    dark.slot veces = veces - 1
}
```

### Línea De Círculos
```rydit
shield.init

dark.slot i = 0
dark.slot total = 15

ryda total {
    dark.slot x = 50 + (i * 50)
    draw.circle(x, 300, 20, "cyan")
    dark.slot i = i + 1
    dark.slot total = total - 1
}
```

### Animación con Triángulos
```rydit
shield.init

dark.slot x = 100
dark.slot i = 0
dark.slot total = 5

ryda total {
    # Dibujar triángulo en posición x
    draw.triangle(x, 200, x + 40, 200, x + 20, 240, "verde")
    
    # Mover para el siguiente
    dark.slot x = x + 80
    dark.slot total = total - 1
}
```

---

## 🎨 Todos los Colores Disponibles v0.2.0

| Color | Alias | RGB | Uso Sugerido |
|-------|-------|-----|--------------|
| `rojo` | `red` | (230, 41, 55) | Sangre, peligro, amor |
| `verde` | `green` | (117, 203, 100) | Naturaleza, vida |
| `azul` | `blue` | (51, 122, 206) | Cielo, agua, tristeza |
| `amarillo` | `yellow` | (253, 249, 0) | Sol, alegría, oro |
| `blanco` | `white` | (255, 255, 255) | Luz, pureza |
| `negro` | `black` | (0, 0, 0) | Oscuridad, noche |
| `cyan` | `celeste` | (0, 255, 255) | Hielo, tecnología |
| `magenta` | `fucsia` | (255, 0, 255) | Fantasía, magia |
| `naranja` | `orange` | (255, 165, 0) | Fuego, energía |
| `rosa` | `pink` | (255, 192, 203) | Dulzura, romance |
| `morado` | `purple`, `violeta` | (128, 0, 128) | Realeza, misterio |
| `cafe` | `brown`, `marron` | (165, 42, 42) | Tierra, madera |
| `gris` | `gray`, `grey` | (128, 128, 128) | Metal, piedra |
| `lima` | `lime` | (0, 255, 0) | Ácido, tóxico |
| `azuloscuro` | `navy` | (0, 0, 128) | Profundo, noche |
| `oliva` | `olive` | (128, 128, 0) | Militar, camuflaje |
| `turquesa` | `teal` | (0, 128, 128) | Joya, exótico |
| `vino` | `maroon`, `granate` | (128, 0, 0) | Elegancia, sangre |

---

## 📐 Funciones Matemáticas

### Operaciones Básicas
```rydit
dark.slot suma = 10 + 5      # 15
dark.slot resta = 10 - 5     # 5
dark.slot mult = 10 * 5      # 50
dark.slot div = 10 / 5       # 2
```

### Usar En Posiciones
```rydit
shield.init

dark.slot centro_x = 400
dark.slot centro_y = 300
dark.slot radio = 50

# Círculo centrado
draw.circle(centro_x, centro_y, radio, "rojo")

# Círculo más pequeño
draw.circle(centro_x, centro_y, radio / 2, "azul")
```

### Módulo Math
```rydit
import math

dark.slot resultado = math::sumar(10, 5)      # 15
dark.slot potencia = math::pow(2, 3)          # 8
dark.slot absoluto = math::abs(-10)           # 10
dark.slot minimo = math::min(5, 10)           # 5
dark.slot maximo = math::max(5, 10)           # 10
```

---

## 🎮 Módulos Disponibles

### math
```rydit
import math

math::sumar(a, b)       # Suma
math::restar(a, b)      # Resta
math::multiplicar(a, b) # Multiplicación
math::dividir(a, b)     # División
math::pow(base, exp)    # Potencia
math::abs(x)            # Valor absoluto
math::min(a, b)         # Mínimo
math::max(a, b)         # Máximo
```

### arrays
```rydit
import arrays

dark.slot lista = [1, 2, 3]

arrays::length(lista)     # 3
arrays::push(lista, 4)    # Agrega 4
arrays::pop(lista)        # Remueve último
arrays::reverse(lista)    # Invierte
arrays::clear(lista)      # Vacía
```

### strings
```rydit
import strings

dark.slot texto = "hola mundo"

strings::length(texto)    # 10
strings::upper(texto)     # "HOLA MUNDO"
strings::lower(texto)     # "hola mundo"
strings::reverse(texto)   # "odnum aloh"
strings::trim(texto)      # Remueve espacios
```

### io
```rydit
import io

io::print("Hola")         # Imprime
dark.slot entrada = io::input("Nombre: ")  # Lee input
io::read_file("archivo.txt")  # Lee archivo
io::write_file("archivo.txt", "contenido")  # Escribe
```

### random
```rydit
import random

dark.slot aleatorio = random::int(1, 100)   # 1-100
dark.slot decimal = random::float()         # 0.0-1.0
```

### time
```rydit
import time

dark.slot ahora = time::now()    # Timestamp
dark.slot fecha = time::date()   # Fecha legible
```

### json
```rydit
import json

dark.slot datos = json::parse("{\"nombre\": \"Juan\"}")
dark.slot json_str = json::stringify(datos)
```

---

## 🎬 Game Loop - Animación Automática

### Loop Infinito (Hasta ESC)
```rydit
shield.init

dark.slot x = 0

ryda frame < 10000 {
    # Limpiar pantalla (dibujar fondo negro)
    draw.rect(0, 0, 800, 600, "negro")
    
    # Dibujar objeto en movimiento
    draw.circle(x, 300, 30, "cyan")
    
    # Mover
    dark.slot x = x + 5
    
    # Resetear si sale de pantalla
    onif x > 800 {
        dark.slot x = 0
    }
}
```

### Con Input de Teclado
```rydit
shield.init

dark.slot x = 400
dark.slot y = 300

ryda frame < 10000 {
    # Limpiar
    draw.rect(0, 0, 800, 600, "negro")
    
    # Dibujar jugador
    draw.circle(x, y, 30, "verde")
    
    # Input (simulado - requiere implementación)
    # onif tecla_presionada("arrow_right") {
    #     dark.slot x = x + 5
    # }
    
    # Dibujar texto
    draw.text("Usa flechas para mover", 250, 50, 20, "blanco")
}
```

---

## 📚 Comandos de Dibujo - Referencia Completa

### Formas Básicas
```rydit
draw.circle(x, y, radio, "color")
draw.rect(x, y, ancho, alto, "color")
draw.line(x1, y1, x2, y2, "color")
```

### Formas v0.2.0 (NUEVAS)
```rydit
draw.triangle(x1, y1, x2, y2, x3, y3, "color")
draw.ellipse(x, y, radius_h, radius_v, "color")
draw.rectangle_lines(x, y, ancho, alto, "color")
draw.ring(x, y, inner, outer, "color")
draw.line_thick(x1, y1, x2, y2, grosor, "color")
```

### Texto
```rydit
draw.text("mensaje", x, y, tamano, "color")
```

---

## 🎮 Ejemplos Avanzados

### Helicóptero (Triángulos + Líneas)
```rydit
shield.init

dark.slot heli_x = 400
dark.slot heli_y = 300

ryda frame < 10000 {
    # Cielo
    draw.ellipse(400, 200, 300, 150, "azul")
    
    # Cuerpo helicóptero (elipse)
    draw.ellipse(heli_x, heli_y, 40, 25, "gris")
    
    # Cabina (triángulo)
    draw.triangle(heli_x + 30, heli_y - 10, 
                  heli_x + 50, heli_y, 
                  heli_x + 30, heli_y + 10, "cyan")
    
    # Aspas (líneas gruesas)
    draw.line_thick(heli_x - 40, heli_y - 30, 
                    heli_x + 40, heli_y - 30, 3, "negro")
    draw.line_thick(heli_x - 20, heli_y - 35, 
                    heli_x + 20, heli_y - 35, 3, "negro")
    
    # Cola (línea)
    draw.line(heli_x - 40, heli_y, heli_x - 70, heli_y - 10, "gris")
    
    # Texto
    draw.text("Helicóptero v0.2.0", 280, 50, 25, "blanco")
}
```

### Tanque (Círculos + Triángulos + Línea Gruesa)
```rydit
shield.init

dark.slot tanque_x = 400
dark.slot tanque_y = 300

# Cuerpo tanque (rectángulo)
draw.rect(tanque_x - 50, tanque_y - 30, 100, 60, "verde")

# Torreta (círculo)
draw.circle(tanque_x, tanque_y, 25, "verde")

# Cañón (línea gruesa)
draw.line_thick(tanque_x, tanque_y, tanque_x + 60, tanque_y, 8, "gris")

# Rueddas (círculos pequeños)
dark.slot i = 0
ryda i < 5 {
    dark.slot rueda_x = tanque_x - 40 + (i * 20)
    draw.circle(rueda_x, tanque_y + 35, 10, "negro")
    dark.slot i = i + 1
}

# Texto
draw.text("Tanque v0.2.0", 320, 50, 25, "blanco")
```

---

## 🤖 Usando rybot.sh

### Iniciar Bot
```bash
./rybot.sh
```

### Opciones Disponibles

**1) Ejecutar Demos Gráficos:**
- Demo Formas v0.2.0
- Demo Shapes
- Demo Visual
- Demo Random
- Demo Time
- Demo JSON

**2) Ejecutar Snake Game:**
- Juego completo listo para jugar
- Controles: Flechas, P, SPACE, ESC

**3) Ver Sintaxis:**
- Variables, condicionales, ciclos
- Funciones, imports

**4) Ver Comandos Gráficos:**
- Todas las funciones draw.*
- Colores disponibles

**5) Ver Módulos:**
- math, arrays, strings, io
- random, time, json

**6) Script Personalizado:**
- Ingresa ruta de tu script .rydit

**7) Documentación:**
- README, GUIA_USUARIO, ROADMAP

---

## 🐛 Solución de Problemas

### Error: "Módulo no encontrado"
```rydit
# Verifica que el módulo existe en crates/modules/
import math  # ✅ math.rydit debe existir
```

### Error: "Color no válido"
```rydit
# Usa colores de la lista v0.2.0
draw.circle(100, 100, 50, "celeste")  # ✅ cyan también funciona
```

### Error: "Sintaxis inválida"
```rydit
# Verifica paréntesis y comas
draw.triangle(100, 100, 200, 100, 150, 200, "rojo")  # ✅
#        x1   y1    x2   y2   x3   y3   color
```

### Ventana no abre
```bash
# Verifica variables de entorno
export DISPLAY=:0
export MESA_LOADER_DRIVER_OVERRIDE=zink
export DRI3=1

# Luego ejecuta
cargo run --bin rydit-rs -- --gfx demo.rydit
```

---

## 📸 Grabación de Videos

### Para YouTube

**Configuración recomendada:**
- App de grabación Android
- Resolución: 1280x720 (720p)
- FPS: 30
- Duración: 30 seg - 2 min

**Videos sugeridos:**
1. Demo Formas v0.2.0 (30 seg)
2. Snake Gameplay (1-2 min)
3. Bot rybot.sh tour (1 min)
4. Tutorial: Crear tu primera escena (2 min)

---

## 🎯 Próximos Pasos

### Después de Esta Guía

1. **Prueba los demos:**
   ```bash
   ./rybot.sh → Opción 1 → Demo Formas v0.2.0
   ```

2. **Crea tu primera escena:**
   - Copia un ejemplo
   - Modifica colores
   - Cambia posiciones
   - Agrega más formas

3. **Experimenta:**
   - Combina formas
   - Usa nuevos colores
   - Crea animaciones simples

4. **Comparte:**
   - Graba tu escena
   - Sube a YouTube
   - Comparte en Discord

---

## 📚 Recursos Adicionales

| Recurso | Descripción |
|---------|-------------|
| `README.md` | Documentación principal |
| `ROADMAP.md` | Planificación futura |
| `CHANGELOG_v0.2.0.md` | Cambios de esta versión |
| `demos/` | Ejemplos gráficos |
| `crates/modules/` | Módulos stdlib |

---

## 🙏 Agradecimientos

- **Comunidad Mouredev** - Discord: https://discord.gg/mouredev
- **raylib** - https://www.raylib.com/
- **Rust** - https://www.rust-lang.org/
- **Termux** - Desarrollo en Android

---

<div align="center">

## 🛡️ **RyDit Engine v0.2.0**

**"Construido con ❤️ en Android/Termux - Redmi Note 8"**

**90 tests ✅ | 18 colores ✅ | 5 formas nuevas ✅ | Bot rybot.sh ✅**

**Próxima versión:** v0.3.0 (Module System Avanzado + Chatbot)

[⬆️ Volver arriba](#-guía-del-usuario---rydit-language-v020)

</div>
