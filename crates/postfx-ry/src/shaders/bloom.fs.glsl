#version 100
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform float u_threshold;
uniform float u_intensity;

void main() {
    vec2 texel = 1.0 / u_resolution;
    vec4 sum = vec4(0.0);

    // 9-tap box blur
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            vec2 offset = vec2(float(x), float(y)) * texel;
            vec4 c = texture2D(u_texture, v_uv + offset);
            float brightness = dot(c.rgb, vec3(0.299, 0.587, 0.114));
            if (brightness > u_threshold) {
                sum += c;
            }
        }
    }
    sum /= 9.0;
    gl_FragColor = vec4(sum.rgb * u_intensity, sum.a);
}
