//VERT

#version 450

layout (location = 0) in vec3 vPos;
layout (location = 1) in vec4 vCol;
layout (location = 2) in vec2 vUVM;
layout (location = 3) in vec3 vNrm;

layout (location = 0) uniform mat4 uView;
layout (location = 1) uniform mat4 uProj;
layout (location = 2) uniform mat4 uTfm;

layout (location = 0) out vec4 fCol;
layout (location = 1) out vec3 fNrm;
layout (location = 2) out vec2 fUVM;

void main() {
    fNrm = transpose(inverse(mat3(uTfm))) * vNrm;
    fCol = vCol;
    fUVM = vUVM;

    gl_Position = uProj * uView * uTfm * vec4(vPos, 1.0);
}


//FRAG

#version 450

layout (location = 0) in vec4 fCol;
layout (location = 1) in vec3 fNrm;
layout (location = 2) in vec2 fUVM;

layout (location = 3) uniform vec3 uLight = normalize(vec3(0.5, 1.0, 0.3));

layout (location = 0) out vec4 fragPIXEL;

uniform sampler2D Tex0;

void main() {
    vec2 coord = gl_FragCoord.xy / 20;
    float light = 1.0 - dot(normalize(fNrm), normalize(uLight));

    vec4 checkerTex = texture(Tex0, coord);
    vec3 CRIMSON = vec3(0.9, 0.2, 0.3);
    vec3 DARKER_CRIMSON = vec3(0.85, 0.175, 0.275);

    vec3 color = mix(CRIMSON, DARKER_CRIMSON, checkerTex.rgb);
    vec3 shadow = color * 0.75;
    vec3 final = mix(color, shadow, light);

    fragPIXEL = vec4(final, 1.0);
}
