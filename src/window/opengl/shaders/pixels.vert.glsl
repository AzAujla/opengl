#version 330 core

layout(location = 0) in vec2 a_Position;
layout(location = 1) in vec2 a_TexCoords;
layout(location = 2) in vec4 a_Color;
layout(location = 3) in uint a_TextureID;

out vec2 v_TexCoords;
out vec4 v_Color;
flat out uint v_TextureID;

uniform mat4 u_Projection;

void main() {
  v_TexCoords = a_TexCoords;
  v_Color = a_Color;
  v_TextureID = a_TextureID;

  gl_Position = u_Projection * vec4(a_Position, 0.0, 1.0);
}
