#version 330 core

in vec2 position;
in vec2 texture;

out vec2 frag_texture;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    frag_texture = texture;
}
