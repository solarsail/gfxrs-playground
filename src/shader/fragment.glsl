#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoord;
in mat4 ModelView;

struct DirLight {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    vec4 dir;
};

struct PointLight {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    vec4 pos;
    float a0, a1, a2, pad;
};

uniform DirLight dirLight;
#define NR_POINT_LIGHTS 4
uniform PointLight pointLights[NR_POINT_LIGHTS];

uniform float material_shininess;
uniform sampler2D material_diffuse;
uniform sampler2D material_specular;
uniform vec3 viewPos;

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir)
{
    vec3 lightDir = normalize(-light.dir);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // combine results
    vec3 ambient  = light.ambient  * vec3(texture(material_diffuse, TexCoords));
    vec3 diffuse  = light.diffuse  * diff * vec3(texture(material_diffuse, TexCoords));
    vec3 specular = light.specular * spec * vec3(texture(material_specular, TexCoords));
    return (ambient + diffuse + specular);
}

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightDir = normalize(light.position - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    // attenuation
    float distance    = length(light.pos - fragPos);
    float attenuation = 1.0 / (light.a0 + light.a1 * distance + 
  			     light.a2 * (distance * distance));    
    // combine results
    vec3 ambient  = light_ambient  * vec3(texture(material_diffuse, TexCoords));
    vec3 diffuse  = light_diffuse  * diff * vec3(texture(material_diffuse, TexCoords));
    vec3 specular = light_specular * spec * vec3(texture(material_specular, TexCoords));
    ambient  *= attenuation;
    diffuse  *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

void main()
{
    // properties
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    // phase 1: Directional lighting
    vec3 result = CalcDirLight(dirLight, norm, viewDir);
    // phase 2: Point lights
    for(int i = 0; i < NR_POINT_LIGHTS; i++)
        result += CalcPointLight(pointLights[i], norm, FragPos, viewDir);    
    // phase 3: Spot light
    //result += CalcSpotLight(spotLight, norm, FragPos, viewDir);    
    
    FragColor = vec4(result, 1.0);
}
