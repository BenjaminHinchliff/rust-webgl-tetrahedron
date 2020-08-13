use wasm_bindgen::JsValue;
use web_sys::{
    HtmlImageElement, WebGlRenderingContext, WebGlTexture,
};

use super::WebGl;

pub struct Texture2D {
    gl: WebGl,
    texture: WebGlTexture,
}

impl Texture2D {
    pub fn new(gl: &WebGl, image: &HtmlImageElement) -> Result<Texture2D, JsValue> {
        let texture = gl.create_texture().ok_or_else(|| "unable to create texture")?;
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

        gl.tex_image_2d_with_u32_and_u32_and_image(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            image,
        )?;

        gl.generate_mipmap(WebGlRenderingContext::TEXTURE_2D);

        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

        Ok(Texture2D { gl: gl.clone(), texture })
    }

    pub fn bind(&self) {
        self.gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
    }

    pub fn unbind(&self) {
        self.gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
    }
}
