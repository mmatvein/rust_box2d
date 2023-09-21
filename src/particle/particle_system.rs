use wrap::*;

#[repr(C)]
#[derive(Clone)]
pub struct ParticleSystemDef {
    pub strict_contact_check: bool,
    pub density: f32,
    pub gravity_scale: f32,
    pub radius: f32,
    pub max_count: i32,
    pub pressure_strength: f32,
    pub damping_strength: f32,
    pub elastic_strength: f32,
    pub spring_strength: f32,
    pub viscous_strength: f32,
    pub surface_tension_pressure_strength: f32,
    pub surface_tension_normal_strength: f32,
    pub repulsive_strength: f32,
    pub powder_strength: f32,
    pub ejection_strength: f32,
    pub static_pressure_strength: f32,
    pub static_pressure_relaxation: f32,
    pub static_pressure_iterations: i32,
    pub color_mixing_strength: f32,
    pub destroy_by_age: bool,
    pub lifetime_granularity: f32
}

impl ParticleSystemDef {
    pub fn new() -> ParticleSystemDef {
        ParticleSystemDef {
            strict_contact_check: false,
            density: 1.0,
            gravity_scale: 1.0,
            radius: 1.0,
            max_count: 0,
            pressure_strength: 0.05,
            damping_strength: 1.0,
            elastic_strength: 0.25,
            spring_strength: 0.25,
            viscous_strength: 0.25,
            surface_tension_pressure_strength: 0.2,
            surface_tension_normal_strength: 0.2,
            repulsive_strength: 1.0,
            powder_strength: 0.5,
            ejection_strength: 0.5,
            static_pressure_strength: 0.2,
            static_pressure_relaxation: 0.2,
            static_pressure_iterations: 8,
            color_mixing_strength: 0.5,
            destroy_by_age: true,
            lifetime_granularity: 1.0 / 60.0,
        }
    }
}

wrap! { ffi::ParticleSystem => pub ParticleSystem }

#[doc(hidden)]
pub mod ffi {
    pub enum ParticleSystem {}

    extern "C" {
        pub fn ParticleSystem_create_particle() -> u32;
    }
}
