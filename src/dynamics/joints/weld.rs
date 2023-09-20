use common::math::Vec2;
use dynamics::joints::{Joint, JointDef, JointType};
use dynamics::world::{BodyHandle, World};
use user_data::UserDataTypes;
use wrap::*;

pub struct WeldJointDef {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub collide_connected: bool,
    pub local_anchor_a: Vec2,
    pub local_anchor_b: Vec2,
    pub reference_angle: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl WeldJointDef {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle) -> WeldJointDef {
        WeldJointDef {
            body_a: body_a,
            body_b: body_b,
            collide_connected: false,
            local_anchor_a: Vec2 { x: 0., y: 0. },
            local_anchor_b: Vec2 { x: 0., y: 0. },
            reference_angle: 0.,
            stiffness: 0.,
            damping: 0.,
        }
    }

    pub fn init<U: UserDataTypes>(
        &mut self,
        world: &World<U>,
        body_a: BodyHandle,
        body_b: BodyHandle,
        anchor: &Vec2,
    ) {
        self.body_a = body_a;
        self.body_b = body_b;
        let a = world.body(body_a);
        let b = world.body(body_b);
        self.local_anchor_a = a.local_point(anchor);
        self.local_anchor_b = b.local_point(anchor);
        self.reference_angle = b.angle() - a.angle();
    }
}

impl JointDef for WeldJointDef {
    fn joint_type() -> JointType
    where
        Self: Sized,
    {
        JointType::Weld
    }

    unsafe fn create<U: UserDataTypes>(&self, world: &mut World<U>) -> *mut ffi::Joint {
        ffi::World_create_weld_joint(
            world.mut_ptr(),
            world.body_mut(self.body_a).mut_ptr(),
            world.body_mut(self.body_b).mut_ptr(),
            self.collide_connected,
            self.local_anchor_a,
            self.local_anchor_b,
            self.reference_angle,
            self.stiffness,
            self.damping,
        )
    }
}

wrap_joint! {
    ffi::WeldJoint => WeldJoint (JointType::Weld)
    < ffi::WeldJoint_as_joint
    > ffi::Joint_as_weld_joint
}

impl WeldJoint {
    pub fn local_anchor_a<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::WeldJoint_get_local_anchor_a(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn local_anchor_b<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::WeldJoint_get_local_anchor_b(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn reference_angle(&self) -> f32 {
        unsafe { ffi::WeldJoint_get_reference_angle(self.ptr()) }
    }

    pub fn stiffnes(&self) -> f32 {
        unsafe { ffi::WeldJoint_get_stiffness(self.ptr()) }
    }

    pub fn damping(&self) -> f32 {
        unsafe { ffi::WeldJoint_get_damping(self.ptr()) }
    }

    pub fn set_stiffness(&mut self, stiffness: f32) {
        unsafe { ffi::WeldJoint_set_stiffness(self.mut_ptr(), stiffness) }
    }

    pub fn set_damping(&mut self, damping: f32) {
        unsafe { ffi::WeldJoint_set_damping(self.mut_ptr(), damping) }
    }
}

#[doc(hidden)]
pub mod ffi {
    use common::math::Vec2;
    pub use dynamics::body::ffi::Body;
    pub use dynamics::joints::ffi::Joint;
    pub use dynamics::world::ffi::World;

    pub enum WeldJoint {}

    extern "C" {
        pub fn World_create_weld_joint(
            world: *mut World,
            body_a: *mut Body,
            body_b: *mut Body,
            collide_connected: bool,
            local_anchor_a: Vec2,
            local_anchor_b: Vec2,
            reference_angle: f32,
            stiffness: f32,
            damping: f32,
        ) -> *mut Joint;
        pub fn WeldJoint_as_joint(slf: *mut WeldJoint) -> *mut Joint;
        pub fn Joint_as_weld_joint(slf: *mut Joint) -> *mut WeldJoint;
        pub fn WeldJoint_get_local_anchor_a(slf: *const WeldJoint) -> *const Vec2;
        pub fn WeldJoint_get_local_anchor_b(slf: *const WeldJoint) -> *const Vec2;
        pub fn WeldJoint_get_reference_angle(slf: *const WeldJoint) -> f32;
        pub fn WeldJoint_set_stiffness(slf: *mut WeldJoint, stiffness: f32);
        pub fn WeldJoint_get_stiffness(slf: *const WeldJoint) -> f32;
        pub fn WeldJoint_set_damping(slf: *mut WeldJoint, damping: f32);
        pub fn WeldJoint_get_damping(slf: *const WeldJoint) -> f32;
    }
}
