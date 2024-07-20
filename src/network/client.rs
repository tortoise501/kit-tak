use std::{collections::VecDeque, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};

use crate::{grid_cell::*, network::{GameEvent,StartClient},GameState};
// use crate::player::
use bevy::{ecs::entity, prelude::*, utils::info, window::PrimaryWindow};
use bevy_quinnet::{client::{certificate::CertificateVerificationMode, connection::ClientEndpointConfiguration, QuinnetClient, QuinnetClientPlugin}, shared::channels::ChannelsConfiguration};

use super::{ReceiveEventQueue, SendEventQueue};

pub struct ClientPlugin;
impl ClientPlugin {
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetClientPlugin::default())
        .insert_resource(ThisPlayer(CellState::Empty))
        .insert_resource(ReceiveEventQueue(VecDeque::new()))
        .insert_resource(SendEventQueue(VecDeque::new()))
        .insert_resource(AvailableGrid(None))
        .insert_state(CurrentPlayer::O)
        .add_systems(Update, start_connection.run_if(in_state(GameState::Connecting)))
        .add_systems(Update, (handle_mouse_clicks,send_messages_to_server).chain().run_if(in_state(GameState::InGame)))
        .add_systems(Update, (receive_server_messages,(occupy_cell,prevent_available_grid_lock).chain()).run_if(in_state(GameState::InGame)));
    }
}
#[derive(Resource,Clone, Copy)]
struct ThisPlayer(CellState);

/// Handles mouse click input updating events to send queue by adding cell that was clicked
fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cell_q: Query<(Entity, &Cell, &GlobalTransform), Without<Grid>>,
    mut send_event_queue: ResMut<SendEventQueue>,
    current_player: Res<State<CurrentPlayer>>,
    this_player:Res<ThisPlayer>
) {

    if current_player.to_state() != this_player.0 {
        return;
    }
    let win = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_q.single();
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(world_position) = win
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            for (entity,  cell, transform) in &cell_q {
                if (transform.translation().x - world_position.x).abs() < 45.
                    && (transform.translation().y - world_position.y).abs() < 45.
                    && cell.state == CellState::Empty
                {
                    send_event_queue.0.push_back(GameEvent::ClickedCell(cell.clone()));
                    break;
                }
            }
        }
    }
}

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
struct AvailableGrid(Option<IVec2>);

/// Prevents situations when player doesn't have valid cells to click because grid is filled
fn prevent_available_grid_lock (
    grid_cell_q:Query<(&Children,&Cell),(With<Grid>,With<Cell>)>,
    cell_q:Query<(Entity,&Cell)>,
    mut available_grid:ResMut<AvailableGrid>,

){
    match available_grid.0 {
        Some(available_grid_pos) => {
            for (children,grid_cell) in &grid_cell_q{
                if grid_cell.pos == available_grid_pos {
                    let mut unfilled_count = 0;
                    for (entity,cell) in &cell_q {
                        if children.contains(&entity) && cell.state == CellState::Empty{
                            unfilled_count+=1;
                        }
                    }
                    if unfilled_count == 0{
                        available_grid.0 = None;
                    }
                    return;
                }
            }
        },
        None => return,
    }
}

/// Occupies cells if they are in received events queue
/// 
/// It is actually an event handler but there no events other then ClickedCell 
fn occupy_cell (
    mut cell_q: Query<(&mut Cell, &mut UpdateState), Without<Grid>>,
    mut available_grid:ResMut<AvailableGrid>,
    curr_player: Res<State<CurrentPlayer>>,
    mut next_player: ResMut<NextState<CurrentPlayer>>,
    mut received_event_queue: ResMut<ReceiveEventQueue>,
) {

    for event in &received_event_queue.0 {
        match event {
            GameEvent::ClickedCell(clicked_cell) => {
                for ( mut cell,mut update) in &mut cell_q{
                    match available_grid.0 {
                        Some(required_grid) if required_grid != cell.grid_pos.unwrap() => continue, //? UNWRAP 
                        _ => {
                            if cell.pos == clicked_cell.pos && cell.grid_pos == clicked_cell.grid_pos && cell.state == CellState::Empty {
                                cell.state = curr_player.to_state();
                                available_grid.0 = Some(cell.pos);
                                *update = UpdateState(true);
                                next_player.set(curr_player.get_next());
                                return;
                            }
                        },
                    }
                }
            },
        }
    }
    received_event_queue.0.clear();
}





/// Start connection with server
fn start_connection(
    mut client: ResMut<QuinnetClient>,
    client_mode_info: Res<State<StartClient>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut this_player: ResMut<ThisPlayer>,
) {
    if client.is_connected() 
    {
        info!("successfully created connection to server");
        next_game_state.set(GameState::StartingGame);
    } else if client.connections().count() == 0 {
        info!("attempting to create connection to server");
        // setting this player cell type according to client type
        *this_player = match client_mode_info.get() {
            StartClient::Server => ThisPlayer(CellState::X),
            StartClient::Client(_) => ThisPlayer(CellState::O),
            _ => panic!("UNEXPECTED BEHAVIOR AAAAAAAAAAAAA")
        };

        let _ = client
        .open_connection(
            ClientEndpointConfiguration::from_ips(
                IpAddr::V4(match client_mode_info.get() {
                    StartClient::Client(addr) => addr.clone(),
                    StartClient::Server => Ipv4Addr::new(127, 0, 0, 1),
                    _ => todo!()
                }),
                6000,
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                0,
            ),
            CertificateVerificationMode::SkipVerification,
            ChannelsConfiguration::default(),
        );
    }
    
    
}

/// Sends messages from event queue to server
fn send_messages_to_server(mut client: ResMut<QuinnetClient>,mut messages:ResMut<SendEventQueue>){
    while messages.0.len() > 0 {
        if let Some(message) = messages.0.pop_front() {
            let connection = client.connection();
            connection.send_message(message);
        }
    }
}
/// Receives messages from server and puts them in receive queue
fn receive_server_messages(
    mut client: ResMut<QuinnetClient>,
    mut received_event_queue: ResMut<ReceiveEventQueue>,
) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<GameEvent>() {
        received_event_queue.0.push_back(message.1);
    }
}
