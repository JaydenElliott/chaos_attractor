// External Modules
use bevy::prelude::*;
use bevy::render::pass::ClearColor;


// Internal Modules
mod attractors;
mod particles;

const WINDOW_SIZE: f32 = 800.0;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Chaotic Attractors".to_string(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(particles::spawn_particles.system()))
        .add_system(particles::particle_movement.system())
        .add_plugins(DefaultPlugins)
        .run();
}



fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale.z = 100.0;    
    commands.spawn_bundle(camera);
    commands.insert_resource(particles::Materials {
        particle_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}



