attribute vec4 a_position;
attribute vec2 a_tex_coord;

varying highp vec2 v_tex_coord;

uniform mat4 model_view_projection;

void main() {
    gl_Position = model_view_projection * a_position;
    v_tex_coord = a_tex_coord;
}
