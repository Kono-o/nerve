//VERT

#version 450

layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 vCol;
layout (location = 2) in vec2 vUVM;
layout (location = 3) in vec3 vNrm;

uniform mat4 uCamView;
uniform mat4 uCamProj;
uniform mat4 uMeshTfm;

out vec3 fCol;
out vec3 fNrm;
out vec2 fUVM;

void main() {
    fNrm = transpose(inverse(mat3(uMeshTfm))) * vNrm;
    fCol = vCol;
    fUVM = vUVM;

    gl_Position = uCamProj * uCamView * uMeshTfm * vec4(vPos, 1.0);
}


//FRAG

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
