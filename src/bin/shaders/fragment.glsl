#version 330

uniform vec3 object_color;
uniform vec3 light_color;

out vec4 color;

void main() {
    color = vec4(object_color * light_color, 1.0);
}
