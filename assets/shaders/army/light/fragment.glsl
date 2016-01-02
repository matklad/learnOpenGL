#version 330

uniform sampler2D albedo;

in vec2 frag_texture;
out vec4 color;


void main() {
    color = texture(albedo, frag_texture);
}
