#type vertex
#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

#type fragment
#version 330 core
uniform vec3 uColor;

out vec4 fColor;

void main() {
    fColor = vec4(uColor, 1.0);
}