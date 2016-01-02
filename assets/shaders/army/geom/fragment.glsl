#version 330 core

uniform sampler2D texture_diffuse;
uniform vec3 color_specular;
uniform float shininess;


in vec3 model_normal;
in vec3 model_position;
in vec2 model_texture;

out vec4 albedo;
out vec4 specular_shininess;
out vec4 normal;
out vec4 position;

void main() {
    albedo = texture(texture_diffuse, model_texture);
    specular_shininess = vec4(color_specular, shininess);
    normal = vec4(normalize(model_normal), 0.0);
    position = vec4(model_position, 0.0);

//    vec3 normal = normalize(model_normal);
//    vec3 light_direction = normalize(light_position - model_position);
//    vec3 view_dir = normalize(-model_position);
//    vec3 reflect_dir = reflect(-light_direction, normal);
//
//    vec3 light_color = vec3(0.2, 0.2, 0.2);
//
//
//    float diff = max(dot(normal, light_direction), 0.0);
//    vec3 diffuse = diff * model_color;
//
//    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
//    vec3 specular = color_specular * spec;
//
//    vec3 result = (amnbient + diffuse + specular) * light_color;
//    color = vec4(result, 1.0);
}