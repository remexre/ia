#version 450

uniform fragParams {
	bool textured;
	vec3 ambient;
	vec3 diffuse;
};
layout(binding = 0) uniform sampler2D tex;

layout(location = 10) in vec3 adjNormal;
layout(location = 11) in vec3 lightDir;
layout(location = 12) in vec2 texcoords;

layout(location = 0) out vec4 outColor;

void main() {
	vec3 ambientC;
	vec3 diffuseC;

	if(textured) {
		ambientC = diffuseC = texture(tex, texcoords).rgb;
	} else {
		ambientC = ambient;
		diffuseC = diffuse;
	}

	ambientC *= 0.1;
	diffuseC *= max(dot(-lightDir, adjNormal), 0.0);

	outColor = vec4(ambientC + diffuseC, 1.0);
}
