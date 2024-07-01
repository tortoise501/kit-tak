mod camera;
mod grid_cell;
use std::cell::Cell;

use bevy::{pbr::deferred, prelude::*, transform::commands};
use camera::CameraPlugin;
use grid_cell::CellGridPlugin;

fn main() {
    let _app = App::new().add_plugins((DefaultPlugins,CameraPlugin,CellGridPlugin)).run();
}

