use bevy::prelude::*;
use crate::{WINDOW_SIZE, attractors};
use rand::prelude::random;

////////////////////////////////////////////////////
// PARTICLE CONSTANTS
///////////////////////////////////////////////////
const PARTICLE_SIZE: f32 = 1.0;

const N_PARTCLES: u32 = 500;

const T: f32 = 0.009; // time value in differential eqs



////////////////////////////////////////////////////
// PARTICLE COMPONENT AND MATERIAL DEFINITIONS
///////////////////////////////////////////////////

pub struct Materials {
    pub particle_material: Handle<ColorMaterial>,
}

pub struct Particle {}
// Used to store the "body and tails" of the particle (for a cool effect)
#[derive(Default)]
pub struct ParticleSegments(Vec<Entity>);



////////////////////////////////////////////////////
// GENERAL PARTICLE SYSTEMS
///////////////////////////////////////////////////
pub fn spawn_particle(
    commands: &mut Commands,
    materials: &Res<Materials>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.particle_material.clone(),
            sprite: Sprite::new(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
            transform: Transform::from_xyz(random::<f32>(),random::<f32>(),1.0),
            ..Default::default()
        })
        .insert(Particle {});
}

pub fn spawn_particles(
    mut commands: Commands,
    materials: Res<Materials>,) {
    for _ in 0..N_PARTCLES {
        spawn_particle(&mut commands, &materials);
    }    
}

////////////////////////////////////////////////////
// PARTICLE MOVEMENT SYSTEM
///////////////////////////////////////////////////

pub fn particle_movement(
    mut query: Query<(&Particle, &mut Transform)>,
){
    for (_, mut transform) in query.iter_mut(){
        println!("{:?}", transform.translation);
        transform.translation = attractors::solve_lorenz(&transform, T, attractors::LA_DEFAULT);
    }
}