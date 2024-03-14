mod abstract_pattern_art;
use crate::abstract_pattern_art::input::get_input;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_asset_loader::prelude::*;
use crate::abstract_pattern_art::shape::{art_generation_system, create_shape_system};

fn main() {
    let matches = get_input();
    App::new()
        .insert_resource(matches)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, art_generation_system)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, create_shape_system.after(art_generation_system))
        .run();
    // Access user input using app.arg("argument_name").unwrap().value()
}

fn setup_camera(mut commands: Commands){
    let mut camera_bundle = Camera2dBundle::default();
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize(1.0),
        ..OrthographicProjection::default()
    };
// change the settings we want to change:
    camera_bundle.camera.clear_color = ClearColorConfig::from(Color::GRAY);
    commands.spawn(camera_bundle);
}

