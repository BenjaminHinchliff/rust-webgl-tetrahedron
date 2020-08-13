const wasmModulePromise = import('./pkg');
import vertexPath from './assets/shaders/tetra.vert';
import fragPath from './assets/shaders/tetra.frag';
import imgPath from './assets/img/cubetexture.png';
import modelPath from './assets/models/Box.glb';

/**
 * Main function that handles webgl instantiation and fetching of resources
 */
async function main() {
  const wasmModule = await wasmModulePromise;
  const {Tetra} = wasmModule;
  // get shader sources
  const [vertexSource, fragSource] = await Promise.all(
      [vertexPath, fragPath].map(async (path) => {
        return await (await fetch(path)).text();
      }));
  // get texture image
  const img = await new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => {
      resolve(image);
    };
    image.onerror = (err) => {
      reject(err);
    };
    image.src = imgPath;
  });
  const model = new Uint8Array(await (await fetch(modelPath)).arrayBuffer());
  const tetra = new Tetra(document.getElementById('webgl'))
      .add_vert_shader(vertexSource)
      .add_frag_shader(fragSource)
      .link_program()
      .load_gltf(model);
  tetra.draw();
  tetra.free();
}

main();
