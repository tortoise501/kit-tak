use std::cmp::min;

use bevy::{prelude::*, render::camera::ScalingMode, transform};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin,LogDiagnosticsPlugin};
// use bevy_lunex::prelude::MainUi;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
        .add_systems(Update, resize_camera)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default());
    }
}

fn add_camera(mut commands: Commands){
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 10.),
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize(1.),
                ..default()
            },
            ..default()
        }
    );
}

fn resize_camera(
    mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
    window: Query<&Window>
){
    let window = window.get_single().unwrap();
    let min = min(window.width() as i32, window.height() as i32);
    if min < 900 {
        let mut camera = camera.get_single_mut().unwrap(); 
        camera.scaling_mode = ScalingMode::WindowSize((min as f32) / 900.);
    }
    
}