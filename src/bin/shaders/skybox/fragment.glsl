#version 330 core

uniform samplerCube skybox;

in vec3 tex_coord;

out vec4 color;

void main() {
    vec3 coord = vec3(tex_coord.x, -tex_coord.y, tex_coord.z);
    color = texture(skybox, coord);
//    color = vec4(tex_coord, 1.0);
}