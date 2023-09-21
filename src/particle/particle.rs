use b2::Vec2;
use ffi;

pub struct ParticleColor();
pub struct ParticleGroup();

#[repr(C)]
#[derive(Clone)]
pub struct ParticleDef {
    flags: u32,
    position: Vec2,
    velocity: Vec2,
    color: ParticleColor,
    lifetime: f32,
    user_data: ffi::Any,
    group: ParticleGroup
}

impl ParticleDef {
    pub fn new() -> Self {
        Self {
            flags: 0,
            position: Vec2::zero(),
            velocity: Vec2::zero(),
            color: ParticleColor(),
            lifetime: 0.0,
            user_data: std::ptr::null_mut(),
            group: ParticleGroup(),
        }
    }
}
