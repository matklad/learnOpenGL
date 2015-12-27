#version 330 core

uniform mat4 view;
uniform mat4 projection;

in vec3 position;

out vec3 tex_coord;

void main() {
    mat3 non_translation = mat3(view);
    gl_Position = projection * mat4(non_translation) * vec4(position, 1.0);
    tex_coord = position;
}

