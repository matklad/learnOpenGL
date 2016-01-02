#version 330

uniform sampler2D albedo;
uniform sampler2D specular;
uniform sampler2D shininess;
uniform sampler2D normal;
uniform sampler2D position;
uniform vec3 light_color;


in vec2 frag_texture;
in vec3 light_position;

out vec4 color;


void main() {
    vec3 model_albedo = texture(albedo, frag_texture).xyz;
    vec3 model_specular = texture(specular, frag_texture).xyz;
    float model_shininess = texture(shininess, frag_texture).x;

    float shininess = 32.0;
    vec3 model_position = texture(position, frag_texture).xyz;
    vec3 model_normal = texture(normal, frag_texture).xyz;
    vec3 view_dir = normalize(-model_position);
    vec3 light_direction = normalize(light_position - model_position);
    vec3 reflect_dir = reflect(-light_direction, model_normal);


    vec3 amnbient = model_albedo;

    float diff = max(dot(model_normal, light_direction), 0.0);
    vec3 diffuse = diff * model_albedo;

    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), model_shininess);
    vec3 specular = model_specular * spec;

    vec3 result = (amnbient + diffuse + specular) * light_color;
    color = vec4(result, 1.0);
}
