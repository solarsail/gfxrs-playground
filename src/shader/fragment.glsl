#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoord;
in mat4 ModelView;

struct DirectionalLight {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    vec3 dir;
};

struct PointLight {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    vec3 pos;
    float a0, a1, a2;
};

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

uniform DirectionalLight dirLight;
#define NR_POINT_LIGHTS 4  
uniform PointLight pointLights[NR_POINT_LIGHTS];
uniform Material material;

vec3 CaldDirLight(DirectionalLight light, vec3 normal, vec3 frag_pos) {
    vec3 lightDir = normalize(-light.dir);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * light.diffuse * vec3(texture(material.diffuse, TexCoord));

    vec3 viewDir = normalize(-frag_pos);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = spec * light.specular * vec3(texture(material.specular, TexCoord));

    return (ambient + diffuse + specular);
}

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, mat4 modelview)
{
    light_pos = modelview * light.pos;
    vec3 lightDir = normalize(light_pos - fragPos);
    vec3 viewDir = normalize(-fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // attenuation
    float distance    = length(light_pos - fragPos);
    float attenuation = 1.0 / (light.a0 + light.a1 * distance + 
  			     light.a2 * (distance * distance));    
    // combine results
    vec3 ambient  = light.ambient  * vec3(texture(material.diffuse, TexCoords));
    vec3 diffuse  = light.diffuse  * diff * vec3(texture(material.diffuse, TexCoords));
    vec3 specular = light.specular * spec * vec3(texture(material.specular, TexCoords));
    ambient  *= attenuation;
    diffuse  *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

void main()
{
    vec3 norm = normalize(Normal);

    // phase 1: Directional lighting
    vec3 result = CalcDirLight(dirLight, norm);
    // phase 2: Point lights
    for(int i = 0; i < NR_POINT_LIGHTS; i++)
        result += CalcPointLight(pointLights[i], norm, FragPos, ModelView);
    // phase 3: Spot light
    //result += CalcSpotLight(spotLight, norm, FragPos, viewDir);    
    
    FragColor = vec4(result, 1.0);
}
