varying highp vec2 v_tex_coord;
varying highp vec3 v_lighting;

uniform sampler2D u_sampler;

void main() {
    highp vec4 texel_color = texture2D(u_sampler, v_tex_coord);
    gl_FragColor = vec4(texel_color.rgb * v_lighting, texel_color.a);
}
