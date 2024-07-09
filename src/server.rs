use std::collections::VecDeque;

use crate::grid_cell::*;
// use crate::player::
use bevy::{ecs::entity, prelude::*, utils::info, window::PrimaryWindow};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EventQueue(VecDeque::new()))
            .insert_resource(AvailableGrid(None))
            .insert_resource(CurrentEvent(None))
            .insert_state(CurrentPlayer::O)
            .insert_state(UpdatePlayers::None)
            .add_systems(Update, ((occupy_cell,prevent_available_grid_lock),process_event_queue).chain());
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

fn occupy_cell (
    mut cell_q: Query<(Entity, &mut Cell, &mut UpdateState, &GlobalTransform), Without<Grid>>,
    mut available_grid:ResMut<AvailableGrid>,
    current_event: Res<CurrentEvent>,
    curr_player: Res<State<CurrentPlayer>>,
    mut next_player: ResMut<NextState<CurrentPlayer>>,
    mut next_update_players:ResMut<NextState<UpdatePlayers>>

) {
    if let Some(GameEvent::ClickedCell(clicked_entity, change_to_state)) = current_event.0 {
        if curr_player.to_state() != change_to_state {
            return
        };

        for (entity, mut cell,mut update,_) in &mut cell_q{
            match available_grid.0 {
                Some(required_grid) if required_grid != cell.grid_pos.unwrap() => continue, //? UNWRAP 
                _ => {
                    if entity == clicked_entity && cell.state == CellState::Empty {
                        cell.state = change_to_state;
                        available_grid.0 = Some(cell.pos);
                        *update = UpdateState(true);
                        next_player.set(curr_player.get_next());
                        next_update_players.set(UpdatePlayers::Update);
                        return;
                    }
                },
            }
        }
    }
}

#[derive(Resource)]
pub struct EventQueue(pub VecDeque<GameEvent>);

pub enum GameEvent {
    ClickedCell(Entity ,CellState),
}

#[derive(Resource)]
struct CurrentEvent(Option<GameEvent>); 

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
enum UpdatePlayers {
    Update,
    None
}


fn process_event_queue(mut current_event:ResMut<CurrentEvent>, mut event_queue:ResMut<EventQueue>,update_players:Res<State<UpdatePlayers>>,next_update_players:ResMut<NextState<UpdatePlayers>>){
    *current_event = CurrentEvent(event_queue.0.pop_front());
}