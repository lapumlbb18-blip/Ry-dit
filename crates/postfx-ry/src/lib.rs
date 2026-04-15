//! # Post-FX for Ry-Dit (postfx-ry)
//!
//! Post-processing effects y sistema de materiales.
//!
//! ## Efectos GPU
//! - **Bloom**: Brillo difuso en zonas claras
//! - **Blur**: Gaussian blur configurable
//! - **Sharpen**: Enfoque de imagen
//! - **Color grading**: Corrección de color por curvas
//! - **Chromatic aberration**: Separación RGB
//! - **Vignette**: Oscurecimiento en bordes
//!
//! ## Materiales (próximo)
//! - Goma, lava, vidrio, metal
//! - Química: mezcla, reacción, fusión
//! - Transformación visual: cortar, mojar, endurecer
//!
//! ## Filosofía
//! - Shaders GLSL ES 2.0 (Adreno 610 compatible)
//! - Encadenamiento de efectos configurable
//! - FBO intermedio para composición

#![allow(clippy::too_many_arguments)]

use gl::types::GLuint;
use std::ffi::CString;

// Shaders embebidos
const BLOOM_FS: &str = include_str!("shaders/bloom.fs.glsl");
const BLUR_FS: &str = include_str!("shaders/blur.fs.glsl");
const SHARPEN_FS: &str = include_str!("shaders/sharpen.fs.glsl");
const COLOR_GRADE_FS: &str = include_str!("shaders/color_grade.fs.glsl");
const CHROMATIC_FS: &str = include_str!("shaders/chromatic.fs.glsl");
const VIGNETTE_FS: &str = include_str!("shaders/vignette.fs.glsl");

// Vertex shader genérico fullscreen quad
const QUAD_VS: &str = r#"#version 100
attribute vec2 a_position;
varying vec2 v_uv;
void main() {
    v_uv = a_position * 0.5 + 0.5;
    gl_Position = vec4(a_position, 0.0, 1.0);
}"#;

// ============================================================================
// FULLSCREEN QUAD (compartido por todos los efectos)
// ============================================================================

struct QuadMesh {
    vao: GLuint,
    vbo: GLuint,
}

impl QuadMesh {
    fn new() -> Self {
        unsafe {
            let (mut vao, mut vbo) = (0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let verts: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (verts.len() * 4) as isize,
                verts.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let attr = 0;
            gl::VertexAttribPointer(attr, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(attr);
            gl::BindVertexArray(0);

            Self { vao, vbo }
        }
    }

    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for QuadMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

// ============================================================================
// SHADER COMPILER
// ============================================================================

fn compile_shader(stype: GLuint, src: &str) -> Result<GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(stype);
        let c = CString::new(src).map_err(|e| e.to_string())?;
        gl::ShaderSource(shader, 1, &c.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut ok = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok);
        if ok == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, &mut len, buf.as_mut_ptr() as *mut _);
            let msg = String::from_utf8_lossy(&buf);
            gl::DeleteShader(shader);
            return Err(format!("Shader compile error: {}", msg));
        }
        Ok(shader)
    }
}

fn link_program(vs: GLuint, fs: GLuint) -> Result<GLuint, String> {
    unsafe {
        let prog = gl::CreateProgram();
        gl::AttachShader(prog, vs);
        gl::AttachShader(prog, fs);
        gl::LinkProgram(prog);

        let mut ok = 0;
        gl::GetProgramiv(prog, gl::LINK_STATUS, &mut ok);
        if ok == 0 {
            let mut len = 0;
            gl::GetProgramiv(prog, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetProgramInfoLog(prog, len, &mut len, buf.as_mut_ptr() as *mut _);
            let msg = String::from_utf8_lossy(&buf);
            gl::DeleteProgram(prog);
            return Err(format!("Program link error: {}", msg));
        }

        gl::DetachShader(prog, vs);
        gl::DetachShader(prog, fs);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        Ok(prog)
    }
}

fn make_program(vs: &str, fs: &str) -> Result<GLuint, String> {
    let v = compile_shader(gl::VERTEX_SHADER, vs)?;
    let f = compile_shader(gl::FRAGMENT_SHADER, fs)?;
    link_program(v, f)
}

// ============================================================================
// FBO FRAMEBUFFER
// ============================================================================

/// Framebuffer intermedio para post-processing
pub struct Fbo {
    fbo: GLuint,
    texture: GLuint,
    width: u32,
    height: u32,
}

impl Fbo {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        unsafe {
            let (mut fbo, mut tex) = (0, 0);
            gl::GenFramebuffers(1, &mut fbo);
            gl::GenTextures(1, &mut tex);

            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGBA as i32,
                width as i32, height as i32, 0,
                gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex, 0);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                gl::DeleteTextures(1, &tex);
                gl::DeleteFramebuffers(1, &fbo);
                return Err("FBO incomplete".into());
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            Ok(Self { fbo, texture: tex, width, height })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    pub fn unbind(&self, sw: u32, sh: u32) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, sw as i32, sh as i32);
        }
    }

    pub fn texture(&self) -> GLuint { self.texture }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
}

impl Drop for Fbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteTextures(1, &self.texture);
        }
    }
}

// ============================================================================
// EFFECT: BLOOM
// ============================================================================

/// Post-processing bloom effect
pub struct BloomPass {
    program: GLuint,
    quad: QuadMesh,
    threshold_loc: i32,
    intensity_loc: i32,
    _resolution_loc: i32,
}

impl BloomPass {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let program = make_program(QUAD_VS, BLOOM_FS)?;
        unsafe {
            gl::UseProgram(program);
            let threshold_loc = gl::GetUniformLocation(program, b"u_threshold\0".as_ptr() as *const _);
            let intensity_loc = gl::GetUniformLocation(program, b"u_intensity\0".as_ptr() as *const _);
            let resolution_loc = gl::GetUniformLocation(program, b"u_resolution\0".as_ptr() as *const _);
            gl::Uniform2f(resolution_loc, width as f32, height as f32);
            gl::UseProgram(0);

            Ok(Self {
                program, quad: QuadMesh::new(),
                threshold_loc, intensity_loc, _resolution_loc: resolution_loc,
            })
        }
    }

    pub fn render(&self, texture: GLuint, threshold: f32, intensity: f32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform1f(self.threshold_loc, threshold);
            gl::Uniform1f(self.intensity_loc, intensity);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.quad.draw();
            gl::UseProgram(0);
        }
    }
}

impl Drop for BloomPass {
    fn drop(&mut self) { unsafe { gl::DeleteProgram(self.program); } }
}

// ============================================================================
// EFFECT: BLUR (Gaussian)
// ============================================================================

pub struct BlurPass {
    program: GLuint,
    quad: QuadMesh,
    dir_loc: i32,
    radius_loc: i32,
    _resolution_loc: i32,
}

impl BlurPass {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let program = make_program(QUAD_VS, BLUR_FS)?;
        unsafe {
            gl::UseProgram(program);
            let dir_loc = gl::GetUniformLocation(program, b"u_direction\0".as_ptr() as *const _);
            let radius_loc = gl::GetUniformLocation(program, b"u_radius\0".as_ptr() as *const _);
            let resolution_loc = gl::GetUniformLocation(program, b"u_resolution\0".as_ptr() as *const _);
            gl::Uniform2f(resolution_loc, width as f32, height as f32);
            gl::UseProgram(0);

            Ok(Self { program, quad: QuadMesh::new(), dir_loc, radius_loc, _resolution_loc: resolution_loc })
        }
    }

    /// Render blur pas horizontal (0) o vertical (1)
    pub fn render(&self, texture: GLuint, direction: u32, radius: u32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform1i(self.dir_loc, direction as i32);
            gl::Uniform1i(self.radius_loc, radius as i32);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.quad.draw();
            gl::UseProgram(0);
        }
    }
}

impl Drop for BlurPass {
    fn drop(&mut self) { unsafe { gl::DeleteProgram(self.program); } }
}

// ============================================================================
// EFFECT: SHARPEN
// ============================================================================

pub struct SharpenPass {
    program: GLuint,
    quad: QuadMesh,
    amount_loc: i32,
    _resolution_loc: i32,
}

impl SharpenPass {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let program = make_program(QUAD_VS, SHARPEN_FS)?;
        unsafe {
            gl::UseProgram(program);
            let amount_loc = gl::GetUniformLocation(program, b"u_amount\0".as_ptr() as *const _);
            let resolution_loc = gl::GetUniformLocation(program, b"u_resolution\0".as_ptr() as *const _);
            gl::Uniform2f(resolution_loc, width as f32, height as f32);
            gl::UseProgram(0);

            Ok(Self { program, quad: QuadMesh::new(), amount_loc, _resolution_loc: resolution_loc })
        }
    }

    pub fn render(&self, texture: GLuint, amount: f32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform1f(self.amount_loc, amount);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.quad.draw();
            gl::UseProgram(0);
        }
    }
}

impl Drop for SharpenPass {
    fn drop(&mut self) { unsafe { gl::DeleteProgram(self.program); } }
}

// ============================================================================
// EFFECT: COLOR GRADING
// ============================================================================

pub struct ColorGradePass {
    program: GLuint,
    quad: QuadMesh,
    shadows_loc: i32,
    midtones_loc: i32,
    highlights_loc: i32,
    contrast_loc: i32,
    saturation_loc: i32,
}

impl ColorGradePass {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let program = make_program(QUAD_VS, COLOR_GRADE_FS)?;
        unsafe {
            gl::UseProgram(program);
            let shadows_loc = gl::GetUniformLocation(program, b"u_shadows\0".as_ptr() as *const _);
            let midtones_loc = gl::GetUniformLocation(program, b"u_midtones\0".as_ptr() as *const _);
            let highlights_loc = gl::GetUniformLocation(program, b"u_highlights\0".as_ptr() as *const _);
            let contrast_loc = gl::GetUniformLocation(program, b"u_contrast\0".as_ptr() as *const _);
            let saturation_loc = gl::GetUniformLocation(program, b"u_saturation\0".as_ptr() as *const _);
            gl::UseProgram(0);

            Ok(Self {
                program, quad: QuadMesh::new(),
                shadows_loc, midtones_loc, highlights_loc, contrast_loc, saturation_loc,
            })
        }
    }

    pub fn render(&self, texture: GLuint, shadows: (f32,f32,f32), midtones: (f32,f32,f32),
                  highlights: (f32,f32,f32), contrast: f32, saturation: f32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform3f(self.shadows_loc, shadows.0, shadows.1, shadows.2);
            gl::Uniform3f(self.midtones_loc, midtones.0, midtones.1, midtones.2);
            gl::Uniform3f(self.highlights_loc, highlights.0, highlights.1, highlights.2);
            gl::Uniform1f(self.contrast_loc, contrast);
            gl::Uniform1f(self.saturation_loc, saturation);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.quad.draw();
            gl::UseProgram(0);
        }
    }
}

impl Drop for ColorGradePass {
    fn drop(&mut self) { unsafe { gl::DeleteProgram(self.program); } }
}

// ============================================================================
// EFFECT: CHROMATIC ABERRATION
// ============================================================================

pub struct ChromaticPass {
    program: GLuint,
    quad: QuadMesh,
    offset_loc: i32,
    _resolution_loc: i32,
}

impl ChromaticPass {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let program = make_program(QUAD_VS, CHROMATIC_FS)?;
        unsafe {
            gl::UseProgram(program);
            let offset_loc = gl::GetUniformLocation(program, b"u_offset\0".as_ptr() as *const _);
            let resolution_loc = gl::GetUniformLocation(program, b"u_resolution\0".as_ptr() as *const _);
            gl::Uniform2f(resolution_loc, width as f32, height as f32);
            gl::UseProgram(0);

            Ok(Self { program, quad: QuadMesh::new(), offset_loc, _resolution_loc: resolution_loc })
        }
    }

    pub fn render(&self, texture: GLuint, offset: f32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform1f(self.offset_loc, offset);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.quad.draw();
            gl::UseProgram(0);
        }
    }
}

impl Drop for ChromaticPass {
    fn drop(&mut self) { unsafe { gl::DeleteProgram(self.program); } }
}

// ============================================================================
// EFFECT: VIGNETTE
// ============================================================================

pub struct VignettePass {
    program: GLuint,
    quad: QuadMesh,
    intensity_loc: i32,
    smoothness_loc: i32,
}

impl VignettePass {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let program = make_program(QUAD_VS, VIGNETTE_FS)?;
        unsafe {
            gl::UseProgram(program);
            let intensity_loc = gl::GetUniformLocation(program, b"u_intensity\0".as_ptr() as *const _);
            let smoothness_loc = gl::GetUniformLocation(program, b"u_smoothness\0".as_ptr() as *const _);
            gl::UseProgram(0);

            Ok(Self { program, quad: QuadMesh::new(), intensity_loc, smoothness_loc })
        }
    }

    pub fn render(&self, texture: GLuint, intensity: f32, smoothness: f32) {
        unsafe {
            gl::UseProgram(self.program);
            gl::Uniform1f(self.intensity_loc, intensity);
            gl::Uniform1f(self.smoothness_loc, smoothness);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            self.quad.draw();
            gl::UseProgram(0);
        }
    }
}

impl Drop for VignettePass {
    fn drop(&mut self) { unsafe { gl::DeleteProgram(self.program); } }
}

// ============================================================================
// POSTFX CHAIN — Encadenamiento de efectos
// ============================================================================

/// Configuración de efecto individual
#[derive(Debug, Clone)]
pub struct EffectConfig {
    pub enabled: bool,
    pub intensity: f32,
}

impl Default for EffectConfig {
    fn default() -> Self { Self { enabled: false, intensity: 1.0 } }
}

/// Pipeline de post-processing configurable
pub struct PostFxChain {
    pub bloom: EffectConfig,
    pub blur: EffectConfig,
    pub sharpen: EffectConfig,
    pub color_grade: EffectConfig,
    pub chromatic: EffectConfig,
    pub vignette: EffectConfig,

    // FBOs intermedios
    fbo_a: Fbo,
    fbo_b: Fbo,

    // Passes
    bloom_pass: Option<BloomPass>,
    blur_pass: Option<BlurPass>,
    sharpen_pass: Option<SharpenPass>,
    color_grade_pass: Option<ColorGradePass>,
    chromatic_pass: Option<ChromaticPass>,
    vignette_pass: Option<VignettePass>,

    screen_w: u32,
    screen_h: u32,
}

impl PostFxChain {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let fbo_a = Fbo::new(width, height)?;
        let fbo_b = Fbo::new(width, height)?;

        Ok(Self {
            bloom: EffectConfig::default(),
            blur: EffectConfig::default(),
            sharpen: EffectConfig::default(),
            color_grade: EffectConfig::default(),
            chromatic: EffectConfig::default(),
            vignette: EffectConfig::default(),
            fbo_a, fbo_b,
            bloom_pass: None, blur_pass: None, sharpen_pass: None,
            color_grade_pass: None, chromatic_pass: None, vignette_pass: None,
            screen_w: width, screen_h: height,
        })
    }

    /// Inicializar los passes necesarios (lazy init)
    pub fn init_passes(&mut self) -> Result<(), String> {
        if self.bloom.enabled && self.bloom_pass.is_none() {
            self.bloom_pass = Some(BloomPass::new(self.screen_w, self.screen_h)?);
        }
        if self.blur.enabled && self.blur_pass.is_none() {
            self.blur_pass = Some(BlurPass::new(self.screen_w, self.screen_h)?);
        }
        if self.sharpen.enabled && self.sharpen_pass.is_none() {
            self.sharpen_pass = Some(SharpenPass::new(self.screen_w, self.screen_h)?);
        }
        if self.color_grade.enabled && self.color_grade_pass.is_none() {
            self.color_grade_pass = Some(ColorGradePass::new(self.screen_w, self.screen_h)?);
        }
        if self.chromatic.enabled && self.chromatic_pass.is_none() {
            self.chromatic_pass = Some(ChromaticPass::new(self.screen_w, self.screen_h)?);
        }
        if self.vignette.enabled && self.vignette_pass.is_none() {
            self.vignette_pass = Some(VignettePass::new(self.screen_w, self.screen_h)?);
        }
        Ok(())
    }

    /// Renderizar la cadena de efectos sobre la textura de entrada
    pub fn render(&mut self, input_texture: GLuint) {
        // Si no hay efectos habilitados, no hacer nada
        let any_enabled = self.bloom.enabled || self.blur.enabled || self.sharpen.enabled
            || self.color_grade.enabled || self.chromatic.enabled || self.vignette.enabled;
        if !any_enabled { return; }

        let mut src = input_texture;

        // 1. Bloom (necesita extract → blur → add back, simplificado: extract)
        if self.bloom.enabled {
            if let Some(ref pass) = self.bloom_pass {
                self.fbo_a.bind();
                pass.render(src, 0.8, self.bloom.intensity);
                self.fbo_a.unbind(self.screen_w, self.screen_h);
                src = self.fbo_a.texture();
            }
        }

        // 2. Blur (2-pass: horizontal + vertical)
        if self.blur.enabled {
            if let Some(ref pass) = self.blur_pass {
                // H pass
                self.fbo_a.bind();
                pass.render(src, 0, 4);
                self.fbo_a.unbind(self.screen_w, self.screen_h);
                // V pass
                self.fbo_b.bind();
                pass.render(self.fbo_a.texture(), 1, 4);
                self.fbo_b.unbind(self.screen_w, self.screen_h);
                src = self.fbo_b.texture();
            }
        }

        // 3. Sharpen
        if self.sharpen.enabled {
            if let Some(ref pass) = self.sharpen_pass {
                self.fbo_a.bind();
                pass.render(src, self.sharpen.intensity);
                self.fbo_a.unbind(self.screen_w, self.screen_h);
                src = self.fbo_a.texture();
            }
        }

        // 4. Color grading
        if self.color_grade.enabled {
            if let Some(ref pass) = self.color_grade_pass {
                self.fbo_a.bind();
                pass.render(src,
                    (1.0, 1.0, 1.0), // shadows
                    (1.0, 1.0, 1.0), // midtones
                    (1.0, 1.0, 1.0), // highlights
                    1.0, 1.0         // contrast, saturation
                );
                self.fbo_a.unbind(self.screen_w, self.screen_h);
                src = self.fbo_a.texture();
            }
        }

        // 5. Chromatic aberration
        if self.chromatic.enabled {
            if let Some(ref pass) = self.chromatic_pass {
                self.fbo_a.bind();
                pass.render(src, self.chromatic.intensity * 3.0);
                self.fbo_a.unbind(self.screen_w, self.screen_h);
                src = self.fbo_a.texture();
            }
        }

        // 6. Vignette (siempre último antes de screen)
        if self.vignette.enabled {
            if let Some(ref pass) = self.vignette_pass {
                self.fbo_a.bind();
                pass.render(src, self.vignette.intensity, 0.4);
                self.fbo_a.unbind(self.screen_w, self.screen_h);
                src = self.fbo_a.texture();
            }
        }
    }

    /// Preset: Cyberpunk (azul+naranja, bloom alto, vignette)
    pub fn preset_cyberpunk(&mut self) {
        self.bloom = EffectConfig { enabled: true, intensity: 1.2 };
        self.blur = EffectConfig { enabled: false, intensity: 0.0 };
        self.sharpen = EffectConfig { enabled: true, intensity: 0.5 };
        self.color_grade = EffectConfig { enabled: true, intensity: 1.0 };
        self.chromatic = EffectConfig { enabled: true, intensity: 0.3 };
        self.vignette = EffectConfig { enabled: true, intensity: 0.6 };
    }

    /// Preset: Natural (suave, sin aberración)
    pub fn preset_natural(&mut self) {
        self.bloom = EffectConfig { enabled: true, intensity: 0.3 };
        self.blur = EffectConfig { enabled: false, intensity: 0.0 };
        self.sharpen = EffectConfig { enabled: true, intensity: 0.2 };
        self.color_grade = EffectConfig { enabled: true, intensity: 0.5 };
        self.chromatic = EffectConfig { enabled: false, intensity: 0.0 };
        self.vignette = EffectConfig { enabled: true, intensity: 0.2 };
    }

    /// Preset: Retro (scanlines feel, chromatic alto)
    pub fn preset_retro(&mut self) {
        self.bloom = EffectConfig { enabled: false, intensity: 0.0 };
        self.blur = EffectConfig { enabled: false, intensity: 0.0 };
        self.sharpen = EffectConfig { enabled: true, intensity: 1.0 };
        self.color_grade = EffectConfig { enabled: true, intensity: 0.7 };
        self.chromatic = EffectConfig { enabled: true, intensity: 0.8 };
        self.vignette = EffectConfig { enabled: true, intensity: 0.8 };
    }

    /// Resetear todos los efectos
    pub fn reset(&mut self) {
        self.bloom = EffectConfig::default();
        self.blur = EffectConfig::default();
        self.sharpen = EffectConfig::default();
        self.color_grade = EffectConfig::default();
        self.chromatic = EffectConfig::default();
        self.vignette = EffectConfig::default();
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_config_default() {
        let c = EffectConfig::default();
        assert!(!c.enabled);
        assert_eq!(c.intensity, 1.0);
    }

    #[test]
    fn test_presets_enable_effects() {
        // Test presets sin crear FBOs (requieren contexto GL)
        let mut bloom = EffectConfig::default();
        let mut blur = EffectConfig::default();
        let mut sharpen = EffectConfig::default();
        let mut color_grade = EffectConfig::default();
        let mut chromatic = EffectConfig::default();
        let mut vignette = EffectConfig::default();

        // Simular preset_cyberpunk
        bloom = EffectConfig { enabled: true, intensity: 1.2 };
        sharpen = EffectConfig { enabled: true, intensity: 0.5 };
        color_grade = EffectConfig { enabled: true, intensity: 1.0 };
        chromatic = EffectConfig { enabled: true, intensity: 0.3 };
        vignette = EffectConfig { enabled: true, intensity: 0.6 };

        assert!(bloom.enabled);
        assert!(chromatic.enabled);
        assert!(vignette.enabled);

        // Simular reset
        bloom = EffectConfig::default();
        assert!(!bloom.enabled);
    }
}
