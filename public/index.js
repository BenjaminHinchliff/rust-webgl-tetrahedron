// assets
import vertexPath from '../assets/shaders/tetra.vert';
import fragPath from '../assets/shaders/tetra.frag';
import modelPath from '../assets/models/ico.glb';
// css
import './index.css';

// wasm
const wasmModulePromise = import('../pkg');

/**
 * Main function that handles webgl instantiation and fetching of resources
 */
async function main() {
  const canvas = document.getElementById('webgl');

  const wasmModule = await wasmModulePromise;
  const {Tetra} = wasmModule;
  const fetchPromises = [vertexPath, fragPath].map(async (path) => {
    return await (await fetch(path)).text();
  });
  fetchPromises.push((async () => {
    return new Uint8Array(await (await fetch(modelPath)).arrayBuffer());
  })());
  // get shader sources
  const [vertexSource, fragSource, model] = await Promise.all(fetchPromises);
  const tetra = new Tetra(canvas)
      .add_vert_shader(vertexSource)
      .add_frag_shader(fragSource)
      .link_program()
      .load_gltf(model);

  const resize = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    tetra.refresh_viewport(canvas);
  };
  window.addEventListener('resize', resize);
  resize();

  /**
   * a single draw step
   * @param {DOMHighResTimeStamp} timestamp
   */
  function step(timestamp) {
    tetra.draw(timestamp);
    window.requestAnimationFrame(step);
  }
  window.requestAnimationFrame(step);
}

main();
