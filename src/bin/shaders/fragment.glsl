#version 330

uniform sampler2D tex;

in vec3 our_color;
in vec2 tex_coord;

out vec4 color;

void main() {
    color = texture(tex, tex_coord);

}
