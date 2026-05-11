//! Demo Input Map — demuestra el sistema de input configurable de ry-input.
//!
//! ```bash
//! cargo run --bin demo_input_map --release
//! ```

use ry_input::{game_2d_defaults, editor_defaults, InputMap, InputState, InputSource, K, M};

fn main() {
    println!("🛡️ Ry-Dit v0.19.0 — Input Map Configurable\n");

    // 1. Defaults de juego
    println!("═══ GAME 2D DEFAULTS ═══");
    let map = game_2d_defaults();
    print_map(&map);

    // 2. Defaults de editor
    println!("\n═══ EDITOR DEFAULTS ═══");
    let editor_map = editor_defaults();
    print_map(&editor_map);

    // 3. Custom input map
    println!("\n═══ CUSTOM MAP ═══");
    let mut custom = InputMap::new();
    custom.add_action("move", vec![K!("W"), K!("A"), K!("S"), K!("D")]);
    custom.add_action("shoot", vec![M!("MouseLeft"), K!("J")]);
    custom.add_action("reload", vec![K!("R")]);
    custom.add_action("grenade", vec![K!("G")]);
    custom.add_action("melee", vec![K!("V"), K!("E")]);
    print_map(&custom);

    // 4. Estado en tiempo real (simulado)
    println!("\n═══ INPUT STATE SIMULADO ═══");
    let mut state = InputState::new(&custom);
    state.begin_frame();
    state.update_key("W", true);
    state.update_key("J", false);
    state.update_key("MouseLeft", true);

    println!("  W pressed      → move: {:?}", state.is_action_pressed("move"));
    println!("  MouseLeft      → shoot: {:?}", state.is_action_pressed("shoot"));
    println!("  G pressed?     → grenade: {:?}", state.is_action_pressed("grenade"));

    // 5. Serialización
    println!("\n═══ SERIALIZADO (.rydit-input) ═══");
    println!("{}", custom.to_contenido());

    println!("✅ Input Map demo completo");
}

fn print_map(map: &InputMap) {
    for action in map.actions() {
        if let Some(sources) = map.get_action(action) {
            let labels: Vec<&str> = sources.iter().map(|s| s.label()).collect();
            println!("  {:16} = {}", action, labels.join(", "));
        }
    }
}
