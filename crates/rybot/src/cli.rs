//! Rybot CLI — Interfaz de línea de comandos
//!
//! ```bash
//! rybot new mi_juego --template game2d
//! rybot new mi_3d -t game3d
//! rybot templates
//! rybot info
//! rybot help
//! ```

use crate::templates::{ProjectTemplate, create_project, list_templates};

#[derive(Debug)]
pub enum CliCommand {
    New { name: String, template: ProjectTemplate, dir: Option<String> },
    ListTemplates,
    Info,
    Help,
    Version,
}

pub fn parse_args(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Ok(CliCommand::Help);
    }

    match args[0].as_str() {
        "new" | "create" => parse_new(args),
        "templates" | "list" | "ls" => Ok(CliCommand::ListTemplates),
        "info" | "status" => Ok(CliCommand::Info),
        "help" | "--help" | "-h" => Ok(CliCommand::Help),
        "version" | "--version" | "-v" => Ok(CliCommand::Version),
        other => Err(format!("Comando desconocido: '{}'. Usa 'rybot help'.", other)),
    }
}

fn parse_new(args: &[String]) -> Result<CliCommand, String> {
    let mut name: Option<String> = None;
    let mut template = ProjectTemplate::Game2D;
    let mut dir: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--template" | "-t" => {
                i += 1;
                if i >= args.len() { return Err("Falta valor después de --template".into()); }
                template = parse_template(&args[i])?;
            }
            "--dir" | "-d" => {
                i += 1;
                if i >= args.len() { return Err("Falta valor después de --dir".into()); }
                dir = Some(args[i].clone());
            }
            f if f.starts_with('-') => return Err(format!("Flag desconocida: '{}'", f)),
            val => name = Some(val.to_string()),
        }
        i += 1;
    }

    let name = name.ok_or("Falta nombre del proyecto. Uso: rybot new mi_proyecto")?;
    Ok(CliCommand::New { name, template, dir })
}

fn parse_template(s: &str) -> Result<ProjectTemplate, String> {
    match s {
        "game2d" | "2d" => Ok(ProjectTemplate::Game2D),
        "game3d" | "3d" => Ok(ProjectTemplate::Game3D),
        "gui" | "gui_app" => Ok(ProjectTemplate::GuiApp),
        "animation" | "anim" => Ok(ProjectTemplate::Animation),
        "science" | "ciencia" => Ok(ProjectTemplate::Science),
        "empty" | "vacio" | "vacío" => Ok(ProjectTemplate::Empty),
        other => Err(format!("Template desconocido: '{}'\n{}", other, format_templates())),
    }
}

pub fn execute_command(cmd: &CliCommand) -> Result<(), String> {
    match cmd {
        CliCommand::New { name, template, dir } => {
            let dest = dir.as_ref().unwrap_or(name);
            println!("🛡️ Rybot — Creando proyecto '{}'", name);
            println!("   Template: {} — {}", template.name(), template.description());
            create_project(dest, name, *template)?;
            println!("✅ Proyecto creado en '{}/'", dest);
            println!("\nSiguientes pasos:");
            println!("  cd {}", dest);
            println!("  cargo build --release");
            println!("  cargo run --release");
            Ok(())
        }
        CliCommand::ListTemplates => {
            println!("{}", format_templates());
            Ok(())
        }
        CliCommand::Info => print_project_info(),
        CliCommand::Help => { print_help(); Ok(()) }
        CliCommand::Version => { println!("rybot v0.19.0 (Ry-Dit Motor Central)"); Ok(()) }
    }
}

fn format_templates() -> String {
    let mut lines = vec!["🛡️ Templates disponibles:\n".to_string()];
    for t in list_templates() {
        lines.push(format!("  {:12} — {}", t.name(), t.description()));
    }
    lines.join("\n")
}

fn print_project_info() -> Result<(), String> {
    let mut current = std::env::current_dir()
        .map_err(|e| format!("No se pudo obtener directorio actual: {}", e))?;

    loop {
        let cargo_path = current.join("Cargo.toml");
        if cargo_path.exists() {
            let content = std::fs::read_to_string(&cargo_path)
                .map_err(|e| format!("Error leyendo Cargo.toml: {}", e))?;

            println!("📦 Proyecto Ry-Dit");
            println!("  Cargo.toml: {}", cargo_path.display());
            for line in content.lines() {
                if line.starts_with("name = ") {
                    let name = line.trim_start_matches("name = ").trim_matches('"');
                    println!("  Nombre: {}", name);
                }
                if line.starts_with("version = ") {
                    let ver = line.trim_start_matches("version = ").trim_matches('"');
                    println!("  Versión: {}", ver);
                }
            }
            let input_path = current.join("input.rydit-input");
            println!("  Input map: {}", if input_path.exists() { "✅" } else { "❌" });
            return Ok(());
        }
        if !current.pop() { break; }
    }

    Err("No se encontró Cargo.toml en el directorio actual o padres".into())
}

fn print_help() {
    println!(
        r#"🛡️ Rybot v0.19.0 — Motor Central de Ry-Dit

USO:
  rybot <comando> [opciones]

COMANDOS:
  new <nombre>              Crear nuevo proyecto
    -t, --template <tipo>   Template (default: game2d)
                            game2d, game3d, gui_app, animation, science, empty
    -d, --dir <ruta>        Directorio destino (default: nombre)

  templates, list, ls       Listar templates

  info, status              Info del proyecto actual

  help, -h, --help          Esta ayuda

  version, -v, --version    Versión

EJEMPLOS:
  rybot new mi_juego
  rybot new mi_3d -t game3d
  rybot new mi_gui --template gui_app
  rybot new mi_anim -t animation
  rybot templates
  rybot info
"#
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_new() {
        let args = vec!["new".to_string(), "mi_juego".to_string()];
        let cmd = parse_args(&args).unwrap();
        match cmd { CliCommand::New { name, .. } => assert_eq!(name, "mi_juego"), _ => panic!() }
    }

    #[test]
    fn test_parse_new_with_template() {
        let args = vec!["new".into(), "mi_3d".into(), "-t".into(), "game3d".into()];
        let cmd = parse_args(&args).unwrap();
        match cmd { CliCommand::New { template, .. } => assert!(matches!(template, ProjectTemplate::Game3D)), _ => panic!() }
    }

    #[test]
    fn test_parse_no_name() {
        assert!(parse_args(&vec!["new".into()]).is_err());
    }

    #[test]
    fn test_parse_templates() {
        for c in &["templates", "list", "ls"] {
            assert!(matches!(parse_args(&vec![c.to_string()]).unwrap(), CliCommand::ListTemplates));
        }
    }

    #[test]
    fn test_parse_help() {
        assert!(matches!(parse_args(&vec!["help".into()]).unwrap(), CliCommand::Help));
    }

    #[test]
    fn test_template_aliases() {
        assert!(matches!(parse_template("2d"), Ok(ProjectTemplate::Game2D)));
        assert!(matches!(parse_template("3d"), Ok(ProjectTemplate::Game3D)));
        assert!(matches!(parse_template("ciencia"), Ok(ProjectTemplate::Science)));
    }

    #[test]
    fn test_template_unknown() {
        assert!(parse_template("foobar").is_err());
    }
}
