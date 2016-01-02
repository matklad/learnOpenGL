#version 330 core

uniform mat4 view;
uniform vec3 light;

in vec2 position;
in vec2 texture;

out vec2 frag_texture;
out vec3 light_position;


void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    frag_texture = texture;
    light_position = vec3(view * vec4(light, 1.0));
}
