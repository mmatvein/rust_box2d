use wrap::*;
use dynamics::world::{ World, JointHandle };
use dynamics::joints::{ Joint, JointType, JointDef };

pub struct GearJointDef {
    pub collide_connected: bool,
    pub joint_a: JointHandle,
    pub joint_b: JointHandle,
    pub ratio: f32
}

impl GearJointDef {
    pub fn new(joint_a: JointHandle, joint_b: JointHandle) -> GearJointDef {
        GearJointDef {
            collide_connected: false,
            joint_a: joint_a,
            joint_b: joint_b,
            ratio: 1.
        }
    }
}

impl JointDef for GearJointDef {
    fn joint_type() -> JointType where Self: Sized { JointType::Gear }

    unsafe fn create(&self, world: &mut World) -> *mut ffi::Joint {
        ffi::World_create_gear_joint(
            world.mut_ptr(),
            self.collide_connected,
            world.get_joint_mut(self.joint_a).mut_base_ptr(),
            world.get_joint_mut(self.joint_b).mut_base_ptr(),
            self.ratio
        )
    }
}

wrap_joint! {
    ffi::GearJoint => GearJoint (JointType::Gear)
    < ffi::GearJoint_as_joint
    > ffi::Joint_as_gear_joint
}

impl GearJoint {
    pub fn ratio(&self) -> f32 {
        unsafe {
            ffi::GearJoint_get_ratio(self.ptr())
        }
    }

    pub fn joint_1(&self) -> usize {
        unsafe {
            ffi::GearJoint_get_joint_1(self.ptr() as *mut ffi::GearJoint) as usize
        }
    }

    pub fn joint_2(&self) -> usize {
        unsafe {
            ffi::GearJoint_get_joint_2(self.ptr() as *mut ffi::GearJoint) as usize
        }
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        unsafe {
            ffi::GearJoint_set_ratio(self.mut_ptr(), ratio)
        }
    }
}

#[doc(hidden)]
pub mod ffi {
    pub use dynamics::world::ffi::World;
    pub use dynamics::joints::ffi::Joint;

    #[repr(C)] pub struct GearJoint;

    extern {
        pub fn World_create_gear_joint(
            world: *mut World,
            collide_connected: bool,
            joint_a: *mut Joint,
            joint_b: *mut Joint,
            ratio: f32
        ) -> *mut Joint;
        pub fn GearJoint_as_joint(slf: *mut GearJoint) -> *mut Joint;
        pub fn Joint_as_gear_joint(slf: *mut Joint) -> *mut GearJoint;
        pub fn GearJoint_get_joint_1(slf: *mut GearJoint) -> *mut Joint;
        pub fn GearJoint_get_joint_2(slf: *mut GearJoint) -> *mut Joint;
        pub fn GearJoint_set_ratio(slf: *mut GearJoint, ratio: f32);
        pub fn GearJoint_get_ratio(slf: *const GearJoint) -> f32;
    }
}
