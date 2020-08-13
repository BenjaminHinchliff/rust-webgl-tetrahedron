attribute vec4 a_position;
attribute vec3 a_normal;
attribute vec2 a_tex_coord;

uniform mat4 u_normal_matrix;
uniform mat4 u_model_view_projection;

varying highp vec2 v_tex_coord;
varying highp vec3 v_lighting;

void main() {
    gl_Position = u_model_view_projection * a_position;
    
    v_tex_coord = a_tex_coord;

    highp vec3 ambient_light = vec3(0.3, 0.3, 0.3);
    highp vec3 directional_light_color = vec3(1, 1, 1);
    highp vec3 directional_vector = normalize(vec3(0.0, 1.0, 0.5));

    highp vec4 transformed_normal = u_normal_matrix * vec4(a_normal, 1.0);
    highp float directional = max(dot(transformed_normal.xyz, directional_vector), 0.0);
    v_lighting = ambient_light + (directional_light_color * directional);
}
