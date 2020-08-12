#![warn(clippy::all)]
use nalgebra as na;
use std::mem;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, HtmlImageElement, WebGlRenderingContext, WebGlTexture, WebGlUniformLocation,
};

mod gl_abstraction;
pub use gl_abstraction::{GlBuffer, Program, Shader, WebGl};

#[wasm_bindgen]
pub struct Tetra {
    gl: WebGl,
    viewport_size: (u32, u32),
    shaders: Vec<Shader>,
    program: Option<Program>,
    vert_buffer: Option<GlBuffer<f32>>,
    element_buffer: Option<GlBuffer<u16>>,
    mvp_loc: Option<WebGlUniformLocation>,
    texture: Option<WebGlTexture>,
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
            vert_buffer: None,
            element_buffer: None,
            mvp_loc: None,
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
        self.mvp_loc = self
            .gl
            .get_uniform_location(&program, "model_view_projection");
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

    pub fn set_texture(mut self, data: HtmlImageElement) -> Result<Tetra, JsValue> {
        let texture = self
            .gl
            .create_texture()
            .ok_or_else(|| "unable to create texture")?;
        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

        self.gl.tex_image_2d_with_u32_and_u32_and_image(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            &data,
        )?;

        self.gl.generate_mipmap(WebGlRenderingContext::TEXTURE_2D);

        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

        self.texture = Some(texture);
        Ok(self)
    }

    pub fn draw(&mut self) {
        let program = self
            .program
            .as_ref()
            .expect("program has to have been created to draw");
        program.set_used();

        self.gl.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(
                &self
                    .texture
                    .as_ref()
                    .expect("texture must have been set to draw"),
            ),
        );
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

        let mvp_loc = self
            .mvp_loc
            .as_ref()
            .expect("model view projection matrix must be defined in vertex shader");
        let mvp = projection.as_matrix() * (view * model).to_homogeneous();
        self.gl
            .uniform_matrix4fv_with_f32_array(Some(mvp_loc), false, mvp.as_slice());

        vert_buffer.bind();

        self.gl.vertex_attrib_pointer_with_i32(
            0,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            5 * mem::size_of::<f32>() as i32,
            0,
        );
        self.gl.enable_vertex_attrib_array(0);

        self.gl.vertex_attrib_pointer_with_i32(
            1,
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
        program.set_unused();
    }
}

impl Drop for Tetra {
    fn drop(&mut self) {
        self.gl.delete_texture(self.texture.as_ref());
    }
}
