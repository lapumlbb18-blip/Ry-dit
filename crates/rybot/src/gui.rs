//! Rybot GUI — Interfaz visual (panel de proyecto)
//!
//! Se integrará con migui para mostrar un panel de creación de proyectos.
//! Por ahora es la lógica de estado sin render.

use crate::templates::{ProjectTemplate, create_project};

/// Estado de la GUI de Rybot
pub struct RybotGui {
    pub open: bool,
    pub project_name: String,
    pub selected_template: usize,
    pub status_message: String,
    pub status_ok: bool,
}

impl Default for RybotGui {
    fn default() -> Self {
        Self::new()
    }
}

impl RybotGui {
    pub fn new() -> Self {
        Self {
            open: false,
            project_name: String::from("mi_proyecto"),
            selected_template: 0,
            status_message: String::new(),
            status_ok: true,
        }
    }

    /// Crear proyecto con valores actuales
    pub fn create(&mut self, dest: &str) -> bool {
        let templates = ProjectTemplate::all();
        let template = if self.selected_template < templates.len() {
            templates[self.selected_template]
        } else {
            ProjectTemplate::Empty
        };

        match create_project(dest, &self.project_name, template) {
            Ok(()) => {
                self.status_message = format!("✅ Proyecto '{}' creado", self.project_name);
                self.status_ok = true;
                true
            }
            Err(e) => {
                self.status_message = format!("❌ Error: {}", e);
                self.status_ok = false;
                false
            }
        }
    }

    pub fn reset(&mut self) {
        self.project_name = String::from("mi_proyecto");
        self.selected_template = 0;
        self.status_message = String::new();
        self.status_ok = true;
    }

    pub fn selected_template_name(&self) -> &str {
        let templates = ProjectTemplate::all();
        if self.selected_template < templates.len() {
            templates[self.selected_template].name()
        } else {
            "empty"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_default() {
        let gui = RybotGui::new();
        assert!(!gui.open);
        assert_eq!(gui.project_name, "mi_proyecto");
        assert_eq!(gui.selected_template, 0);
    }

    #[test]
    fn test_create_success() {
        let mut gui = RybotGui::new();
        gui.project_name = "test_gui".into();
        gui.selected_template = 5;
        assert!(gui.create("test_gui"));
        assert!(gui.status_ok);
        let _ = fs::remove_dir_all("test_gui");
    }

    #[test]
    fn test_create_existing() {
        let mut gui = RybotGui::new();
        fs::create_dir_all("test_gui_exist").ok();
        assert!(!gui.create("test_gui_exist"));
        assert!(!gui.status_ok);
        let _ = fs::remove_dir_all("test_gui_exist");
    }
}
