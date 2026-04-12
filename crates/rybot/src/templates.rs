//! Templates de proyecto para Rybot CLI

use std::fs;
use std::path::Path;

/// Template de proyecto
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectTemplate {
    /// Juego 2D básico
    Game2D,
    /// Juego 3D con cámara
    Game3D,
    /// App GUI
    GuiApp,
    /// Animación
    Animation,
    /// Ciencia
    Science,
    /// Vacío
    Empty,
}

impl ProjectTemplate {
    pub fn all() -> &'static [Self] {
        &[Self::Game2D, Self::Game3D, Self::GuiApp, Self::Animation, Self::Science, Self::Empty]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Game2D => "game2d",
            Self::Game3D => "game3d",
            Self::GuiApp => "gui_app",
            Self::Animation => "animation",
            Self::Science => "science",
            Self::Empty => "empty",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Game2D => "Juego 2D con game loop, input, sprite rendering",
            Self::Game3D => "Juego 3D con cámara orbital, controles táctiles",
            Self::GuiApp => "App GUI con migui (menús, paneles, consola)",
            Self::Animation => "Animación con ry-anim (12 principios Disney)",
            Self::Science => "Simulación científica (Bezier, ondas, L-System)",
            Self::Empty => "Proyecto vacío con estructura mínima",
        }
    }
}

pub fn list_templates() -> Vec<ProjectTemplate> {
    ProjectTemplate::all().to_vec()
}

/// Crear proyecto desde template
pub fn create_project(dest: &str, name: &str, template: ProjectTemplate) -> Result<(), String> {
    let dest = Path::new(dest);
    if dest.exists() {
        return Err(format!("Directorio '{}' ya existe", dest.display()));
    }

    fs::create_dir_all(dest.join("src"))
        .map_err(|e| format!("No se pudo crear '{}': {}", dest.display(), e))?;

    // Cargo.toml
    let deps = match template {
        ProjectTemplate::Game2D => "ry-input = \"0.1\"\nrybot = \"0.1\"\n",
        ProjectTemplate::Game3D => "ry-input = \"0.1\"\nrybot = \"0.1\"\nry3d-gfx = \"0.1\"\n",
        ProjectTemplate::GuiApp => "rybot = \"0.1\"\nmigui = \"0.4\"\n",
        ProjectTemplate::Animation => "ry-anim = \"0.12\"\n",
        ProjectTemplate::Science => "ry-science = \"0.7\"\n",
        ProjectTemplate::Empty => "",
    };

    let cargo = format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n{}\n", name, deps);
    fs::write(dest.join("Cargo.toml"), cargo)
        .map_err(|e| format!("Error escribiendo Cargo.toml: {}", e))?;

    // main.rs
    let main = match template {
        ProjectTemplate::Game2D => r#"fn main() {
    println!("🛡️ Juego 2D - Presiona ESC");
}
"#,
        ProjectTemplate::Game3D => r#"fn main() -> Result<(), String> {
    println!("🛡️ Juego 3D - Presiona ESC");
    Ok(())
}
"#,
        ProjectTemplate::GuiApp => r#"fn main() {
    println!("🛡️ GUI App");
}
"#,
        ProjectTemplate::Animation => r#"fn main() {
    println!("🛡️ Animación");
}
"#,
        ProjectTemplate::Science => r#"fn main() {
    println!("🛡️ Ciencia");
}
"#,
        ProjectTemplate::Empty => r#"fn main() {
    println!("🛡️ Mi Proyecto");
}
"#,
    };
    fs::write(dest.join("src").join("main.rs"), main)
        .map_err(|e| format!("Error escribiendo main.rs: {}", e))?;

    // input map
    let input = "# Ry-Dit Input Map\nmove_up = W, Up\nmove_down = S, Down\nmove_left = A, Left\nmove_right = D, Right\njump = Space\npause = Escape, P\n";
    fs::write(dest.join("input.rydit-input"), input)
        .map_err(|e| format!("Error escribiendo input map: {}", e))?;

    // .gitignore
    fs::write(dest.join(".gitignore"), "target/\n**/*.rs.bk\n")
        .map_err(|e| format!("Error escribiendo .gitignore: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_templates() {
        let tpls = list_templates();
        assert_eq!(tpls.len(), 6);
    }

    #[test]
    fn test_create_project() {
        let dest = "test_rybot_proj";
        let result = create_project(dest, "test", ProjectTemplate::Empty);
        assert!(result.is_ok());
        assert!(Path::new(dest).join("Cargo.toml").exists());
        assert!(Path::new(dest).join("input.rydit-input").exists());
        let _ = fs::remove_dir_all(dest);
    }

    #[test]
    fn test_create_existing_fails() {
        let dest = "test_rybot_exist";
        fs::create_dir_all(dest).ok();
        let result = create_project(dest, "test", ProjectTemplate::Empty);
        assert!(result.is_err());
        let _ = fs::remove_dir_all(dest);
    }
}
