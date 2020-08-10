const wasmModulePromise = import('./pkg');
import vertexPath from './assets/tetra.vert';
import fragPath from './assets/tetra.frag';

/**
 * Main function that handles webgl instantiation and linking with wasm
 */
async function main() {
  const wasmModule = await wasmModulePromise;
  const {Tetra} = wasmModule;
  const [vertexSource, fragSource] = await Promise.all(
      [vertexPath, fragPath].map(async (path) => {
        return await (await fetch(path)).text();
      }));
  new Tetra(document.getElementById('webgl'))
      .add_vert_shader(vertexSource)
      .add_frag_shader(fragSource)
      .link_program()
      .add_vertices([
        -0.7, -0.7, 0.0, // bottom left
        0.7, -0.7, 0.0, // bottom right
        0.0, 0.7, 0.0, // top
      ])
      .draw();
}

main();
