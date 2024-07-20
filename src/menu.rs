use std::net::Ipv4Addr;

use bevy::{prelude::*, utils::info};
use bevy_quinnet::server;
use bevy_egui::{egui::{self, Color32}, EguiContexts, EguiPlugin};
use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_state(ClientMode::Client("".to_string()))
        .insert_state(StartClient::None)
        .add_plugins(EguiPlugin)
        .add_systems(Update, (ui_example_system,start_system).run_if(in_state(GameState::InMenu)));
    }
}




#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ClientMode {
    Server,
    Client(String)
}

fn ui_example_system(
    mut contexts: EguiContexts,
    current_client_mode:Res<State<ClientMode>>, 
    mut next_client_mode:ResMut<NextState<ClientMode>>, 
    current_start_state:Res<State<StartClient>>,
    mut next_start_state:ResMut<NextState<StartClient>>,
) {
    let mut server_addr_string = String::new();
    let mut is_server = match current_client_mode.get() {
        ClientMode::Server => true,
        ClientMode::Client(addr) => {
            server_addr_string = addr.clone();
            false
        },
    };
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui|{
            ui.group(|ui|
            {
                ui.label("Game Creation");
                ui.checkbox(&mut is_server, "Is Server");
                if !is_server{
                    ui.text_edit_singleline(&mut server_addr_string);
                }
                if ui.button("Start").clicked(){
                    match current_client_mode.get() {
                        ClientMode::Server => {
                            next_start_state.set(StartClient::Server)
                        },
                        ClientMode::Client(addr) => {
                            next_start_state.set(match addr.parse::<Ipv4Addr>() {
                                Ok(serv_addr) => StartClient::Client(serv_addr),
                                Err(_) => {
                                    StartClient::IncorrectAddress
                                },
                            })
                        }
                    }
                }
                if current_start_state.get() == &StartClient::IncorrectAddress {
                    ui.colored_label(Color32::RED, "incorrect address");
                }
            });
            
        });
        
    });
    match is_server {
        true => next_client_mode.set(ClientMode::Server),
        false => next_client_mode.set(ClientMode::Client(server_addr_string)),
    }
}


use crate::network::StartClient;

fn start_system(
    mut commands:Commands,
    start_client:Res<State<StartClient>>,
    mut next_game_state:ResMut<NextState<GameState>>
){
    match start_client.get() {
        StartClient::Client(addr) => {
            next_game_state.set(GameState::Connecting);
            // TODO: idk do something
            info!("starting in client mode");
        },
        StartClient::Server => {
            next_game_state.set(GameState::CreatingServer);
            // TODO: idk do something
            info!("starting in server mode");
        },
        _ => return
    }
}