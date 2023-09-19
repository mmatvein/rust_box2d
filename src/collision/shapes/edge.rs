use std::mem;

use super::Shape;
use common::math::Vec2;
use wrap::*;

wrap_shape! {
    ffi::EdgeShape => EdgeShape
    < ffi::EdgeShape_as_shape
    > ffi::Shape_as_edge_shape
}

impl EdgeShape {
    pub fn new() -> Self {
        unsafe { EdgeShape::from_ffi(ffi::EdgeShape_new()) }
    }

    pub fn new_with(v1: &Vec2, v2: &Vec2) -> Self {
        let mut s = Self::new();
        s.set_two_sided(v1, v2);
        s
    }

    pub fn set_one_sided(&mut self, v0: &Vec2, v1: &Vec2, v2: &Vec2, v3: &Vec2) {
        unsafe { ffi::EdgeShape_set_one_sided(self.mut_ptr(), v0, v1, v2, v3) }
    }

    pub fn set_two_sided(&mut self, v1: &Vec2, v2: &Vec2) {
        unsafe { ffi::EdgeShape_set_two_sided(self.mut_ptr(), v1, v2) }
    }

    pub fn v1(&self) -> Vec2 {
        unsafe { ffi::EdgeShape_get_v1(self.ptr()) }
    }

    pub fn v2(&self) -> Vec2 {
        unsafe { ffi::EdgeShape_get_v2(self.ptr()) }
    }

    pub fn v0(&self) -> Option<Vec2> {
        unsafe {
            let mut v0 = mem::MaybeUninit::uninit();
            if ffi::EdgeShape_get_v0(self.ptr(), &mut *v0.as_mut_ptr()) {
                Some(v0.assume_init())
            } else {
                None
            }
        }
    }

    pub fn v3(&self) -> Option<Vec2> {
        unsafe {
            let mut v3 = mem::MaybeUninit::uninit();
            if ffi::EdgeShape_get_v3(self.ptr(), &mut *v3.as_mut_ptr()) {
                Some(v3.assume_init())
            } else {
                None
            }
        }
    }
}

impl Drop for EdgeShape {
    fn drop(&mut self) {
        unsafe { ffi::EdgeShape_drop(self.mut_ptr()) }
    }
}

#[doc(hidden)]
pub mod ffi {
    pub use collision::shapes::ffi::Shape;
    use common::math::Vec2;

    pub enum EdgeShape {}

    extern "C" {
        pub fn EdgeShape_new() -> *mut EdgeShape;
        pub fn EdgeShape_drop(slf: *mut EdgeShape);
        pub fn EdgeShape_as_shape(slf: *mut EdgeShape) -> *mut Shape;
        pub fn Shape_as_edge_shape(slf: *mut Shape) -> *mut EdgeShape;
        pub fn EdgeShape_set_one_sided(
            slf: *mut EdgeShape,
            v0: *const Vec2,
            v1: *const Vec2,
            v2: *const Vec2,
            v3: *const Vec2,
        );
        pub fn EdgeShape_set_two_sided(slf: *mut EdgeShape, v1: *const Vec2, v2: *const Vec2);
        pub fn EdgeShape_get_v1(slf: *const EdgeShape) -> Vec2;
        pub fn EdgeShape_get_v2(slf: *const EdgeShape) -> Vec2;

        pub fn EdgeShape_get_v0(slf: *const EdgeShape, v0: &mut Vec2) -> bool;
        pub fn EdgeShape_get_v3(slf: *const EdgeShape, v3: &mut Vec2) -> bool;
    }
}
