#version 330 core

in vec2 v_TexCoords;
in vec4 v_Color;
flat in uint v_TextureID;

out vec4 FragColor;

uniform sampler2DArray u_TextureArray;

vec4 getTextureColor() {
  if (v_TextureID >= 16u) {
    return vec4(1.0);
  }

  ivec2 pixel_coords = ivec2(v_TexCoords);
  int layer_index = int(v_TextureID);

  vec3 uvl = vec3(v_TexCoords.x, v_TexCoords.y, float(v_TextureID));
  // return texture(u_TextureArray, uvl);
  return texelFetch(u_TextureArray, ivec3(pixel_coords, layer_index), 0);
}

void main() {
  FragColor = getTextureColor() * v_Color;
}
