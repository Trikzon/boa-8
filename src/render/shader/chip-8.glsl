#type vertex
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aPixelPos;

out vec2 vPixelPos;

void main() {
    vPixelPos = aPixelPos;
    gl_Position = vec4(aPos.xyz, 1.0);
}

#type fragment
#version 330 core
uniform uint[64] uPixels;

in vec2 vPixelPos;

out vec4 fColor;

int WIDTH = 64;
int HEIGHT = 32;

void main() {
    bool is_bright = bool((uPixels[int(vPixelPos.x * WIDTH)] << int((vPixelPos.y * HEIGHT))) >> HEIGHT - 1);

    if (is_bright) {
        fColor = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        fColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}