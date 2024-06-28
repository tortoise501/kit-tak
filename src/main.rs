mod camera;
mod grid_cell;
use std::cell::Cell;

use bevy::{pbr::deferred, prelude::*, transform::commands};
use camera::CameraPlugin;
use grid_cell::*;

fn main() {
    let _app = App::new().add_plugins((DefaultPlugins,CameraPlugin)).add_systems(Startup, (spawn_grid,check_cells).chain()).run();
}

