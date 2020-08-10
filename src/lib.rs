use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext, WebGlShader, WebGlProgram};
use web_sys::console;

macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format_args!($($t)*).to_string().into()))
}

#[wasm_bindgen]
pub struct Tetra {
    gl: WebGlRenderingContext,
    shaders: Vec<WebGlShader>,
    program: Option<WebGlProgram>,
}

#[wasm_bindgen]
impl Tetra {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Tetra, JsValue> {
        Ok(Tetra {
            gl: canvas.get_context("webgl")
            .expect("invalid web context")
            .expect("unable to get webgl context from #webgl")
            .dyn_into::<WebGlRenderingContext>()?,
            shaders: Vec::new(),
            program: None,
        })
    }

    pub fn add_shader(mut self, shader_type: u32, source: &str) -> Result<Tetra, JsValue> {
        self.shaders.push(compile_shader(&self.gl, shader_type, source)?);
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

    pub fn draw(&mut self) {
        self.gl.use_program(Some(self.program.as_ref().unwrap()));

        let vertices: [f32; 9] = [
            -0.7, -0.7, 0.0, // bottom left
            0.7, -0.7, 0.0, // bottom right
            0.0, 0.7, 0.0, // top
        ];

        let buffer = self.gl.create_buffer().ok_or("failed to create buffer").unwrap();
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
    
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
    
        self.gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(0);
    
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
        self.gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() / 3) as i32,
        );
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
