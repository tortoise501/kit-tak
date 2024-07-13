use std::{collections::VecDeque, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};

use crate::{grid_cell::*, network::GameEvent};
// use crate::player::
use bevy::{ecs::entity, prelude::*, utils::info, window::PrimaryWindow};
use bevy_quinnet::{server::{certificate::CertificateRetrievalMode, QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration}, shared::channels::ChannelsConfiguration};
use serde::{Deserialize, Serialize};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default()).add_systems(Startup, start_listening).add_systems(Update, handle_client_messages);
    }
}

/// Starts listening for connection
fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6000),
            CertificateRetrievalMode::GenerateSelfSigned { server_hostname: "serv".to_string() },
            ChannelsConfiguration::default(),
        )
        .unwrap();
}

/// Here server just broadcasts messages it gets
fn handle_client_messages(
    mut server: ResMut<QuinnetServer>,
) {
    let mut endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<GameEvent>(client_id) {
            endpoint.broadcast_message(message.1);
        }
    }
}