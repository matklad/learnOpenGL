#version 330 core

uniform vec3 camera_position;
uniform samplerCube skybox;

in vec3 tex_coord;

out vec4 color;

void main() {
    vec3 coord = vec3(tex_coord.x, -tex_coord.y, tex_coord.z);
    color = texture(skybox, coord);
}