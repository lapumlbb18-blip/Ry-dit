#version 100
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_texture;
uniform float u_intensity;
uniform float u_smoothness;

void main() {
    vec3 color = texture2D(u_texture, v_uv).rgb;

    // Vignette: darken edges
    vec2 center = v_uv - 0.5;
    float dist = length(center);
    float vignette = smoothstep(0.8, u_smoothness, dist);
    color *= 1.0 - vignette * u_intensity;

    gl_FragColor = vec4(color, 1.0);
}
