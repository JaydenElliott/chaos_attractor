use bevy::prelude::*;
use crate::attractors;
use rand::prelude::random;


////////////////////////////////////////////////////
// PARTICLE CONSTANTS
///////////////////////////////////////////////////

const PARTICLE_SIZE: f32 = 0.2;

const N_PARTCLES: u32 = 2000;

const RANDOM_SPAWNER_WIDTH: f32 = 100.0;


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

pub fn spawn_particles(
    mut commands: Commands,
    materials: Res<Materials>,) {
    for _ in 0..N_PARTCLES {
        spawn_particle(&mut commands, &materials);
    }    
}


pub fn spawn_particle(
    commands: &mut Commands,
    materials: &Res<Materials>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.particle_material.clone(),
            sprite: Sprite::new(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
            transform: Transform::from_xyz(random::<f32>()*RANDOM_SPAWNER_WIDTH,random::<f32>()*RANDOM_SPAWNER_WIDTH,1.0),
            ..Default::default()
        })
        .insert(Particle {});
}


////////////////////////////////////////////////////
// PARTICLE MOVEMENT SYSTEM
///////////////////////////////////////////////////

pub fn particle_movement(mut query: Query<(&Particle, &mut Transform)>){
    for (_, mut transform) in query.iter_mut(){
        transform.translation = attractors::solve_lorenz(&transform, attractors::LA_DEFAULT);
    }
}