use web_sys::WebGlUniformLocation;
use wasm_bindgen::prelude::*;

use crate::gl_abstraction::{Program, WebGl};

// TODO: either generate all of this, or use a macro to help a bit
pub struct AttribLocs {
    pub position: u32,
    pub normal: u32,
    pub tex_coord: u32,
}

impl AttribLocs {
    pub fn new(gl: &WebGl, program: &Program) -> Result<AttribLocs, JsValue> {
        let position = gl.get_attrib_location(program, "a_position");
        if position == -1 {
            return Err("position attribute doesn't exist".into());
        }
        let normal = gl.get_attrib_location(program, "a_normal");
        if normal == -1 {
            return Err("normal attribute doesn't exist".into());
        }
        let tex_coord = gl.get_attrib_location(program, "a_tex_coord");
        if tex_coord == -1 {
            return Err("tex_coord attribute doesn't exist".into());
        }
        Ok(AttribLocs {
            position: position as u32,
            normal: normal as u32,
            tex_coord: tex_coord as u32,
        })
    }
}

pub struct UniformLocs {
    pub model_view_projection: WebGlUniformLocation,
    pub normal_matrix: WebGlUniformLocation,
    pub sampler: WebGlUniformLocation,
}

impl UniformLocs {
    pub fn new(gl: &WebGl, program: &Program) -> Result<UniformLocs, JsValue> {
        Ok(UniformLocs {
            model_view_projection: gl
                .get_uniform_location(program, "u_model_view_projection")
                .ok_or_else(|| "model_view_projection uniform doesn't exist")?,
            normal_matrix: gl
                .get_uniform_location(program, "u_normal_matrix")
                .ok_or_else(|| "normal_matrix uniform doesn't exist")?,
            sampler: gl
                .get_uniform_location(program, "u_sampler")
                .ok_or_else(|| "sampler uniform doesn't exist")?,
        })
    }
}

pub struct ProgramInfo {
    pub attrib_locs: AttribLocs,
    pub uniform_locs: UniformLocs,
}

impl ProgramInfo {
    pub fn new(gl: &WebGl, program: &Program) -> Result<ProgramInfo, JsValue> {
        Ok(ProgramInfo {
            attrib_locs: AttribLocs::new(gl, program)?,
            uniform_locs: UniformLocs::new(gl, program)?,
        })
    }
}
