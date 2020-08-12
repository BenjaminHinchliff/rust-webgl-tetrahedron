varying highp vec2 v_tex_coord;

uniform sampler2D u_sampler;

void main() {
    gl_FragColor = texture2D(u_sampler, v_tex_coord * vec2(1, -1));
}
