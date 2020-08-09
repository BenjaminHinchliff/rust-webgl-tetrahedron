use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let title = document.get_element_by_id("title").expect("#title html element should exist");

    title.set_text_content(Some("Hello from rust!"));

    Ok(())
}
