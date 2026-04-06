//! ry-backend - Dual backend (raylib + SDL2) para ry-dit
//!
//! ## Filosofía
//! - **SDL2 para texto TTF**: Más profesional, anti-alias, fuentes reales
//! - **Raylib para drawing 2D/3D**: Más rápido, API simple
//! - **SDL2 para input completo**: Mouse, touch, keyboard, gamepad
//!
//! ## Features
//! - `dual-backend` (default): Raylib drawing + SDL2 input/TTF/audio
//! - `raylib-only`: Solo raylib (todo en uno)
//! - `sdl2-only`: Solo SDL2 (input/TTF/audio/assets)
//! - `mobile-hybrid`: Raylib + SDL2 para Termux-X11

// Módulos públicos
#[cfg(any(feature = "dual-backend", feature = "raylib-only", feature = "mobile-hybrid"))]
pub mod raylib_draw;

#[cfg(any(feature = "dual-backend", feature = "sdl2-only", feature = "mobile-hybrid"))]
pub mod sdl2_core;

// Re-exports principales
#[cfg(any(feature = "dual-backend", feature = "raylib-only", feature = "mobile-hybrid"))]
pub use raylib_draw::*;

#[cfg(any(feature = "dual-backend", feature = "sdl2-only", feature = "mobile-hybrid"))]
pub use sdl2_core::*;

// Versión del crate
pub const VERSION: &str = "0.1.0";
