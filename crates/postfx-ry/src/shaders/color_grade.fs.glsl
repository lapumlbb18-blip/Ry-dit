#version 100
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_texture;
uniform vec3 u_shadows;
uniform vec3 u_midtones;
uniform vec3 u_highlights;
uniform float u_contrast;
uniform float u_saturation;

void main() {
    vec3 color = texture2D(u_texture, v_uv).rgb;

    // Contrast
    color = (color - 0.5) * u_contrast + 0.5;

    // Saturation
    float lum = dot(color, vec3(0.299, 0.587, 0.114));
    color = mix(vec3(lum), color, u_saturation);

    // Color grading (shadows, midtones, highlights)
    float brightness = dot(color, vec3(0.299, 0.587, 0.114));
    vec3 graded = color;

    if (brightness < 0.33) {
        graded *= u_shadows;
    } else if (brightness < 0.66) {
        graded *= u_midtones;
    } else {
        graded *= u_highlights;
    }

    gl_FragColor = vec4(clamp(graded, 0.0, 1.0), 1.0);
}
