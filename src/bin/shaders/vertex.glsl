#version 330

in vec3 position;
in vec3 color;
in vec2 texture;

out vec3 our_color;
out vec2 tex_coord;

void main() {
    gl_Position = vec4(position, 1.0);
    our_color = color;
    tex_coord = texture;
}

