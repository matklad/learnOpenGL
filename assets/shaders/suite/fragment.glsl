#version 330 core

in vec3 model_normal;
in vec3 model_position;
in vec3 light_position;

out vec4 color;

void main() {
    vec3 object_color = vec3(0.1, 0.2, 0.8);
    vec3 light_color = vec3(0.2, 0.2, 0.2);

    float ambient_strength = 0.05;
    vec3 amnbient = ambient_strength * light_color;

    float diffuse_strength = 0.5;
    vec3 normal = normalize(model_normal);
    vec3 light_direction = normalize(light_position - model_position);
    float diff = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = diffuse_strength * diff * light_color;

    float specular_strength = 0.5;
    vec3 view_dir = normalize(-model_position);
    vec3 reflect_dir = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light_color;

    vec3 result = (amnbient + diffuse + specular) * object_color;
    color = vec4(result, 1.0);
}