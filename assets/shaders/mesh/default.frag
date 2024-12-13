#version 450

in vec3 v_Color;
in vec3 v_Normal;

out vec4 fragColor;

uniform vec3 u_LightDirection = normalize(vec3(0.5, 1.0, 0.3));

void main() {

    float lightBrightness = dot(normalize(v_Normal), normalize(u_LightDirection));
    vec3 shadowColor = v_Color * 0.5;
    vec4 f_1 = vec4(mix(shadowColor, v_Color, lightBrightness), 1.0);
    vec4 f_2 = vec4(v_Normal, 1.0);
    fragColor = mix(f_1, f_2, 0.5);
}
