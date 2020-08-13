#![warn(clippy::all)]
use gltf::mesh::util::{ReadIndices, ReadTexCoords};
use log::{debug, Level};
use nalgebra as na;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext, WebGlUniformLocation};

mod gl_abstraction;
pub use gl_abstraction::{GlBuffer, Program, Shader, Texture2D, WebGl};

// TODO: either generate all of this, or use a macro to help a bit
struct AttribLocs {
    position: u32,
    normal: u32,
    tex_coord: u32,
}

impl AttribLocs {
    fn new(gl: &WebGl, program: &Program) -> Result<AttribLocs, JsValue> {
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

struct UniformLocs {
    model_view_projection: WebGlUniformLocation,
    normal_matrix: WebGlUniformLocation,
}

impl UniformLocs {
    fn new(gl: &WebGl, program: &Program) -> Result<UniformLocs, JsValue> {
        Ok(UniformLocs {
            model_view_projection: gl
                .get_uniform_location(program, "u_model_view_projection")
                .ok_or_else(|| "model_view_projection uniform doesn't exist")?,
            normal_matrix: gl
                .get_uniform_location(program, "u_normal_matrix")
                .ok_or_else(|| "normal_matrix uniform doesn't exist")?,
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
    normal_buffer: Option<GlBuffer<f32>>,
    tex_coord_buffer: Option<GlBuffer<f32>>,
    element_buffer: Option<GlBuffer<u16>>,
}

#[wasm_bindgen]
impl Tetra {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Tetra, JsValue> {
        console_error_panic_hook::set_once();
        console_log::init_with_level(Level::Debug).unwrap();
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
            normal_buffer: None,
            tex_coord_buffer: None,
            element_buffer: None,
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

    pub fn load_gltf(mut self, data: &[u8]) -> Result<Tetra, JsValue> {
        let (gltf, buffers, images) = gltf::import_slice(data).unwrap();
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    let mut vertices: Vec<f32> = Vec::new();
                    for vertex_position in iter {
                        vertices.extend_from_slice(&vertex_position);
                    }
                    self.vert_buffer = Some(GlBuffer::new(
                        &self.gl,
                        WebGlRenderingContext::ARRAY_BUFFER,
                        vertices,
                    )?);
                }
                if let Some(tex_coord_iter_enum) = reader.read_tex_coords(0) {
                    if let ReadTexCoords::F32(tex_coord_iter) = tex_coord_iter_enum {
                        let mut tex_coords: Vec<f32> = Vec::new();
                        for tex_coord in tex_coord_iter {
                            tex_coords.extend_from_slice(&tex_coord);
                        }
                        self.tex_coord_buffer = Some(GlBuffer::new(
                            &self.gl,
                            WebGlRenderingContext::ARRAY_BUFFER,
                            tex_coords,
                        )?);
                    }
                }
                if let Some(normals_iter) = reader.read_normals() {
                    let mut normals: Vec<f32> = Vec::new();
                    for normal in normals_iter {
                        normals.extend_from_slice(&normal);
                    }
                    self.normal_buffer = Some(GlBuffer::new(
                        &self.gl,
                        WebGlRenderingContext::ARRAY_BUFFER,
                        normals,
                    )?);
                }
                if let Some(indices_type) = reader.read_indices() {
                    let mut indicies_temp: Vec<u16> = Vec::new();
                    if let ReadIndices::U16(indices_buffer) = indices_type {
                        indicies_temp.extend(indices_buffer);
                    }
                    self.element_buffer = Some(GlBuffer::new(
                        &self.gl,
                        WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                        indicies_temp,
                    )?);
                }
            }
        }
        Ok(self)
    }

    pub fn draw(&mut self, timestamp: f32) {
        let program = self
            .program
            .as_ref()
            .expect("program has to have been created to draw");
        program.set_used();
        let program_info = self
            .program_info
            .as_ref()
            .expect("program info should've been created");

        let vert_buffer = self
            .vert_buffer
            .as_ref()
            .expect("vertex buffer must be created to draw");

        let normal_buffer = self
            .normal_buffer
            .as_ref()
            .expect("normal buffer must exist to calculate lighting");

        let tex_coord_buffer = self
            .tex_coord_buffer
            .as_ref()
            .expect("tex coord buffer must exist to render texture");

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl.enable(WebGlRenderingContext::DEPTH_TEST);
        self.gl.depth_func(WebGlRenderingContext::LEQUAL);

        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );

        vert_buffer.bind();
        self.gl.vertex_attrib_pointer_with_i32(
            program_info.attrib_locs.position,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl
            .enable_vertex_attrib_array(program_info.attrib_locs.position);

        normal_buffer.bind();
        self.gl.vertex_attrib_pointer_with_i32(
            program_info.attrib_locs.normal,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl
            .enable_vertex_attrib_array(program_info.attrib_locs.normal);
        
        tex_coord_buffer.bind();
        self.gl.vertex_attrib_pointer_with_i32(
            program_info.attrib_locs.tex_coord,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl
            .enable_vertex_attrib_array(program_info.attrib_locs.tex_coord);

        let model_rot = na::UnitQuaternion::from_scaled_axis(na::Vector3::y() * timestamp * 0.001);
        let model: na::Isometry3<f32> =
            na::Isometry3::from_parts(na::Translation3::identity(), model_rot);
        let view: na::Isometry3<f32> =
            na::Isometry3::new(na::Vector3::new(0.0, 0.0, -5.0), na::zero());
        let (width, height) = self.viewport_size;
        let projection: na::Perspective3<f32> = na::Perspective3::new(
            width as f32 / height as f32,
            std::f32::consts::FRAC_PI_4,
            0.1,
            100.0,
        );

        let matrix_view = (view * model).to_homogeneous();
        let mvp = projection.as_matrix() * matrix_view;
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&program_info.uniform_locs.model_view_projection),
            false,
            mvp.as_slice(),
        );

        let normal_matrix = matrix_view.clone().try_inverse().unwrap().transpose();
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&program_info.uniform_locs.normal_matrix),
            false,
            normal_matrix.as_slice(),
        );

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

        normal_buffer.unbind();
        vert_buffer.unbind();
        program.set_unused();
    }
}
