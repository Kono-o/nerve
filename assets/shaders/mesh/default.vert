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

