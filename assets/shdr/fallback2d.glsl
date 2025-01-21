//VERTEX

#version 450

layout (location = 0) in vec3 vPos;
layout (location = 1) in vec4 vCol;
layout (location = 2) in vec2 vUVM;

layout (location = 0) uniform mat4 uProj;
layout (location = 1) uniform mat4 uTfm;
layout (location = 2) uniform uint uLayer;

layout (location = 0) out vec4 fCol;
layout (location = 1) out vec2 fUVM;

void main() {
    fCol = vCol,
    fUVM = vUVM;
    gl_Position = uProj * uTfm * vec4(vPos.xy, uLayer * 0.001, 1.0);
}

//FRAG

#version 450

layout (location = 0) in vec3 fCol;
layout (location = 1) in vec2 fUVM;

layout (location = 0) out vec4 fragPIXEL;

uniform sampler2D Tex0;

void main() {
    vec2 coord = gl_FragCoord.xy / 20;
    vec4 checkerTex = texture(Tex0, coord);
    vec3 DARK_RED = vec3(0.4, 0.05, 0.1);
    vec3 DARKER_RED = vec3(0.35, 0.04, 0.09);

    vec3 color = mix(DARK_RED, DARKER_RED, checkerTex.rgb);

    fragPIXEL = vec4(color, 0.95);
}

