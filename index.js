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
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
      ])
      .add_indices([
        0, 1, 3, // first
        1, 2, 3, // second
      ])
      .draw();
}

main();
