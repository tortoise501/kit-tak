mod camera;
mod grid_cell;
mod player;
mod server;

use bevy::{pbr::deferred, prelude::*, transform::commands};
use camera::CameraPlugin;
use grid_cell::CellGridPlugin;
use player::PlayerPlugin;
use server::ServerPlugin;

fn main() {
    let _app = App::new().add_plugins((DefaultPlugins,PlayerPlugin,CameraPlugin,CellGridPlugin,ServerPlugin)).run();
}

