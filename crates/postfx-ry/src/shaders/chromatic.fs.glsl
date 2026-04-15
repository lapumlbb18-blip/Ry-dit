#version 100
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform float u_offset;

void main() {
    vec2 center = v_uv - 0.5;
    float dist = length(center);
    vec2 dir = normalize(center) * u_offset * dist;

    float r = texture2D(u_texture, v_uv + dir).r;
    float g = texture2D(u_texture, v_uv).g;
    float b = texture2D(u_texture, v_uv - dir).b;

    gl_FragColor = vec4(r, g, b, 1.0);
}
