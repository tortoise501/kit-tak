use std::{collections::VecDeque, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};

use crate::{grid_cell::*};
// use crate::player::
use bevy::{ecs::entity, prelude::*, utils::info, window::PrimaryWindow};
use serde::{Deserialize, Serialize};
pub mod server;
pub mod client;

/// Queue of events to be send to client from server or from server to client
#[derive(Resource)]
pub struct SendEventQueue(pub VecDeque<GameEvent>);

/// Queue of received events ready to be processed by client/server
#[derive(Resource)]
pub struct ReceiveEventQueue(pub VecDeque<GameEvent>);

/// Game Event
#[derive(Serialize,Deserialize)]
pub enum GameEvent {
    ClickedCell(Cell),
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartClient{
    /// Don't start client
    None,
    /// Attempted to start but couldn't use given ip address
    IncorrectAddress,
    /// Start as client and connect to given address
    Client(Ipv4Addr),
    /// Start as server
    Server
}