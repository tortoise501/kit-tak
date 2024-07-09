use std::{collections::VecDeque, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};

use crate::{grid_cell::*};
// use crate::player::
use bevy::{ecs::entity, prelude::*, utils::info, window::PrimaryWindow};
use serde::{Deserialize, Serialize};
pub mod server;
pub mod client;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
enum CurrentPlayer {
    X,
    O,
}
impl CurrentPlayer {
    fn get_next(&self) -> CurrentPlayer {
        match self {
            CurrentPlayer::X => CurrentPlayer::O,
            CurrentPlayer::O => CurrentPlayer::X,
        }
    }
    fn to_state(&self) -> CellState {
        match self {
            CurrentPlayer::X => CellState::X,
            CurrentPlayer::O => CellState::O,
        }
    }
}

#[derive(Resource)]
pub struct AvailableGrid(pub Option<IVec2>);


#[derive(Resource)]
pub struct SendEventQueue(pub VecDeque<GameEvent>);
#[derive(Resource)]
pub struct ReceiveEventQueue(pub VecDeque<GameEvent>);

#[derive(Resource)]
struct CurrentEvent(Option<GameEvent>); 

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
enum UpdatePlayers {
    Update,
    None
}

#[derive(Serialize,Deserialize)]
pub enum GameEvent {
    ClickedCell(Cell),
}




#[derive(Serialize,Deserialize)]
struct Package(Vec<Cell>);