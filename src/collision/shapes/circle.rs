use super::Shape;
use common::math::Vec2;
use wrap::*;

wrap_shape! {
    ffi::CircleShape => CircleShape
    < ffi::CircleShape_as_shape
    > ffi::Shape_as_circle_shape
}

impl CircleShape {
    pub fn new() -> Self {
        unsafe { CircleShape::from_ffi(ffi::CircleShape_new()) }
    }

    pub fn new_with(position: Vec2, radius: f32) -> Self {
        let mut circle = Self::new();
        circle.set_position(position);
        circle.set_radius(radius);
        circle
    }

    pub fn radius(&self) -> f32 {
        unsafe { ffi::Shape_get_radius(self.base_ptr()) }
    }

    pub fn set_radius(&mut self, radius: f32) {
        unsafe { ffi::Shape_set_radius(self.mut_base_ptr(), radius) }
    }

    pub fn position(&self) -> Vec2 {
        unsafe { ffi::CircleShape_get_pos(self.ptr()) }
    }

    pub fn set_position(&mut self, pos: Vec2) {
        unsafe { ffi::CircleShape_set_pos(self.mut_ptr(), pos) }
    }
}

impl Drop for CircleShape {
    fn drop(&mut self) {
        unsafe { ffi::CircleShape_drop(self.mut_ptr()) }
    }
}

#[doc(hidden)]
pub mod ffi {
    pub use collision::shapes::ffi::Shape;
    pub use collision::shapes::ffi::{Shape_get_radius, Shape_set_radius};
    use common::math::Vec2;

    pub enum CircleShape {}

    extern "C" {
        pub fn CircleShape_new() -> *mut CircleShape;
        pub fn CircleShape_drop(slf: *mut CircleShape);
        pub fn CircleShape_as_shape(slf: *mut CircleShape) -> *mut Shape;
        pub fn Shape_as_circle_shape(slf: *mut Shape) -> *mut CircleShape;
        pub fn CircleShape_get_pos(slf: *const CircleShape) -> Vec2;
        pub fn CircleShape_set_pos(slf: *mut CircleShape, pos: Vec2);
    }
}
