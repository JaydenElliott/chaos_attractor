use crate::attractors;
use bevy::prelude::*;
use rand::prelude::random;

////////////////////////////////////////////////////
// PARTICLE CONSTANTS
///////////////////////////////////////////////////

pub const PARTICLE_SIZE: f32 = 0.2;

const N_PARTCLES: u32 = 2000;

// Increasing this leads to particles spawning further away from the center
// and subsequently having a higher probability of converging to infinity
// and disapearing ... i.e. not cool.
const RANDOM_SPAWNER_MULTIPLIER: f32 = 1.0;

////////////////////////////////////////////////////
// PARTICLE COMPONENT AND MATERIAL DEFINITIONS
///////////////////////////////////////////////////

pub struct Materials {
    pub particle_material: Handle<ColorMaterial>,
    pub new_particle_material: Handle<ColorMaterial>,
}

pub struct Particle {
    pub new: bool,
}

////////////////////////////////////////////////////
// GENERAL PARTICLE SYSTEMS
///////////////////////////////////////////////////

pub fn spawn_particles(mut commands: Commands, materials: Res<Materials>) {
    for _ in 0..N_PARTCLES {
        spawn_particle(&mut commands, &materials);
    }
}

pub fn spawn_particle(commands: &mut Commands, materials: &Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.particle_material.clone(),
            sprite: Sprite::new(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
            transform: Transform::from_xyz(
                random::<f32>() * RANDOM_SPAWNER_MULTIPLIER,
                random::<f32>() * RANDOM_SPAWNER_MULTIPLIER,
                1.0,
            ),
            ..Default::default()
        })
        .insert(Particle { new: false });
}

////////////////////////////////////////////////////
// PARTICLE MOVEMENT SYSTEM
///////////////////////////////////////////////////

pub fn particle_movement(mut query: Query<(&Particle, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation = attractors::solve_lorenz(&transform, attractors::LORENZ_DEFAULT);
    }
}
