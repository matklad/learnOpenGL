#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 projector_view;



in vec3 position;

void main() {
    vec4 world = inverse(projection * projector_view) * vec4(position, 1);
    world /= world.w;
    gl_Position = projection * view * world;
}

