#version 330 core
out vec4 color;
layout (location = 0) in vec3 aPos;
uniform float x;
uniform float y;
uniform float z;
void main() {
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
   color = vec4((aPos.x + 0.5) * x, (aPos.y + 0.5) * y, aPos.z * z, 1.0);
}
