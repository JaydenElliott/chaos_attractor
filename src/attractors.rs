use bevy::prelude::*;
pub struct LorenzAttractor {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

pub const LA_DEFAULT: LorenzAttractor = LorenzAttractor {
    a: 10.0,
    b: 28.0,
    c: 2.66
};

pub fn solve_lorenz(position: &Transform, t: f32, constants: LorenzAttractor) -> Vec3 {
    let pos = position.translation;
    let dx: f32 = pos.x + t * constants.a * (pos.y - pos.x);
    let dy: f32 = pos.y + t * (pos.x * (constants.b - pos.z) - pos.y);
    let dz: f32 = pos.z + t * (pos.x * pos.y - constants.c * pos.z);

    Vec3::new(dx, dy, dz,)
    
}
