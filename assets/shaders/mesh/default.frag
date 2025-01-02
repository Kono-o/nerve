#version 450

in vec3 fCol;
in vec3 fNrm;
in vec2 fUVM;

uniform sampler2D tDif1;

out vec4 frag;

uniform vec3 uLight = normalize(vec3(0.5, 1.0, 0.3));

void main() {

    float light = 1.0 - dot(normalize(fNrm), normalize(uLight));
    vec4 texCol = texture(tDif1, fUVM * 40);
    vec4 difCol = texCol * vec4(fCol, 1.0);
    vec4 shadCol = difCol * 0.9;

    frag = mix(difCol, shadCol, light);
}
