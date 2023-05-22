#version 150 core

in vec3 pos;

uniform float scl;

void main(void) {
	gl_Position = vec4(pos.x*scl, pos.y*scl, pos.z*scl, 1.0f);
}
