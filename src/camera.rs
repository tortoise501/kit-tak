use bevy::{prelude::*, transform};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}

fn add_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(0., 0., 10.),
        ..default()
    });
}