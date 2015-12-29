#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

in vec3 position;

out vec3 world_position;

void main() {
    vec4 hpos = vec4(position, 1.0);
    gl_Position = projection * view * model * hpos;
    world_position = vec3(model * hpos);
}

