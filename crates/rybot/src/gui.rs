//! Rybot GUI — Inspector visual + Creador de proyectos con migui
//!
//! Paneles:
//! - **New Project**: nombre, template, directorio
//! - **Inspector**: propiedades del proyecto actual
//! - **Scene Tree**: nodos de la escena activa
//! - **Engine Stats**: FPS, frame time, nodos

use migui::{Migui, WidgetId, Rect, Color};
use crate::templates::ProjectTemplate;
use crate::RybotEngine;

// ============================================================================
// RYBOT GUI — Estado completo
// ============================================================================

/// Panel activo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    NewProject,
    Inspector,
    SceneTree,
    EngineStats,
    None,
}

/// Estado de la GUI de Rybot
pub struct RybotGui {
    // Paneles
    pub open: bool,
    pub active_panel: ActivePanel,

    // New Project
    pub project_name: String,
    pub selected_template: usize,
    pub project_dir: String,
    pub project_created: bool,
    pub status_message: String,
    pub status_ok: bool,

    // Window positions
    pub new_project_rect: Rect,
    pub inspector_rect: Rect,
    pub scene_tree_rect: Rect,
    pub stats_rect: Rect,

    // Menu bar
    pub menu_bar_open: bool,

    // Toggle flags
    pub show_new_project: bool,
    pub show_inspector: bool,
    pub show_scene_tree: bool,
    pub show_stats: bool,
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
            active_panel: ActivePanel::None,
            project_name: String::from("mi_proyecto"),
            selected_template: 0,
            project_dir: String::new(),
            project_created: false,
            status_message: String::new(),
            status_ok: true,
            new_project_rect: Rect::new(20.0, 20.0, 400.0, 300.0),
            inspector_rect: Rect::new(440.0, 20.0, 350.0, 500.0),
            scene_tree_rect: Rect::new(20.0, 340.0, 400.0, 300.0),
            stats_rect: Rect::new(440.0, 540.0, 350.0, 100.0),
            menu_bar_open: true,
            show_new_project: false,
            show_inspector: true,
            show_scene_tree: true,
            show_stats: true,
        }
    }

    /// Dibujar toda la GUI de Rybot
    pub fn draw(&mut self, gui: &mut Migui, engine: &RybotEngine) {
        if !self.open {
            return;
        }

        // Menu bar
        if self.menu_bar_open {
            self.draw_menu_bar(gui);
        }

        // Paneles
        if self.show_new_project {
            self.draw_new_project(gui);
        }
        if self.show_inspector {
            self.draw_inspector(gui, engine);
        }
        if self.show_scene_tree {
            self.draw_scene_tree(gui, engine);
        }
        if self.show_stats {
            self.draw_engine_stats(gui, engine);
        }
    }

    // ========================================================================
    // MENU BAR
    // ========================================================================

    fn draw_menu_bar(&mut self, gui: &mut Migui) {
        let bar_h = 25.0f32;
        gui.panel(
            WidgetId::new("menu_bg"),
            Rect::new(0.0, 0.0, 1200.0, bar_h),
            Color::new(40, 40, 40, 255),
        );

        let mut x = 5.0;
        let items = [
            ("Archivo", &["Nuevo Proyecto", "Abrir", "Guardar", "---", "Salir"] as &[&str]),
            ("Ver", &["Inspector", "Scene Tree", "Stats", "---", "Reset Layout"]),
            ("Ayuda", &["Documentación", "Acerca de Rybot"]),
        ];

        for (label, _subitems) in &items {
            let btn_w = 80.0;
            let rect = Rect::new(x, 2.0, btn_w, bar_h - 4.0);
            if gui.button(WidgetId::new(&format!("menu_{}", label)), rect, label) {
                match *label {
                    "Archivo" => self.show_new_project = !self.show_new_project,
                    "Ver" => {
                        self.show_inspector = !self.show_inspector;
                        self.show_scene_tree = !self.show_scene_tree;
                        self.show_stats = !self.show_stats;
                    }
                    "Ayuda" => {}
                    _ => {}
                }
            }
            x += btn_w + 5.0;
        }
    }

    // ========================================================================
    // NEW PROJECT PANEL
    // ========================================================================

    fn draw_new_project(&mut self, gui: &mut Migui) {
        let mut open = self.show_new_project;
        if gui.window(
            WidgetId::new("win_new_project"),
            "🛡️ Nuevo Proyecto",
            self.new_project_rect,
            &mut open,
        ) {
            self.show_new_project = open;

            let mut y = 60.0;
            let left = self.new_project_rect.x + 15.0;
            let field_w = self.new_project_rect.w - 30.0;

            // Nombre del proyecto
            gui.label(WidgetId::new("lbl_name"), "Nombre del proyecto:",
                Rect::new(left, y, 200.0, 20.0));
            y += 22.0;

            let name = gui.textbox(
                WidgetId::new("txt_name"),
                Rect::new(left, y, field_w, 28.0),
            ).to_string();
            if name != self.project_name {
                self.project_name = name;
                if self.project_dir.is_empty() {
                    self.project_dir = self.project_name.clone();
                }
            }
            y += 35.0;

            // Directorio
            gui.label(WidgetId::new("lbl_dir"), "Directorio:",
                Rect::new(left, y, 200.0, 20.0));
            y += 22.0;
            let dir_text = gui.textbox(
                WidgetId::new("txt_dir"),
                Rect::new(left, y, field_w, 28.0),
            ).to_string();
            self.project_dir = dir_text;
            y += 35.0;

            // Template selector
            gui.label(WidgetId::new("lbl_template"), "Template:",
                Rect::new(left, y, 200.0, 20.0));
            y += 22.0;

            let templates = ProjectTemplate::all();
            for (i, t) in templates.iter().enumerate() {
                let is_selected = i == self.selected_template;
                let label = format!("{} — {}", t.name(), t.description());
                let mut checked = is_selected;
                let radio_id = format!("radio_tpl_{}", i);
                // Usar checkbox como radio visual
                let _changed = gui.checkbox(
                    WidgetId::new(&radio_id),
                    &label,
                    &mut checked,
                    Rect::new(left, y, field_w, 20.0),
                );
                if checked && !is_selected {
                    self.selected_template = i;
                }
                y += 24.0;
            }
            y += 5.0;

            // Botón crear
            if gui.button(
                WidgetId::new("btn_create"),
                Rect::new(left, y, field_w, 32.0),
                "🚀 Crear Proyecto",
            ) {
                self.create_project();
            }
            y += 40.0;

            // Status
            if !self.status_message.is_empty() {
                let color = if self.status_ok { Color::GREEN } else { Color::RED };
                gui.label(WidgetId::new("lbl_status"), &self.status_message,
                    Rect::new(left, y, field_w, 20.0));
                // Override color for status — simplified (migui no tiene color override en label)
            }
        }
    }

    fn create_project(&mut self) {
        let dest = if self.project_dir.is_empty() {
            &self.project_name
        } else {
            &self.project_dir
        };

        let templates = ProjectTemplate::all();
        let template = if self.selected_template < templates.len() {
            templates[self.selected_template]
        } else {
            ProjectTemplate::Empty
        };

        match crate::templates::create_project(dest, &self.project_name, template) {
            Ok(()) => {
                self.status_message = format!("✅ Proyecto '{}' creado en {}/", self.project_name, dest);
                self.status_ok = true;
                self.project_created = true;
            }
            Err(e) => {
                self.status_message = format!("❌ {}", e);
                self.status_ok = false;
                self.project_created = false;
            }
        }
    }

    // ========================================================================
    // INSPECTOR PANEL
    // ========================================================================

    fn draw_inspector(&mut self, gui: &mut Migui, engine: &RybotEngine) {
        let mut open = self.show_inspector;
        if gui.window(
            WidgetId::new("win_inspector"),
            "📋 Inspector",
            self.inspector_rect,
            &mut open,
        ) {
            self.show_inspector = open;

            let mut y = 60.0;
            let left = self.inspector_rect.x + 15.0;
            let w = self.inspector_rect.w - 30.0;

            // Sección: Motor
            gui.label(WidgetId::new("sec_motor"), "══ MOTOR ══",
                Rect::new(left, y, w, 20.0));
            y += 22.0;

            let state_label = match engine.state() {
                crate::EngineState::Running => "🟢 Running",
                crate::EngineState::Paused => "🟡 Paused",
                crate::EngineState::Loading => "🔵 Loading",
                crate::EngineState::Initializing => "⚪ Initializing",
                crate::EngineState::ShuttingDown => "🔴 ShuttingDown",
            };
            gui.label(WidgetId::new("ins_state"), &format!("Estado: {}", state_label),
                Rect::new(left, y, w, 20.0));
            y += 20.0;
            gui.label(WidgetId::new("ins_frame"), &format!("Frame: {}", engine.frame()),
                Rect::new(left, y, w, 20.0));
            y += 20.0;

            // Botones de control del motor
            y += 5.0;
            if engine.is_running() {
                if gui.button(WidgetId::new("btn_pause"),
                    Rect::new(left, y, w / 2.0 - 5.0, 28.0), "⏸ Pausar") {
                    // engine mut needed
                }
                if gui.button(WidgetId::new("btn_shutdown"),
                    Rect::new(left + w / 2.0 + 5.0, y, w / 2.0 - 5.0, 28.0), "⏹ Detener") {
                    // engine mut needed
                }
            } else if engine.is_paused() {
                if gui.button(WidgetId::new("btn_resume"),
                    Rect::new(left, y, w, 28.0), "▶ Reanudar") {
                    // engine mut needed
                }
            }
            y += 35.0;

            // Sección: Subsistemas
            y += 5.0;
            gui.label(WidgetId::new("sec_subs"), "══ SUBSISTEMAS ══",
                Rect::new(left, y, w, 20.0));
            y += 22.0;

            let subs = [
                ("Input", engine.input().action_count() > 0),
                ("Físicas", engine.physics().enabled()),
                ("Animación", engine.animation().enabled()),
                ("Ciencia", engine.science().enabled()),
                ("Red", engine.network().enabled()),
            ];
            for (name, active) in &subs {
                let status = if *active { "✅" } else { "❌" };
                gui.label(WidgetId::new(&format!("sub_{}", name)),
                    &format!("{}  {}", status, name),
                    Rect::new(left, y, w, 20.0));
                y += 20.0;
            }
        }
    }

    // ========================================================================
    // SCENE TREE PANEL
    // ========================================================================

    fn draw_scene_tree(&mut self, gui: &mut Migui, engine: &RybotEngine) {
        let mut open = self.show_scene_tree;
        if gui.window(
            WidgetId::new("win_scene"),
            "🌳 Scene Tree",
            self.scene_tree_rect,
            &mut open,
        ) {
            self.show_scene_tree = open;

            let mut y = 60.0;
            let left = self.scene_tree_rect.x + 15.0;
            let w = self.scene_tree_rect.w - 30.0;

            let scene = engine.scene();
            let count = scene.node_count();
            gui.label(WidgetId::new("st_count"),
                &format!("Nodos: {}", count),
                Rect::new(left, y, w, 20.0));
            y += 25.0;

            // Lista de nodos
            self.draw_node_tree(gui, &scene.root, left, &mut y, 0);
        }
    }

    fn draw_node_tree(&self, gui: &mut Migui, node: &crate::SceneNode, x: f32, y: &mut f32, depth: usize) {
        let indent = depth as f32 * 16.0;
        let icon = match node.node_type {
            crate::NodeType::Root => "📁",
            crate::NodeType::Entity => "🎮",
            crate::NodeType::Camera => "📷",
            crate::NodeType::Light => "💡",
            crate::NodeType::UI => "🖼",
            crate::NodeType::Tilemap => "🗺",
            crate::NodeType::Particles => "✨",
            crate::NodeType::Audio => "🔊",
            crate::NodeType::Script => "📜",
        };

        let children_info = if node.children.is_empty() {
            String::new()
        } else {
            format!(" ({})", node.children.len())
        };
        let label = format!("{}{} {}{}",
            "  ".repeat(depth),
            icon,
            node.name,
            children_info,
        );

        gui.label(WidgetId::new(&format!("node_{}", node.name)),
            &label,
            Rect::new(x + indent, *y, self.scene_tree_rect.w - 30.0 - indent, 20.0));
        *y += 20.0;

        for child in &node.children {
            self.draw_node_tree(gui, child, x, y, depth + 1);
        }
    }

    // ========================================================================
    // ENGINE STATS PANEL
    // ========================================================================

    fn draw_engine_stats(&mut self, gui: &mut Migui, engine: &RybotEngine) {
        let stats = engine.get_stats();

        let mut open = self.show_stats;
        if gui.window(
            WidgetId::new("win_stats"),
            "📊 Engine Stats",
            self.stats_rect,
            &mut open,
        ) {
            self.show_stats = open;

            let mut y = 60.0;
            let left = self.stats_rect.x + 15.0;
            let w = self.stats_rect.w - 30.0;

            let lines = vec![
                format!("Frame: {} | Target FPS: {}", stats.frame, stats.target_fps),
                format!("Nodos escena: {}", stats.scene_nodes),
                format!("Acciones input: {}", stats.input_actions),
                format!("Físicas: {} | Animación: {} | Red: {}",
                    if stats.physics_enabled { "ON" } else { "OFF" },
                    if stats.animation_enabled { "ON" } else { "OFF" },
                    if stats.network_enabled { "ON" } else { "OFF" },
                ),
            ];

            for line in &lines {
                gui.label(WidgetId::new(&format!("stat_{}", line)), line,
                    Rect::new(left, y, w, 18.0));
                y += 18.0;
            }
        }
    }

    // ========================================================================
    // HELPERS
    // ========================================================================

    pub fn reset(&mut self) {
        self.project_name = String::from("mi_proyecto");
        self.selected_template = 0;
        self.project_dir = String::new();
        self.project_created = false;
        self.status_message = String::new();
        self.status_ok = true;
        self.show_new_project = false;
        self.show_inspector = true;
        self.show_scene_tree = true;
        self.show_stats = true;
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
    fn test_default_gui() {
        let gui = RybotGui::new();
        assert!(!gui.open);
        assert!(gui.show_inspector);
        assert!(gui.show_scene_tree);
        assert!(gui.show_stats);
    }

    #[test]
    fn test_create_project() {
        let mut gui = RybotGui::new();
        gui.open = true;
        gui.project_name = "test_gui_proj".into();
        gui.selected_template = 5;
        gui.create_project();
        assert!(gui.project_created);
        assert!(gui.status_ok);
        let _ = fs::remove_dir_all("test_gui_proj");
    }

    #[test]
    fn test_reset() {
        let mut gui = RybotGui::new();
        gui.project_name = "otro".into();
        gui.selected_template = 3;
        gui.reset();
        assert_eq!(gui.project_name, "mi_proyecto");
        assert_eq!(gui.selected_template, 0);
    }
}
