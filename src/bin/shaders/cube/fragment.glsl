#version 330 core

uniform vec3 camera_position;
uniform samplerCube skybox;

in vec3 world_position;
in vec3 world_normal;

out vec4 color;

void main() {
    vec3 view = normalize(world_position - camera_position);
    vec3 rview = reflect(view, normalize(world_normal));
    vec3 coord = vec3(rview.x, -rview.y, rview.z);
    color = texture(skybox, coord);
}