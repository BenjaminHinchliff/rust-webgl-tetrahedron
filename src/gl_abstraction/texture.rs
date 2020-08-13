use wasm_bindgen::JsValue;
use web_sys::{ImageData, WebGlRenderingContext as WebGlCtx, WebGlTexture};

use super::WebGl;

pub struct Texture2D {
    gl: WebGl,
    texture: WebGlTexture,
}

impl Texture2D {
    pub fn new(gl: &WebGl, image: &ImageData) -> Result<Texture2D, JsValue> {
        let texture = gl
            .create_texture()
            .ok_or_else(|| "unable to create texture")?;
        gl.bind_texture(WebGlCtx::TEXTURE_2D, Some(&texture));

        gl.tex_image_2d_with_u32_and_u32_and_image_data(
            WebGlCtx::TEXTURE_2D,
            0,
            WebGlCtx::RGBA as i32,
            WebGlCtx::RGBA,
            WebGlCtx::UNSIGNED_BYTE,
            image,
        )?;

        if is_power_of_2(image.width()) && is_power_of_2(image.height()) {
            gl.generate_mipmap(WebGlCtx::TEXTURE_2D);
        } else {
            gl.tex_parameteri(
                WebGlCtx::TEXTURE_2D,
                WebGlCtx::TEXTURE_WRAP_S,
                WebGlCtx::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                WebGlCtx::TEXTURE_2D,
                WebGlCtx::TEXTURE_WRAP_T,
                WebGlCtx::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                WebGlCtx::TEXTURE_2D,
                WebGlCtx::TEXTURE_MIN_FILTER,
                WebGlCtx::LINEAR as i32,
            );
        }

        gl.bind_texture(WebGlCtx::TEXTURE_2D, None);

        Ok(Texture2D {
            gl: gl.clone(),
            texture,
        })
    }

    pub fn bind(&self) {
        self.gl
            .bind_texture(WebGlCtx::TEXTURE_2D, Some(&self.texture));
    }

    pub fn unbind(&self) {
        self.gl.bind_texture(WebGlCtx::TEXTURE_2D, None);
    }
}

fn is_power_of_2(value: u32) -> bool {
    (value & (value - 1)) == 0
}
