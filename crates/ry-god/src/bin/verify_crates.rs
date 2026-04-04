//! Binary para verificar estado de todos los crates
//!
//! Uso: cargo run --bin verify_crates

use ry_god::tests::CrateVerifier;

fn main() {
    println!("🛡️ RY-GOD: Verificación de Crates\n");

    let statuses = CrateVerifier::verify_all();
    CrateVerifier::print_report(&statuses);

    // Contar listos para publicar
    let ready: Vec<_> = statuses.iter().filter(|s| s.ready_for_crates_io).collect();
    println!("\n📦 Crates listos para publicar: {}/{}", ready.len(), statuses.len());
    for s in &ready {
        println!("  ✅ {} v{}", s.name, s.version);
    }
}
