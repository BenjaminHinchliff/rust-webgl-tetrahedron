use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader};

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

#[wasm_bindgen]
pub struct Tetra {
    gl: WebGlRenderingContext,
    shaders: Vec<WebGlShader>,
    program: Option<WebGlProgram>,
    vert_array: Option<Vec<f32>>,
    vert_buffer: Option<WebGlBuffer>,
    element_array: Option<Vec<u16>>,
    element_buffer: Option<WebGlBuffer>,
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
        Ok(Tetra {
            gl,
            shaders: Vec::new(),
            program: None,
            vert_array: None,
            vert_buffer: None,
            element_array: None,
            element_buffer: None,
        })
    }

    pub fn add_shader(mut self, shader_type: u32, source: &str) -> Result<Tetra, JsValue> {
        self.shaders
            .push(compile_shader(&self.gl, shader_type, source)?);
        Ok(self)
    }

    pub fn add_vert_shader(self, source: &str) -> Result<Tetra, JsValue> {
        self.add_shader(WebGlRenderingContext::VERTEX_SHADER, source)
    }

    pub fn add_frag_shader(self, source: &str) -> Result<Tetra, JsValue> {
        self.add_shader(WebGlRenderingContext::FRAGMENT_SHADER, source)
    }

    pub fn link_program(mut self) -> Result<Tetra, JsValue> {
        self.program = Some(link_program(&self.gl, &self.shaders)?);
        Ok(self)
    }

    pub fn add_vertices(mut self, verts: Vec<f32>) -> Result<Tetra, JsValue> {
        self.vert_array = Some(verts);

        let buffer = self.gl.create_buffer().ok_or("failed to create buffer")?;
        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(self.vert_array.as_ref().unwrap());

            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
        self.vert_buffer = Some(buffer);
        Ok(self)
    }

    pub fn add_indices(mut self, indices: Vec<u16>) -> Result<Tetra, JsValue> {
        self.element_array = Some(indices);
        let buffer = self
            .gl
            .create_buffer()
            .ok_or("failed to create index buffer")?;
        self.gl
            .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let element_array = js_sys::Uint16Array::view(self.element_array.as_ref().unwrap());

            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &element_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.gl
            .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
        self.element_buffer = Some(buffer);
        Ok(self)
    }

    pub fn draw(&mut self) {
        self.gl.use_program(Some(self.program.as_ref().expect("program has to have been created to draw")));
        self.vert_buffer.as_ref().expect("vertex buffer must be created to draw");
        self.vert_array.as_ref().expect("vertex array must be created to draw");

        self.gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            self.vert_buffer.as_ref(),
        );

        self.gl
            .vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(0);

        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        if let (Some(element_buffer), Some(element_array)) =
            (self.element_buffer.as_ref(), self.element_array.as_ref())
        {
            self.gl.bind_buffer(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(element_buffer),
            );
            self.gl.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                element_array.len() as i32,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
            self.gl
                .bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);
        } else if let Some(ref vert_array) = self.vert_array {
            self.gl.draw_arrays(
                WebGlRenderingContext::TRIANGLES,
                0,
                (vert_array.len() / 3) as i32,
            );
        }

        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
        self.gl.use_program(None);
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

fn link_program(
    gl: &WebGlRenderingContext,
    shaders: &[WebGlShader],
) -> Result<WebGlProgram, String> {
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
