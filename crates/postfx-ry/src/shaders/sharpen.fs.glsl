#version 100
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform float u_amount;

void main() {
    vec2 texel = 1.0 / u_resolution;

    // Sharpen kernel (laplacian)
    vec4 center = texture2D(u_texture, v_uv) * (1.0 + 4.0 * u_amount);
    vec4 top    = texture2D(u_texture, v_uv + vec2(0.0, -1.0) * texel) * u_amount;
    vec4 bottom = texture2D(u_texture, v_uv + vec2(0.0, 1.0) * texel) * u_amount;
    vec4 left   = texture2D(u_texture, v_uv + vec2(-1.0, 0.0) * texel) * u_amount;
    vec4 right  = texture2D(u_texture, v_uv + vec2(1.0, 0.0) * texel) * u_amount;

    gl_FragColor = center - top - bottom - left - right;
}
