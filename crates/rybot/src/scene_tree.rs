//! Árbol de escena — como el SceneTree de Godot

use std::collections::HashMap;

/// Tipo de nodo en la escena
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    /// Nodo raíz de la escena
    Root,
    /// Entidad con sprite y física
    Entity,
    /// Cámara 2D o 3D
    Camera,
    /// Luz
    Light,
    /// UI/HUD
    UI,
    /// Tilemap
    Tilemap,
    /// Partículas
    Particles,
    /// Audio
    Audio,
    /// Lógica/script
    Script,
}

/// Nodo individual en el árbol de escena
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub name: String,
    pub node_type: NodeType,
    pub position: (f32, f32),
    pub rotation: f32,
    pub scale: (f32, f32),
    pub visible: bool,
    pub properties: HashMap<String, String>,
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    pub fn new(name: &str, node_type: NodeType) -> Self {
        Self {
            name: name.to_string(),
            node_type,
            position: (0.0, 0.0),
            rotation: 0.0,
            scale: (1.0, 1.0),
            visible: true,
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Agregar hijo
    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    /// Contar nodos (recursivo)
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(|c| c.count()).sum::<usize>()
    }

    /// Buscar nodo por nombre
    pub fn find(&self, name: &str) -> Option<&SceneNode> {
        if self.name == name {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(name) {
                return Some(found);
            }
        }
        None
    }

    /// Buscar nodo mutable por nombre
    pub fn find_mut(&mut self, name: &str) -> Option<&mut SceneNode> {
        if self.name == name {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_mut(name) {
                return Some(found);
            }
        }
        None
    }
}

/// Árbol de escena completo
#[derive(Debug)]
pub struct SceneTree {
    pub name: String,
    pub root: SceneNode,
}

impl Default for SceneTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneTree {
    pub fn new() -> Self {
        Self {
            name: "Untitled".to_string(),
            root: SceneNode::new("root", NodeType::Root),
        }
    }

    /// Contar todos los nodos
    pub fn node_count(&self) -> usize {
        self.root.count()
    }

    /// Agregar nodo al root
    pub fn add_node(&mut self, node: SceneNode) {
        self.root.add_child(node);
    }

    /// Buscar nodo por nombre
    pub fn find(&self, name: &str) -> Option<&SceneNode> {
        self.root.find(name)
    }

    /// Buscar nodo mutable
    pub fn find_mut(&mut self, name: &str) -> Option<&mut SceneNode> {
        self.root.find_mut(name)
    }

    /// Actualizar árbol de escena
    pub fn update(&mut self, _delta: f32) {
        // Actualizar nodos
    }

    /// Parsear escena desde texto .ryscene
    pub fn parse(content: &str) -> Result<Self, String> {
        let mut tree = Self::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("@name") {
                if let Some(name) = line.splitn(2, ' ').nth(1) {
                    tree.name = name.trim().trim_matches('"').to_string();
                }
            } else if line.starts_with("entity ") {
                let name = extract_name(line, "entity")?;
                tree.add_node(SceneNode::new(&name, NodeType::Entity));
            } else if line.starts_with("camera ") {
                let name = extract_name(line, "camera")?;
                tree.add_node(SceneNode::new(&name, NodeType::Camera));
            } else if line.starts_with("light ") {
                let name = extract_name(line, "light")?;
                tree.add_node(SceneNode::new(&name, NodeType::Light));
            } else if line.starts_with("tilemap ") {
                let name = extract_name(line, "tilemap").unwrap_or("tilemap".to_string());
                tree.add_node(SceneNode::new(&name, NodeType::Tilemap));
            }
        }

        Ok(tree)
    }
}

fn extract_name(line: &str, prefix: &str) -> Result<String, String> {
    let rest = line.strip_prefix(prefix).unwrap_or(line).trim();
    if rest.is_empty() {
        return Err(format!("{} sin nombre", prefix));
    }
    // Soporta: entity "player" o entity player
    let name = rest.trim_matches('"').split_whitespace().next()
        .ok_or_else(|| format!("{} sin nombre válido", prefix))?;
    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_tree_creation() {
        let tree = SceneTree::new();
        assert_eq!(tree.node_count(), 1); // solo root
    }

    #[test]
    fn test_add_nodes() {
        let mut tree = SceneTree::new();
        tree.add_node(SceneNode::new("player", NodeType::Entity));
        tree.add_node(SceneNode::new("camera", NodeType::Camera));
        assert_eq!(tree.node_count(), 3);
    }

    #[test]
    fn test_find_node() {
        let mut tree = SceneTree::new();
        tree.add_node(SceneNode::new("player", NodeType::Entity));
        tree.add_node(SceneNode::new("enemy", NodeType::Entity));

        assert!(tree.find("player").is_some());
        assert!(tree.find("enemy").is_some());
        assert!(tree.find("nonexistent").is_none());
    }

    #[test]
    fn test_parse_scene() {
        let content = r#"
@name "Mi Nivel"
entity "player"
entity "enemy1"
camera "main_camera"
"#;
        let tree = SceneTree::parse(content).unwrap();
        assert_eq!(tree.name, "Mi Nivel");
        assert_eq!(tree.node_count(), 4); // root + 3 nodos
        assert!(tree.find("player").is_some());
    }

    #[test]
    fn test_node_count_recursive() {
        let mut tree = SceneTree::new();
        let mut parent = SceneNode::new("group", NodeType::Root);
        parent.add_child(SceneNode::new("child1", NodeType::Entity));
        parent.add_child(SceneNode::new("child2", NodeType::Entity));
        tree.root.add_child(parent);

        assert_eq!(tree.node_count(), 4); // root + group + child1 + child2
    }
}
