mod camera;
mod grid_cell;
mod network;


use std::net::{IpAddr, Ipv4Addr};

use bevy::{ecs::entity, pbr::deferred, prelude::*, transform::commands};
use bevy_quinnet::{client::{certificate::CertificateVerificationMode, connection::ClientEndpointConfiguration, QuinnetClient, QuinnetClientPlugin}, shared::channels::ChannelsConfiguration};
use camera::CameraPlugin;
use grid_cell::CellGridPlugin;
use serde::{Deserialize, Serialize};
use network::server::ServerPlugin;
use network::client::ClientPlugin;


fn main() {

    let args = std::env::args().collect::<Vec<String>>();
    let username = &args[1];

    if username == "serv" {
        let _app = App::new().add_plugins((DefaultPlugins,CameraPlugin,CellGridPlugin,ClientPlugin::new(grid_cell::CellState::O),ServerPlugin)).run();
    } else {
        let _app = App::new().add_plugins((DefaultPlugins,CameraPlugin,CellGridPlugin,ClientPlugin::new(grid_cell::CellState::X))).run();
    }

}


