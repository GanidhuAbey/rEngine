#version 330 core

out vec4 Color;

uniform vec4 fragColor;

void main() {
    Color = fragColor;
}