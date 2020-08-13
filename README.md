# Webgl Rendering with Rust WebAssembly

This one was fun - it's rendering with webgl from rust compiled to WebAssembly (WASM) with the help (primarily) of the excellent crates [wasm-bindgen](https://crates.io/crates/wasm-bindgen) and [gltf](https://crates.io/crates/gltf) (seriously the rust community is amazing).

## Notes

* the code loads a model from a .glb file made in blender but the code that does it isn't robust. It's just there because I didn't want to enter all the vertices, normals, indices, and tex-coords manually because that seems really tedious, and it was fun to put together some model loading code (not to mention how easy the model loader made it)
* The wasm build isn't optimized - I suspect that it could be made a lot smaller with a little extra effort and a lot less logging
