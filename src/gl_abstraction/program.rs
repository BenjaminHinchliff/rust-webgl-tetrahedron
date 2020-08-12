use std::ops::Deref;

use wasm_bindgen::JsValue;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use super::shader::Shader;
use super::WebGl;

pub struct Program {
    gl: WebGl,
    program: WebGlProgram,
}

impl Program {
    pub fn new(gl: &WebGl, shaders: &[Shader]) -> Result<Program, JsValue> {
        let program = link_program(gl, shaders)?;
        Ok(Program {
            gl: gl.clone(),
            program,
        })
    }

    pub fn set_used(&self) {
        self.gl.use_program(Some(&self.program));
    }

    pub fn set_unused(&self) {
        self.gl.use_program(None);
    }
}

impl Deref for Program {
    type Target = WebGlProgram;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}

fn link_program(gl: &WebGlRenderingContext, shaders: &[Shader]) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    for shader in shaders {
        gl.attach_shader(&program, shader);
    }
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
