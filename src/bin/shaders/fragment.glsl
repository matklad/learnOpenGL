#version 330

uniform sampler2D tex1;
uniform sampler2D tex2;

in vec3 our_color;
in vec2 tex_coord;

out vec4 color;

void main() {
    color = mix(texture(tex1, tex_coord),
                texture(tex2, tex_coord),
                0.2);
}
