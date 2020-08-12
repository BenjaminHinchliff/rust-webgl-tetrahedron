attribute vec4 position;

uniform mat4 model_view_projection;

void main() {
    gl_Position = model_view_projection * position;
}
