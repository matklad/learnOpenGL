#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 light;

in vec3 position;
in vec3 normal;

out vec3 model_position;
out vec3 model_normal;
out vec3 light_position;


void main() {
    mat4 view_model = view * model;
    vec4 hpos = vec4(position, 1.0);
    gl_Position = projection * view_model * hpos;
    model_position = vec3(view_model * hpos);
    model_normal = mat3(transpose(inverse(view_model))) * normal;
    light_position = vec3(view * vec4(light, 1.0));
}
