mod camera;
mod grid_cell;
use bevy::{prelude::*, transform::commands};
use camera::CameraPlugin;

fn main() {
    let _app = App::new().add_plugins((DefaultPlugins,CameraPlugin)).run();
}


fn create_grid(mut commands:Commands){
    
}