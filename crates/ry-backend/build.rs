// crates/ry-backend/build.rs
// Vinculación de SDL2 y raylib según features activos.
// ry-backend llama al FFI de SDL2_ttf/image/mixer y raylib directamente,
// así que debe declarar sus propias libs (rustc solo propaga libs nativas
// de crates usados en el binario final).

fn main() {
    let sdl2 = [
        "CARGO_FEATURE_SDL2_ONLY",
        "CARGO_FEATURE_SDL2_BACKEND",
        "CARGO_FEATURE_DUAL_BACKEND",
        "CARGO_FEATURE_MOBILE_HYBRID",
    ]
    .iter()
    .any(|f| std::env::var_os(f).is_some());
    if sdl2 {
        println!("cargo:rustc-link-lib=SDL2");
        println!("cargo:rustc-link-lib=SDL2_image");
        println!("cargo:rustc-link-lib=SDL2_ttf");
        println!("cargo:rustc-link-lib=SDL2_mixer");
    }

    let raylib = [
        "CARGO_FEATURE_RAYLIB_ONLY",
        "CARGO_FEATURE_RAYLIB_BACKEND",
        "CARGO_FEATURE_DUAL_BACKEND",
        "CARGO_FEATURE_MOBILE_HYBRID",
    ]
    .iter()
    .any(|f| std::env::var_os(f).is_some());
    if raylib {
        println!("cargo:rustc-link-lib=raylib");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
