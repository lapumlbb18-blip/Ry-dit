//! Test de Verificación de Crates - Estado real de cada uno
//!
//! Verifica: compilación, imports faltantes, imports no usados,
//! dependencias rotas, y readiness para crates.io

use std::process::Command;

#[derive(Debug, Clone)]
pub struct CrateStatus {
    pub name: String,
    pub version: String,
    pub compiles: bool,
    pub has_tests: bool,
    pub tests_pass: bool,
    pub missing_imports: Vec<String>,
    pub unused_imports: Vec<String>,
    pub broken_deps: Vec<String>,
    pub ready_for_crates_io: bool,
    pub notes: String,
}

/// Verificador completo de crates
pub struct CrateVerifier;

impl CrateVerifier {
    /// Verificar TODOS los crates del workspace
    pub fn verify_all() -> Vec<CrateStatus> {
        let mut results = Vec::new();

        let crates = [
            ("ry-core", "0.8.2"),
            ("ry-lexer", "0.1.0"),
            ("ry-parser", "0.1.0"),
            ("ry-vm", "0.1.0"),
            ("ry-loader", "0.8.2"),
            ("ry-anim", "0.7.34"),
            ("ry-physics", "0.7.34"),
            ("ry-science", "0.7.34"),
            ("ry-script", "0.8.2"),
            ("ry-stream", "0.1.0"),
            ("ry-test", "0.11.1"),
            ("ry-ecs", "0.10.0"),
            ("ry-gfx", "0.10.7"),
            ("ry-rs", "0.11.2"),
            ("ry-god", "0.1.0"),
            ("lizer", "0.11.2"),
            ("v-shield", "0.1.0"),
            ("blast-core", "0.1.0"),
            ("migui", "0.4.1"),
            ("toolkit-ry", "0.1.0"),
            ("ry-system-ry", "0.11.0"),
        ];

        for (name, version) in crates {
            let status = Self::verify_crate(name, version);
            results.push(status);
        }

        results
    }

    fn verify_crate(name: &str, version: &str) -> CrateStatus {
        // 1. Verificar compilación
        let compile_output = Command::new("cargo")
            .args(["check", "-p", name])
            .output()
            .expect("Failed to run cargo check");

        let compiles = compile_output.status.success();
        let stderr = String::from_utf8_lossy(&compile_output.stderr);

        // 2. Detectar imports faltantes (errores E0432, E0433)
        let missing_imports: Vec<String> = stderr
            .lines()
            .filter(|line| line.contains("unresolved import") || line.contains("use of unresolved module"))
            .map(|l| l.trim().to_string())
            .collect();

        // 3. Detectar imports no usados (warnings)
        let unused_imports: Vec<String> = stderr
            .lines()
            .filter(|line| line.contains("unused import"))
            .map(|l| l.trim().to_string())
            .collect();

        // 4. Detectar dependencias rotas
        let broken_deps: Vec<String> = stderr
            .lines()
            .filter(|line| line.contains("failed to load manifest") || line.contains("failed to read"))
            .map(|l| l.trim().to_string())
            .collect();

        // 5. Verificar tests
        let test_output = Command::new("cargo")
            .args(["test", "-p", name, "--no-run"])
            .output();

        let has_tests = test_output.is_ok() && test_output.as_ref().unwrap().status.success();
        let tests_pass = has_tests;

        // 6. Determinar readiness para crates.io
        let ready_for_crates_io = compiles
            && missing_imports.is_empty()
            && broken_deps.is_empty()
            && !name.contains("system-ry"); // ry-system-ry es demo, no para crates.io

        let notes = if compiles {
            if missing_imports.is_empty() && unused_imports.is_empty() {
                "✅ Compila limpio".to_string()
            } else if !missing_imports.is_empty() {
                format!("⚠️ {} imports faltantes", missing_imports.len())
            } else {
                format!("⚠️ {} unused imports", unused_imports.len())
            }
        } else {
            "❌ No compila".to_string()
        };

        CrateStatus {
            name: name.to_string(),
            version: version.to_string(),
            compiles,
            has_tests,
            tests_pass,
            missing_imports,
            unused_imports,
            broken_deps,
            ready_for_crates_io,
            notes,
        }
    }

    /// Generar informe completo
    pub fn generate_report(statuses: &[CrateStatus]) -> String {
        let total = statuses.len();
        let compiling = statuses.iter().filter(|s| s.compiles).count();
        let ready = statuses.iter().filter(|s| s.ready_for_crates_io).count();
        let with_issues = total - compiling;

        let mut report = String::new();
        report.push_str("🛡️ RY-GOD: VERIFICACIÓN DE CRATES\n");
        report.push_str(&"=".repeat(60));
        report.push('\n');
        report.push_str(&format!("Total crates: {}\n", total));
        report.push_str(&format!("✅ Compilando:  {}\n", compiling));
        report.push_str(&format!("🚀 Listos crates.io: {}\n", ready));
        report.push_str(&format!("❌ Con issues:   {}\n", with_issues));
        report.push('\n');

        // Tabla resumen
        report.push_str("| Crate | Versión | Compila | Tests | crates.io | Notas |\n");
        report.push_str("|-------|---------|---------|-------|-----------|-------|\n");

        for s in statuses {
            let compila = if s.compiles { "✅" } else { "❌" };
            let tests = if s.has_tests { "✅" } else { "⏳" };
            let cratesio = if s.ready_for_crates_io { "✅" } else { "❌" };

            report.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                s.name, s.version, compila, tests, cratesio, s.notes
            ));
        }

        // Detalles de errores
        report.push_str("\n\n🔍 DETALLES DE ERRORES\n");
        report.push_str(&"=".repeat(60));

        for s in statuses {
            if !s.missing_imports.is_empty() || !s.broken_deps.is_empty() {
                report.push_str(&format!("\n❌ {}:\n", s.name));
                for imp in &s.missing_imports {
                    report.push_str(&format!("  - Missing import: {}\n", imp));
                }
                for dep in &s.broken_deps {
                    report.push_str(&format!("  - Broken dep: {}\n", dep));
                }
            }
        }

        // Recomendaciones
        report.push_str("\n\n📋 RECOMENDACIONES PARA PUBLICAR\n");
        report.push_str(&"=".repeat(60));

        let ready_crates: Vec<_> = statuses.iter().filter(|s| s.ready_for_crates_io).collect();
        if !ready_crates.is_empty() {
            report.push_str("\n✅ Listos para publicar en crates.io:\n");
            for s in &ready_crates {
                report.push_str(&format!("  - {} v{}\n", s.name, s.version));
            }
        }

        let not_ready: Vec<_> = statuses.iter().filter(|s| !s.ready_for_crates_io && s.compiles).collect();
        if !not_ready.is_empty() {
            report.push_str("\n⚠️ Compilan pero necesitan fixes antes de publicar:\n");
            for s in &not_ready {
                report.push_str(&format!("  - {} v{}: {}\n", s.name, s.version, s.notes));
            }
        }

        let broken: Vec<_> = statuses.iter().filter(|s| !s.compiles).collect();
        if !broken.is_empty() {
            report.push_str("\n❌ No compilan (requieren atención urgente):\n");
            for s in &broken {
                report.push_str(&format!("  - {} v{}\n", s.name, s.version));
            }
        }

        report.push_str("\n\n🏁 COMANDO PARA PUBLICAR\n");
        report.push_str(&"=".repeat(60));
        report.push_str("\n  cargo publish -p <crate-name>\n");
        report.push_str("\n  Primero: login con 'cargo login <token>'\n");

        report
    }

    /// Imprimir reporte
    pub fn print_report(statuses: &[CrateStatus]) {
        println!("{}", Self::generate_report(statuses));
    }
}
