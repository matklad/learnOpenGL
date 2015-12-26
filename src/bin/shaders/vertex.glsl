#version 330

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;


in vec3 position;
in vec3 color;
in vec2 texture;

out vec3 our_color;
out vec2 tex_coord;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    our_color = color;
    tex_coord = texture;
}

