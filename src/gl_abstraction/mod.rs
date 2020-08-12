use std::rc::Rc;

use web_sys::WebGlRenderingContext;

/// a type alias for a reference counted webgl rendering context that can be passed into other
/// structs and functions
pub type WebGl = Rc<WebGlRenderingContext>;

mod buffer;
pub use buffer::GlBuffer;

mod shader;
pub use shader::Shader;

mod program;
pub use program::Program;
