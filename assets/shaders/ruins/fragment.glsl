#version 330 core

uniform sampler2D texture_diffuse;
uniform vec3 color_specular;
uniform float shininess;
uniform sampler2D awesome;
uniform mat4 projector_view;
uniform mat4 projection;



in vec3 model_normal;
in vec3 model_position;
in vec2 model_texture;
in vec3 light_position;

in vec4 model_world;


out vec4 color;

void main() {
    vec3 model_color = vec3(texture(texture_diffuse, model_texture));
    vec3 normal = normalize(model_normal);
    vec3 light_direction = normalize(light_position - model_position);
    vec3 view_dir = normalize(-model_position);
    vec3 reflect_dir = reflect(-light_direction, normal);

    vec3 light_color = vec3(0.2, 0.2, 0.2);

    vec3 amnbient = model_color;

    float diff = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = diff * model_color;

    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    vec3 specular = color_specular * spec;

    vec3 result = (amnbient + diffuse + specular) * light_color;
    color = vec4(result, 1.0);

    vec4 h_model_projector = projection * projector_view * model_world;
    vec3 model_projector = h_model_projector.xyz / h_model_projector.w;

    if (all(lessThan(vec3(-1.0), model_projector)) && all(lessThan(model_projector, vec3(1.0)))) {
        vec2 tex_coord = (model_projector.xy + vec2(1.0)) / 2;
        color = 0.7 * color + 0.3 * texture(awesome, tex_coord);
    }

}