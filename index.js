const wasm_module_promise = import('./pkg');
import vertex_path from './assets/tetra.vert';
import frag_path from './assets/tetra.frag';

async function main() {
    const wasm_module = await wasm_module_promise;
    const { Tetra } = wasm_module;
    const vertex_source = await (await fetch(vertex_path)).text();
    const frag_source = await (await fetch(frag_path)).text();
    new Tetra(document.getElementById('webgl'))
        .add_vert_shader(vertex_source)
        .add_frag_shader(frag_source)
        .link_program()
        .add_vertices([
            -0.7, -0.7, 0.0, // bottom left
            0.7, -0.7, 0.0, // bottom right
            0.0, 0.7, 0.0, // top
        ])
        .draw();
}

main();
