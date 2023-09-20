use common::math::Vec2;
use dynamics::joints::{Joint, JointDef, JointType};
use dynamics::world::{BodyHandle, World};
use user_data::UserDataTypes;
use wrap::*;

pub struct WheelJointDef {
    pub body_a: BodyHandle,
    pub body_b: BodyHandle,
    pub collide_connected: bool,
    pub local_anchor_a: Vec2,
    pub local_anchor_b: Vec2,
    pub local_axis_a: Vec2,
    pub enable_motor: bool,
    pub max_motor_torque: f32,
    pub motor_speed: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl WheelJointDef {
    pub fn new(body_a: BodyHandle, body_b: BodyHandle) -> WheelJointDef {
        WheelJointDef {
            body_a: body_a,
            body_b: body_b,
            collide_connected: false,
            local_anchor_a: Vec2 { x: 0., y: 0. },
            local_anchor_b: Vec2 { x: 0., y: 0. },
            local_axis_a: Vec2 { x: 1., y: 0. },
            enable_motor: false,
            max_motor_torque: 0.,
            motor_speed: 0.,
            stiffness: 2.,
            damping: 0.7,
        }
    }

    pub fn init<U: UserDataTypes>(
        &mut self,
        world: &World<U>,
        body_a: BodyHandle,
        body_b: BodyHandle,
        anchor: &Vec2,
        axis: &Vec2,
    ) {
        self.body_a = body_a;
        self.body_b = body_b;
        let a = world.body(body_a);
        let b = world.body(body_b);
        self.local_anchor_a = a.local_point(anchor);
        self.local_anchor_b = b.local_point(anchor);
        self.local_axis_a = a.local_vector(axis);
    }
}

impl JointDef for WheelJointDef {
    fn joint_type() -> JointType
    where
        Self: Sized,
    {
        JointType::Wheel
    }

    unsafe fn create<U: UserDataTypes>(&self, world: &mut World<U>) -> *mut ffi::Joint {
        ffi::World_create_wheel_joint(
            world.mut_ptr(),
            world.body_mut(self.body_a).mut_ptr(),
            world.body_mut(self.body_b).mut_ptr(),
            self.collide_connected,
            self.local_anchor_a,
            self.local_anchor_b,
            self.local_axis_a,
            self.enable_motor,
            self.max_motor_torque,
            self.motor_speed,
            self.stiffness,
            self.damping,
        )
    }
}

wrap_joint! {
    ffi::WheelJoint => WheelJoint (JointType::Wheel)
    < ffi::WheelJoint_as_joint
    > ffi::Joint_as_wheel_joint
}

impl WheelJoint {
    pub fn local_anchor_a<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::WheelJoint_get_local_anchor_a(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn local_anchor_b<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::WheelJoint_get_local_anchor_b(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn local_axis_a<'a>(&'a self) -> &'a Vec2 {
        unsafe {
            &*ffi::WheelJoint_get_local_axis_a(self.ptr()) // Comes from a C++ &
        }
    }

    pub fn joint_translation(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_joint_translation(self.ptr()) }
    }

    pub fn joint_linear_speed(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_joint_linear_speed(self.ptr()) }
    }

    pub fn joint_angular_speed(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_joint_angular_speed(self.ptr()) }
    }

    pub fn is_motor_enabled(&self) -> bool {
        unsafe { ffi::WheelJoint_is_motor_enabled(self.ptr()) }
    }

    pub fn motor_speed(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_motor_speed(self.ptr()) }
    }

    pub fn max_motor_torque(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_max_motor_torque(self.ptr()) }
    }

    pub fn motor_torque(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_motor_torque(self.ptr()) }
    }

    pub fn spring_stiffness(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_stiffness(self.ptr()) }
    }

    pub fn spring_damping(&self) -> f32 {
        unsafe { ffi::WheelJoint_get_damping(self.ptr()) }
    }

    pub fn enable_motor(&mut self, flag: bool) {
        unsafe { ffi::WheelJoint_enable_motor(self.mut_ptr(), flag) }
    }

    pub fn set_motor_speed(&mut self, speed: f32) {
        unsafe { ffi::WheelJoint_set_motor_speed(self.mut_ptr(), speed) }
    }

    pub fn set_max_motor_torque(&mut self, torque: f32) {
        unsafe { ffi::WheelJoint_set_max_motor_torque(self.mut_ptr(), torque) }
    }

    pub fn set_spring_stiffness(&mut self, stiffness: f32) {
        unsafe { ffi::WheelJoint_set_stiffness(self.mut_ptr(), stiffness) }
    }

    pub fn set_spring_damping(&mut self, damping: f32) {
        unsafe { ffi::WheelJoint_set_damping(self.mut_ptr(), damping) }
    }
}

#[doc(hidden)]
pub mod ffi {
    use common::math::Vec2;
    pub use dynamics::body::ffi::Body;
    pub use dynamics::joints::ffi::Joint;
    pub use dynamics::world::ffi::World;

    pub enum WheelJoint {}

    extern "C" {
        pub fn World_create_wheel_joint(
            world: *mut World,
            body_a: *mut Body,
            body_b: *mut Body,
            collide_connected: bool,
            local_anchor_a: Vec2,
            local_anchor_b: Vec2,
            local_axis_a: Vec2,
            enable_motor: bool,
            max_motor_torque: f32,
            motor_speed: f32,
            stiffness: f32,
            damping: f32,
        ) -> *mut Joint;
        pub fn WheelJoint_as_joint(slf: *mut WheelJoint) -> *mut Joint;
        pub fn Joint_as_wheel_joint(slf: *mut Joint) -> *mut WheelJoint;
        pub fn WheelJoint_get_local_anchor_a(slf: *const WheelJoint) -> *const Vec2;
        pub fn WheelJoint_get_local_anchor_b(slf: *const WheelJoint) -> *const Vec2;
        pub fn WheelJoint_get_local_axis_a(slf: *const WheelJoint) -> *const Vec2;
        pub fn WheelJoint_get_joint_translation(slf: *const WheelJoint) -> f32;
        pub fn WheelJoint_get_joint_linear_speed(slf: *const WheelJoint) -> f32;
        pub fn WheelJoint_get_joint_angular_speed(slf: *const WheelJoint) -> f32;
        pub fn WheelJoint_is_motor_enabled(slf: *const WheelJoint) -> bool;
        pub fn WheelJoint_enable_motor(slf: *mut WheelJoint, flag: bool);
        pub fn WheelJoint_set_motor_speed(slf: *mut WheelJoint, speed: f32);
        pub fn WheelJoint_get_motor_speed(slf: *const WheelJoint) -> f32;
        pub fn WheelJoint_set_max_motor_torque(slf: *mut WheelJoint, torque: f32);
        pub fn WheelJoint_get_max_motor_torque(slf: *const WheelJoint) -> f32;
        pub fn WheelJoint_get_motor_torque(slf: *const WheelJoint) -> f32;
        pub fn WheelJoint_set_stiffness(slf: *mut WheelJoint, stiffness: f32);
        pub fn WheelJoint_get_stiffness(slf: *const WheelJoint) -> f32;
        pub fn WheelJoint_set_damping(slf: *mut WheelJoint, damping: f32);
        pub fn WheelJoint_get_damping(slf: *const WheelJoint) -> f32;
    }
}
