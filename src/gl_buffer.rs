use std::mem;
use wasm_bindgen::prelude::*;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

use crate::WebGl;

/// a generic type for an opengl buffer that contains both the buffer itself
/// and also owns the array to the underlying data. Currently the array is
/// immutable so the buffer isn't invalidated, but this could be changed with
/// an appropriate setter
pub struct GlBuffer<T> {
    gl: WebGl,
    type_: u32,
    array: Vec<T>,
    buffer: WebGlBuffer,
}

impl<T> GlBuffer<T> {
    /// create a new buffer with usage type of STATIC_DRAW
    /// # Arguments
    /// * `gl` - a reference counted pointer to the webgl context that should be linked to
    /// * `type_`  - an enum that denotes the type of the buffer e.g. ARRAY_BUFFER or ELEMENT_ARRAY_BUFFER
    /// * `array` - a vector to the data to be stored in the buffer
    pub fn new(gl: &WebGl, type_: u32, array: Vec<T>) -> Result<GlBuffer<T>, JsValue> {
        let buffer = gl
            .create_buffer()
            .ok_or_else(|| "failed to create buffer")?;
        gl.bind_buffer(type_, Some(&buffer));

        let buffer_array = unsafe {
            std::slice::from_raw_parts(
                array.as_ptr() as *const u8,
                array.len() * mem::size_of::<T>(),
            )
        };
        gl.buffer_data_with_u8_array(type_, buffer_array, WebGlRenderingContext::STATIC_DRAW);

        gl.bind_buffer(type_, None);
        Ok(GlBuffer {
            gl: gl.clone(),
            type_,
            array,
            buffer,
        })
    }

    /// bind a buffer to the stored WebGl instance
    pub fn bind(&self) {
        self.gl.bind_buffer(self.type_, Some(&self.buffer));
    }

    /// unbind a buffer to the stored WebGl instance
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.type_, None);
    }

    /// get an immutable reference to the underlying array
    pub fn array(&self) -> &Vec<T> {
        &self.array
    }
}

impl<T> Drop for GlBuffer<T> {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(&self.buffer));
    }
}
