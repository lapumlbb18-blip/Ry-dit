#version 100
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform int u_direction; // 0 = horizontal, 1 = vertical
uniform int u_radius;

void main() {
    vec2 texel = 1.0 / u_resolution;
    vec4 sum = vec4(0.0);
    float total = 0.0;

    // Gaussian weights (simplified)
    float weights[5];
    weights[0] = 0.227027;
    weights[1] = 0.1945946;
    weights[2] = 0.1216216;
    weights[3] = 0.054054;
    weights[4] = 0.016216;

    sum += texture2D(u_texture, v_uv) * weights[0];
    total += weights[0];

    for (int i = 1; i < 5 && i <= u_radius; i++) {
        float w = weights[i];
        vec2 offset;
        if (u_direction == 0) {
            offset = vec2(float(i), 0.0) * texel;
        } else {
            offset = vec2(0.0, float(i)) * texel;
        }
        sum += texture2D(u_texture, v_uv + offset) * w;
        sum += texture2D(u_texture, v_uv - offset) * w;
        total += w * 2.0;
    }

    gl_FragColor = sum / total;
}
