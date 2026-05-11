// crates/rydit-rs/src/bin/rybot_cli.rs
// RyBot CLI - Herramienta de línea de comandos para debug
// v0.13.0 - IPC vía archivo de status
//
// Uso:
//   rybot_cli status              - Ver estado del sistema
//   rybot_cli inspect modules     - Ver módulos registrados
//   rybot_cli logs --last 10      - Ver últimos eventos
//   rybot_cli alerts              - Ver alertas activas

use std::path::PathBuf;

// Ruta por defecto del archivo de status
fn default_status_path() -> PathBuf {
    let home = std::env::var("HOME")
        .unwrap_or_else(|_| "/data/data/com.termux/files/home".to_string());
    PathBuf::from(home).join(".rybot_status.json")
}

fn main() {
    println!("🛡️ RyBot CLI v0.13.0");
    println!("====================");
    println!();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "status" => cmd_status(),
        "inspect" => cmd_inspect(&args[2..]),
        "logs" => cmd_logs(&args[2..]),
        "alerts" => cmd_alerts(),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            eprintln!("❌ Comando desconocido: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("Uso: rybot_cli <comando> [argumentos]");
    println!();
    println!("Comandos:");
    println!("  status              - Ver estado del sistema");
    println!("  inspect <tipo>      - Inspeccionar módulos/entidades");
    println!("  logs [--last N]     - Ver últimos eventos");
    println!("  alerts              - Ver alertas activas");
    println!("  help                - Mostrar esta ayuda");
    println!();
    println!("Nota: rydit-rs debe estar corriendo para datos en tiempo real.");
    println!("El status se actualiza automáticamente cada 30 frames.");
}

fn cmd_status() {
    let path = default_status_path();
    
    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                println!("{}", content);
                println!("📁 Archivo: {}", path.display());
            }
            Err(e) => {
                eprintln!("❌ Error leyendo status: {}", e);
                show_fallback_status();
            }
        }
    } else {
        println!("⚠️ rydit-rs no ha generado archivo de status aún.");
        println!("   Inicia rydit-rs primero y espera unos segundos.");
        println!();
        show_fallback_status();
    }
}

fn show_fallback_status() {
    println!("=== RyBot Status (fallback) ===");
    println!("Frame: 0");
    println!("Uptime: 0s");
    println!();
    println!("=== Metrics ===");
    println!("FPS: 60.0");
    println!("Frame time: 16.67ms");
    println!("Parse time: 0.8ms");
    println!("Eval time: 2.1ms");
    println!("Render time: 8.3ms");
    println!("Entities: 0");
    println!("Modules: 0");
    println!();
    println!("💡 Tip: Ejecuta rydit-rs para datos reales.");
}

fn cmd_inspect(args: &[String]) {
    if args.is_empty() {
        println!("Uso: inspect <entidad|module|modules>");
        return;
    }

    match args[0].as_str() {
        "modules" | "module" => {
            let path = default_status_path();
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    // Extraer sección de módulos del status
                    if let Some(start) = content.find("=== Módulos ===") {
                        println!("{}", &content[start..]);
                        return;
                    }
                }
            }
            println!("=== Módulos Registrados ===");
            println!("(Ninguno - rydit-rs no está corriendo)");
        }
        "entidad" | "entity" => {
            if args.len() < 2 {
                println!("Uso: inspect entity <id>");
                return;
            }
            println!("Inspeccionando entidad: {}", args[1]);
            println!("(Datos en tiempo real requieren rydit-rs corriendo)");
        }
        _ => {
            eprintln!("❌ Tipo de inspección desconocido: {}", args[0]);
        }
    }
}

fn cmd_logs(args: &[String]) {
    let mut limit = 10;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "--last" || args[i] == "-n" {
            if i + 1 < args.len() {
                if let Ok(n) = args[i + 1].parse() {
                    limit = n;
                }
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    let path = default_status_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Some(start) = content.find("=== Últimos") {
                let logs = &content[start..];
                // Mostrar solo las primeras 'limit' líneas
                let lines: Vec<&str> = logs.lines().take(limit + 5).collect();
                println!("{}", lines.join("\n"));
                return;
            }
        }
    }

    println!("=== Últimos {} eventos ===", limit);
    println!("(Ninguno - rydit-rs no está corriendo)");
}

fn cmd_alerts() {
    let path = default_status_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Some(start) = content.find("=== Alertas ===") {
                println!("{}", &content[start..]);
                return;
            }
        }
    }

    println!("=== Alertas Activas ===");
    println!("(Ninguna - rydit-rs no está corriendo)");
}
