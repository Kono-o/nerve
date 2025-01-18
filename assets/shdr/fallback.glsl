//VERT

#version 450

layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 vCol;
layout (location = 2) in vec2 vUVM;
layout (location = 3) in vec3 vNrm;

layout (location = 0) uniform mat4 uCamView;
layout (location = 1) uniform mat4 uCamProj;
layout (location = 2) uniform mat4 uMeshTfm;

layout (location = 0) out vec3 fCol;
layout (location = 1) out vec3 fNrm;
layout (location = 2) out vec2 fUVM;

void main() {
    fNrm = transpose(inverse(mat3(uMeshTfm))) * vNrm;
    fCol = vCol;
    fUVM = vUVM;

    gl_Position = uCamProj * uCamView * uMeshTfm * vec4(vPos, 1.0);
}


//FRAG

#version 450

layout (location = 0) in vec3 fCol;
layout (location = 1) in vec3 fNrm;
layout (location = 2) in vec2 fUVM;

layout (location = 0) out vec4 frag;

layout (location = 3) uniform vec3 uLight = normalize(vec3(0.5, 1.0, 0.3));

uniform sampler2D Tex0;

void main() {
    vec2 fragCoord = gl_FragCoord.xy / 20;
    float light = 1.0 - dot(normalize(fNrm), normalize(uLight));

    vec4 texCol = texture(Tex0, fragCoord);
    vec4 MAGENTA = vec4(1.0, 0.0, 1.0, 1.0);
    vec4 PURPLE = vec4(0.5, 0.0, 0.5, 1.0);
    vec4 difCol = mix(MAGENTA, PURPLE, texCol);

    vec4 shadCol = difCol * 0.4;

    frag = mix(difCol, shadCol, light);
}
