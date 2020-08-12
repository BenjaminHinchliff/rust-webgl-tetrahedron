use std::ops::Deref;

use wasm_bindgen::JsValue;
use web_sys::{WebGlRenderingContext, WebGlShader};

use super::WebGl;

pub struct Shader {
    gl: WebGl,
    shader: WebGlShader,
}

impl Shader {
    pub fn new(gl: &WebGl, type_: u32, source: &str) -> Result<Shader, JsValue> {
        let shader = compile_shader(gl, type_, source)?;
        Ok(Shader {
            gl: gl.clone(),
            shader,
        })
    }
}

impl Deref for Shader {
    type Target = WebGlShader;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(Some(&self.shader));
    }
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("unknown error creating shader")))
    }
}
