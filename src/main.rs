// External Modules
use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::render::pass::ClearColor;
use spawner::spawner_system;

// Internal Modules
mod attractors;
mod particles;
mod spawner;

const WINDOW_HEIGHT: f32 = 800.0;
const WINDOW_WIDTH: f32 = 1200.0;
const CAMERA_SCALE: f32 = 0.08;

#[allow(dead_code)]
const TOP_RIGHT: [f32;2] =  [WINDOW_WIDTH * CAMERA_SCALE / 2.0, WINDOW_HEIGHT * CAMERA_SCALE / 2.0];
#[allow(dead_code)]
const TOP_LEFT: [f32;2] =  [-1.0 * WINDOW_WIDTH * CAMERA_SCALE / 2.0, WINDOW_HEIGHT * CAMERA_SCALE / 2.0];
#[allow(dead_code)]
const BOTTOM_RIGHT: [f32;2] =  [WINDOW_WIDTH * CAMERA_SCALE / 2.0, -1.0 * WINDOW_HEIGHT * CAMERA_SCALE / 2.0];
#[allow(dead_code)]
const BOTTOM_LEFT: [f32;2] =  [-1.0 * WINDOW_WIDTH * CAMERA_SCALE / 2.0, -1.0 * WINDOW_HEIGHT * CAMERA_SCALE / 2.0]; 

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Chaotic Attractors".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup",
            SystemStage::single(particles::spawn_particles.system()),
        )
        .add_system(particles::particle_movement.system())
        // .add_system(spawner::spawner_system.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(spawner_system.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = CAMERA_SCALE;
    commands.spawn_bundle(camera);
    commands.insert_resource(particles::Materials {
        particle_material: materials.add(Color::WHITE.into()),
        new_particle_material: materials.add(Color::rgb(0.8, 0.08, 0.58).into())
    });
    
}
