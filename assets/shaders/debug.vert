#version 450

uniform matrices {
	mat4 model;
	mat4 view;
	mat4 proj;
};

layout(location = 0) in vec3 xyz;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(location = 10) out vec3 adjNormal;
layout(location = 11) out vec3 lightDir;
layout(location = 12) out vec2 texcoords;

const vec3 inLightDir = normalize(vec3(1.0, 2.0, 1.0));

void main() {
   gl_Position = proj * view * model * vec4(xyz, 1.0);

   vec4 norm4 = transpose(inverse(view * model)) * vec4(normal, 0.0);
   adjNormal = normalize(norm4.xyz);

   lightDir = (view * vec4(inLightDir, 0.0)).xyz;

   texcoords = uv;
}
