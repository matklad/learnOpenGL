#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

in vec3 position;
in vec3 normal;

out vec3 world_position;
out vec3 world_normal;

void main() {
    vec4 hpos = vec4(position, 1.0);
    gl_Position = projection * view * model * hpos;
    world_position = vec3(model * hpos);
    world_normal = vec3(transpose(inverse(model)) * vec4(normal, 1));
}

