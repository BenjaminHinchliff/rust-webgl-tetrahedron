use std::mem;
use wasm_bindgen::prelude::*;
use web_sys::{
    WebGlBuffer, WebGlRenderingContext,
};

use crate::WebGl;

pub struct GlBuffer<T> {
    gl: WebGl,
    type_: u32,
    array: Vec<T>,
    buffer: WebGlBuffer,
}

impl<T> GlBuffer<T> {
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

    pub fn bind(&self) {
        self.gl.bind_buffer(self.type_, Some(&self.buffer));
    }

    pub fn unbind(&self) {
        self.gl.bind_buffer(self.type_, None);
    }

    pub fn array(&self) -> &Vec<T> {
        &self.array
    }
}

impl<T> Drop for GlBuffer<T> {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(&self.buffer));
    }
}