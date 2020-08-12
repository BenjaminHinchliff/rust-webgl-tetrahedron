const wasmModulePromise = import('./pkg');
import vertexPath from './assets/shaders/tetra.vert';
import fragPath from './assets/shaders/tetra.frag';
import imgPath from './assets/img/cubetexture.png';

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
  const img = await new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => {
      resolve(image);
    };
    image.src = imgPath;
  });
  const tetra = new Tetra(document.getElementById('webgl'))
      .add_vert_shader(vertexSource)
      .add_frag_shader(fragSource)
      .link_program()
      .set_texture(img)
      .add_vertices([
        -1.0, -1.0, 0.0, 0.0, 0.0, // bottom left
        1.0, -1.0, 0.0, 1.0, 0.0, // bottom right
        1.0, 1.0, 0.0, 1.0, 1.0, // top right
        -1.0, 1.0, 0.0, 0.0, 1.0, // top left
      ])
      .add_indices([
        0, 1, 3, // first
        1, 2, 3, // second
      ]);
  tetra.draw();
  tetra.free();
}

main();
