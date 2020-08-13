#![warn(clippy::all)]
use nalgebra as na;
use std::mem;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlImageElement, WebGlRenderingContext, WebGlUniformLocation};

mod gl_abstraction;
pub use gl_abstraction::{GlBuffer, Program, Shader, Texture2D, WebGl};

// TODO: either generate all of this, or use a macro to help a bit
struct AttribLocs {
    position: u32,
    tex_coord: u32,
}

impl AttribLocs {
    fn new(gl: &WebGl, program: &Program) -> Result<AttribLocs, JsValue> {
        let position = gl.get_attrib_location(program, "a_position");
        if position == -1 {
            return Err("position attribute doesn't exist".into());
        }
        let tex_coord = gl.get_attrib_location(program, "a_tex_coord");
        if tex_coord == -1 {
            return Err("tex_coord attribute doesn't exist".into());
        }
        Ok(AttribLocs {
            position: position as u32,
            tex_coord: tex_coord as u32,
        })
    }
}

struct UniformLocs {
    model_view_projection: WebGlUniformLocation,
    sampler: WebGlUniformLocation,
}

impl UniformLocs {
    fn new(gl: &WebGl, program: &Program) -> Result<UniformLocs, JsValue> {
        Ok(UniformLocs {
            model_view_projection: gl
                .get_uniform_location(program, "u_model_view_projection")
                .ok_or_else(|| "model_view_projection uniform doesn't exist")?,
            sampler: gl
                .get_uniform_location(program, "u_sampler")
                .ok_or_else(|| "model_view_projection uniform doesn't exist")?,
        })
    }
}

struct ProgramInfo {
    attrib_locs: AttribLocs,
    uniform_locs: UniformLocs,
}

impl ProgramInfo {
    fn new(gl: &WebGl, program: &Program) -> Result<ProgramInfo, JsValue> {
        Ok(ProgramInfo {
            attrib_locs: AttribLocs::new(gl, program)?,
            uniform_locs: UniformLocs::new(gl, program)?,
        })
    }
}

#[wasm_bindgen]
pub struct Tetra {
    gl: WebGl,
    viewport_size: (u32, u32),
    shaders: Vec<Shader>,
    program: Option<Program>,
    program_info: Option<ProgramInfo>,
    vert_buffer: Option<GlBuffer<f32>>,
    element_buffer: Option<GlBuffer<u16>>,
    texture: Option<Texture2D>,
}

#[wasm_bindgen]
impl Tetra {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Tetra, JsValue> {
        console_error_panic_hook::set_once();
        let gl = canvas
            .get_context("webgl")
            .expect("invalid web context")
            .expect("unable to get webgl context from #webgl")
            .dyn_into::<WebGlRenderingContext>()?;
        let (width, height) = (canvas.width(), canvas.height());
        gl.viewport(0, 0, width as i32, height as i32);
        Ok(Tetra {
            gl: Rc::new(gl),
            viewport_size: (width, height),
            shaders: Vec::new(),
            program: None,
            program_info: None,
            vert_buffer: None,
            element_buffer: None,
            texture: None,
        })
    }

    pub fn add_shader(mut self, shader_type: u32, source: &str) -> Result<Tetra, JsValue> {
        self.shaders
            .push(Shader::new(&self.gl, shader_type, source)?);
        Ok(self)
    }

    pub fn add_vert_shader(self, source: &str) -> Result<Tetra, JsValue> {
        self.add_shader(WebGlRenderingContext::VERTEX_SHADER, source)
    }

    pub fn add_frag_shader(self, source: &str) -> Result<Tetra, JsValue> {
        self.add_shader(WebGlRenderingContext::FRAGMENT_SHADER, source)
    }

    pub fn link_program(mut self) -> Result<Tetra, JsValue> {
        let program = Program::new(&self.gl, &self.shaders)?;
        self.shaders.clear();
        self.program_info = Some(ProgramInfo::new(&self.gl, &program)?);
        self.program = Some(program);
        Ok(self)
    }

    pub fn add_vertices(mut self, verts: Vec<f32>) -> Result<Tetra, JsValue> {
        self.vert_buffer = Some(GlBuffer::new(
            &self.gl,
            WebGlRenderingContext::ARRAY_BUFFER,
            verts,
        )?);
        Ok(self)
    }

    pub fn add_indices(mut self, indices: Vec<u16>) -> Result<Tetra, JsValue> {
        self.element_buffer = Some(GlBuffer::new(
            &self.gl,
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            indices,
        )?);
        Ok(self)
    }

    pub fn set_texture(mut self, image: HtmlImageElement) -> Result<Tetra, JsValue> {
        self.texture = Some(Texture2D::new(&self.gl, &image)?);
        Ok(self)
    }

    pub fn draw(&mut self) {
        let program = self
            .program
            .as_ref()
            .expect("program has to have been created to draw");
        program.set_used();
        let program_info = self
            .program_info
            .as_ref()
            .expect("program info should've been created");

        let texture = self.texture.as_ref().expect("texture must have been set");
        self.gl.active_texture(WebGlRenderingContext::TEXTURE0);
        texture.bind();
        self.gl
            .uniform1i(Some(&program_info.uniform_locs.sampler), 0);

        let vert_buffer = self
            .vert_buffer
            .as_ref()
            .expect("vertex buffer must be created to draw");

        let model_rot =
            na::UnitQuaternion::from_scaled_axis(na::Vector3::x() * -std::f32::consts::FRAC_PI_4);
        let model: na::Isometry3<f32> =
            na::Isometry3::from_parts(na::Translation3::identity(), model_rot);
        let view: na::Isometry3<f32> =
            na::Isometry3::new(na::Vector3::new(0.0, 0.0, -4.0), na::zero());
        let (width, height) = self.viewport_size;
        let projection: na::Perspective3<f32> = na::Perspective3::new(
            width as f32 / height as f32,
            std::f32::consts::FRAC_PI_4,
            0.1,
            100.0,
        );

        let mvp = projection.as_matrix() * (view * model).to_homogeneous();
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&program_info.uniform_locs.model_view_projection),
            false,
            mvp.as_slice(),
        );

        vert_buffer.bind();

        self.gl.vertex_attrib_pointer_with_i32(
            program_info.attrib_locs.position,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            5 * mem::size_of::<f32>() as i32,
            0,
        );
        self.gl.enable_vertex_attrib_array(0);

        self.gl.vertex_attrib_pointer_with_i32(
            program_info.attrib_locs.tex_coord,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            5 * mem::size_of::<f32>() as i32,
            3 * mem::size_of::<f32>() as i32,
        );
        self.gl.enable_vertex_attrib_array(1);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        if let Some(ref element_buffer) = self.element_buffer {
            element_buffer.bind();
            self.gl.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                element_buffer.array().len() as i32,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
            element_buffer.unbind();
        } else {
            self.gl.draw_arrays(
                WebGlRenderingContext::TRIANGLES,
                0,
                (vert_buffer.array().len() / 3) as i32,
            );
        }

        vert_buffer.unbind();
        texture.unbind();
        program.set_unused();
    }
}
