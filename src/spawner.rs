use crate::particles::{Materials, Particle, PARTICLE_SIZE};
use bevy::prelude::*;

// Particle spawner system for spacebar clicking events
pub fn spawner_system(
    mut commands:  Commands,
    mut materials: Res<Materials>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        commands
        .spawn_bundle(SpriteBundle {
            material: materials.new_particle_material.clone(),
            sprite: Sprite::new(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
            transform: Transform::from_xyz(
                3.0,
                5.0,
                1.0,
            ),
            ..Default::default()
        })
        .insert(Particle { new: false });
        commands
        .spawn_bundle(SpriteBundle {
            material: materials.new_particle_material.clone(),
            sprite: Sprite::new(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
            transform: Transform::from_xyz(
                -3.0,
                -5.0,
                1.0,
            ),
            ..Default::default()
        })
        .insert(Particle { new: false });}
}

