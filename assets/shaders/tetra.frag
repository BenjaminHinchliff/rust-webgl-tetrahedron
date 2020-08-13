varying highp vec3 v_lighting;

void main() {
    highp vec4 color = vec4(1.0, 0.0, 0.0, 1.0);
    gl_FragColor = vec4(color.rgb * v_lighting, color.a);
}
