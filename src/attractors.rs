use bevy::prelude::*;
pub struct AttractorConstants {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub t: f32,
}

////////////////////////////////////////////////////
// LORENZ ATTRACTOR
///////////////////////////////////////////////////

pub const LORENZ_DEFAULT: AttractorConstants = AttractorConstants {
    a: 10.0,
    b: 28.0,
    c: 2.66,
    t: 0.01,
};

pub fn solve_lorenz(position: &Transform, constants: AttractorConstants) -> Vec3 {
    let pos = position.translation;
    let dx: f32 = pos.x + (constants.t * constants.a * (pos.y - pos.x));
    let dy: f32 = pos.y + (constants.t * (pos.x * (constants.b - pos.z) - pos.y));
    let dz: f32 = pos.z + (constants.t * (pos.x * pos.y - constants.c * pos.z));

    Vec3::new(dx, dy, dz)
}

////////////////////////////////////////////////////
// ROSSLER ATTRACTOR (NOT WORKING)
///////////////////////////////////////////////////

#[allow(dead_code)]
pub const ROSSLER_DEFAULT: AttractorConstants = AttractorConstants {
    a: 0.2,
    b: 0.2,
    c: 5.7,
    t: 0.001,
};

#[allow(dead_code)]
pub fn solve_rossler(position: &Transform, constants: AttractorConstants) -> Vec3 {
    let pos = position.translation;
    let dx: f32 = pos.x + (constants.t * (-1.0 * (pos.y + pos.z)));
    let dy: f32 = pos.y + (constants.t * (pos.x + constants.a * pos.y));
    let dz: f32 = pos.z + (constants.b + (pos.x * pos.z) - (constants.c * pos.z));
    Vec3::new(dx, dy, dz)
}
