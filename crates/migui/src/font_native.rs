// crates/migui/src/font_native.rs
// Fuentes - v0.13.0
// ab_glyph con API correcta

use crate::Color;

// Intentar usar ab_glyph si está disponible
#[cfg(feature = "ab_glyph_feat")]
mod real_font {
    use ab_glyph::{point, Font, FontRef, Glyph, PxScale, ScaleFont};
    use crate::Color;

    // Fuente TTF copiada por build.rs a OUT_DIR (DejaVuSans.ttf)
    const FONT_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/font.ttf"));

    pub struct TextTexture {
        pub pixels: Vec<u8>,
        pub width: u32,
        pub height: u32,
    }

    pub struct NativeFontManager {
        font: Option<FontRef<'static>>,
    }

    impl NativeFontManager {
        pub fn new() -> Self {
            Self {
                font: FontRef::try_from_slice(FONT_BYTES).ok(),
            }
        }

        pub fn render_text(&self, text: &str, size: f32, color: Color) -> Option<TextTexture> {
            let font = self.font.as_ref()?;

            let scale = PxScale::from(size);
            let scaled = font.as_scaled(scale);

            // Layout horizontal: posición de cada glifo sobre la línea base (y = 0)
            let mut glyphs = Vec::new();
            let mut pen_x = 0.0f32;
            for ch in text.chars() {
                let gid = scaled.glyph_id(ch);
                glyphs.push(Glyph {
                    id: gid,
                    scale,
                    position: point(pen_x, 0.0),
                });
                pen_x += scaled.h_advance(gid);
            }

            let outlined: Vec<_> = glyphs
                .iter()
                .filter_map(|g| font.outline_glyph(g.clone()))
                .collect();
            if outlined.is_empty() {
                // Texto vacío o sin glifos (p. ej. solo espacios): textura mínima
                let width = pen_x.ceil().max(1.0) as u32;
                let height = size.max(1.0) as u32;
                return Some(TextTexture {
                    pixels: vec![0u8; (width * height * 4) as usize],
                    width,
                    height,
                });
            }

            // Bounds globales entre todos los glifos
            let (mut min_x, mut min_y) = (f32::MAX, f32::MAX);
            let (mut max_x, mut max_y) = (f32::MIN, f32::MIN);
            for og in &outlined {
                let b = og.px_bounds();
                min_x = min_x.min(b.min.x);
                min_y = min_y.min(b.min.y);
                max_x = max_x.max(b.max.x);
                max_y = max_y.max(b.max.y);
            }
            let width = ((max_x - min_x).ceil() as u32).max(1);
            let height = ((max_y - min_y).ceil() as u32).max(1);

            // Rasterizar a RGBA
            let mut pixels = vec![0u8; (width * height * 4) as usize];
            for og in &outlined {
                let b = og.px_bounds();
                og.draw(|x, y, coverage| {
                    let px = (b.min.x - min_x).round() as i32 + x as i32;
                    let py = (b.min.y - min_y).round() as i32 + y as i32;
                    if px < 0 || py < 0 || px as u32 >= width || py as u32 >= height {
                        return;
                    }
                    let i = ((py as u32 * width + px as u32) * 4) as usize;
                    pixels[i] = color.r;
                    pixels[i + 1] = color.g;
                    pixels[i + 2] = color.b;
                    pixels[i + 3] = (coverage * color.a as f32) as u8;
                });
            }

            Some(TextTexture {
                pixels,
                width,
                height,
            })
        }

        pub fn text_dimensions(&self, text: &str, size: f32) -> (u32, u32) {
            if let Some(font) = &self.font {
                let scale = PxScale::from(size);
                let scaled = font.as_scaled(scale);
                let w: f32 = text
                    .chars()
                    .map(|c| scaled.h_advance(scaled.glyph_id(c)))
                    .sum();
                let h = scaled.ascent() - scaled.descent();
                (w.ceil().max(1.0) as u32, h.ceil().max(1.0) as u32)
            } else {
                (text.len() as u32 * 8, 16)
            }
        }

        pub fn has_font(&self) -> bool {
            self.font.is_some()
        }
    }

    impl Default for NativeFontManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(not(feature = "ab_glyph_feat"))]
mod bitmap_font {
    use crate::Color;

    /// Textura de texto renderizado
    pub struct TextTexture {
        pub pixels: Vec<u8>,
        pub width: u32,
        pub height: u32,
    }

    /// Gestor de fuentes
    pub struct NativeFontManager {
        has_font: bool,
    }

    impl NativeFontManager {
        pub fn new() -> Self {
            Self { has_font: false }
        }

        /// Renderizar texto como bitmap simple
        /// Cada carácter = bloque de color (placeholder hasta tener TTF real)
        pub fn render_text(&self, text: &str, size: f32, color: Color) -> Option<TextTexture> {
            let char_w = (size as u32 / 2).max(4);
            let char_h = size as u32;
            let w = text.len() as u32 * char_w;
            let h = char_h;
            let mut pixels = vec![0u8; (w * h * 4) as usize];

            for (ci, ch) in text.chars().enumerate() {
                // Dibujar bloque simple como representación del carácter
                let ox = ci as u32 * char_w;
                for py in 0..h {
                    for px in 0..char_w {
                        // Patrón simple basado en el carácter
                        let pattern = (ch as u32 + px + py) % 3;
                        if pattern != 0 {
                            let i = ((py * w + ox + px) * 4) as usize;
                            if i + 3 < pixels.len() {
                                pixels[i] = color.r;
                                pixels[i+1] = color.g;
                                pixels[i+2] = color.b;
                                pixels[i+3] = 200;
                            }
                        }
                    }
                }
            }

            Some(TextTexture { pixels, width: w, height: h })
        }

        pub fn text_dimensions(&self, text: &str, size: f32) -> (u32, u32) {
            let char_w = (size as u32 / 2).max(4);
            (text.len() as u32 * char_w, size as u32)
        }

        pub fn has_font(&self) -> bool { self.has_font }
    }

    impl Default for NativeFontManager {
        fn default() -> Self { Self::new() }
    }
}

#[cfg(feature = "ab_glyph_feat")]
pub use real_font::*;

#[cfg(not(feature = "ab_glyph_feat"))]
pub use bitmap_font::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create() { let _ = NativeFontManager::new(); }
    #[test]
    fn test_render() {
        let m = NativeFontManager::new();
        let tex = m.render_text("Hola", 16.0, Color { r: 255, g: 255, b: 255, a: 255 });
        assert!(tex.is_some());
    }
}
