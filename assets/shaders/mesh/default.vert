#version 450

layout (location = 0) in vec3 a_Position;
layout (location = 1) in vec3 a_Color;
//layout (location = 2) in vec3 a_UVMap;
layout (location = 2) in vec3 a_Normals;

uniform mat4 u_MeshTransform;
uniform mat4 u_CamView;
uniform mat4 u_CamProj;

out vec3 v_Color;
out vec3 v_Normal;

void main() {
    gl_Position = u_CamProj * u_CamView * u_MeshTransform * vec4(a_Position, 1.0);
    v_Color = a_Color;
    v_Normal = transpose(inverse(mat3(u_MeshTransform))) * a_Normals;
}

