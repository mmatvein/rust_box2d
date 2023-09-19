use std::slice;

use common::math::Vec2;
use wrap::*;

use super::{EdgeShape, Shape};

wrap_shape! {
    ffi::ChainShape => ChainShape
    < ffi::ChainShape_as_shape
    > ffi::Shape_as_chain_shape
}

impl ChainShape {
    pub fn new() -> Self {
        unsafe { ChainShape::from_ffi(ffi::ChainShape_new()) }
    }

    pub fn new_loop(vertices: &[Vec2]) -> Self {
        let mut s = Self::new();
        s.create_loop(vertices);
        s
    }

    pub fn new_chain(vertices: &[Vec2], prev_vertex: &Vec2, next_vertex: &Vec2) -> Self {
        let mut s = Self::new();
        s.create_chain(vertices, prev_vertex, next_vertex);
        s
    }

    pub fn clear(&mut self) {
        unsafe { ffi::ChainShape_clear(self.mut_ptr()) }
    }

    pub fn create_loop(&mut self, vertices: &[Vec2]) {
        unsafe {
            ffi::ChainShape_create_loop(self.mut_ptr(), vertices.as_ptr(), vertices.len() as i32)
        }
    }

    pub fn create_chain(&mut self, vertices: &[Vec2], prev_vertex: &Vec2, next_vertex: &Vec2) {
        unsafe {
            let prev_vertex_ptr = prev_vertex as *const _;
            let next_vertex_ptr = next_vertex as *const _;
            ffi::ChainShape_create_chain(self.mut_ptr(), vertices.as_ptr(), vertices.len() as i32, prev_vertex_ptr, next_vertex_ptr);
        }
    }

    pub fn vertices(&self) -> &[Vec2] {
        unsafe {
            let vertices = ffi::ChainShape_get_vertices_const(self.ptr());
            let count = ffi::ChainShape_get_vertex_count(self.ptr());
            slice::from_raw_parts(vertices, count as usize)
        }
    }

    pub fn child_edge(&self, index: i32) -> EdgeShape {
        unsafe {
            let mut edge = EdgeShape::new();
            ffi::ChainShape_get_child_edge(self.ptr(), edge.mut_ptr(), index);
            edge
        }
    }
}

impl Drop for ChainShape {
    fn drop(&mut self) {
        unsafe { ffi::ChainShape_drop(self.mut_ptr()) }
    }
}

#[doc(hidden)]
pub mod ffi {
    pub use collision::shapes::edge::ffi::EdgeShape;
    pub use collision::shapes::ffi::Shape;
    use common::math::Vec2;

    pub enum ChainShape {}

    extern "C" {
        pub fn ChainShape_new() -> *mut ChainShape;
        pub fn ChainShape_drop(slf: *mut ChainShape);
        pub fn ChainShape_as_shape(slf: *mut ChainShape) -> *mut Shape;
        pub fn Shape_as_chain_shape(slf: *mut Shape) -> *mut ChainShape;
        pub fn ChainShape_clear(slf: *mut ChainShape);
        pub fn ChainShape_create_loop(slf: *mut ChainShape, vertices: *const Vec2, count: i32);
        pub fn ChainShape_create_chain(slf: *mut ChainShape, vertices: *const Vec2, count: i32, prev: *const Vec2, next: *const Vec2);
        pub fn ChainShape_get_vertices_const(slf: *const ChainShape) -> *const Vec2;
        pub fn ChainShape_get_vertex_count(slf: *const ChainShape) -> i32;
        pub fn ChainShape_get_child_edge(slf: *const ChainShape,
                                         edge: *mut EdgeShape,
                                         index: i32);
    }
}
