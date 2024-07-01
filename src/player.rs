use crate::grid_cell::*;
use bevy::{prelude::*, utils::info, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ThisPlayer(CellState::O))
            .insert_resource(AvailableGrid(None))
            .insert_state(CurrentPlayer::O)
            .add_systems(Update, (handle_mouse_clicks,prevent_available_grid_lock).chain());
    }
}

#[derive(Resource)]
struct ThisPlayer(CellState);

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
enum CurrentPlayer {
    X,
    O,
}
impl CurrentPlayer {
    fn next(&mut self) {
        match self {
            CurrentPlayer::X => *self = CurrentPlayer::O,
            CurrentPlayer::O => *self = CurrentPlayer::X,
        }
    }
}

#[derive(Resource)]
struct AvailableGrid(Option<IVec2>);

fn prevent_available_grid_lock(
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


fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cell_q: Query<(&mut Cell, &mut UpdateState, &GlobalTransform), Without<Grid>>,
    this_player: Res<ThisPlayer>,
    curr_player: Res<State<CurrentPlayer>>,
    mut next_player: ResMut<NextState<CurrentPlayer>>,
    mut available_grid:ResMut<AvailableGrid>
) {
    let win = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_q.single();
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("click at {:?}", win.cursor_position());
        if let Some(world_position) = win
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            println!("click at world {:?}", world_position);
            for (mut cell, mut update, transform) in &mut cell_q {
                if (transform.translation().x - world_position.x).abs() < 45.
                    && (transform.translation().y - world_position.y).abs() < 45.
                    && cell.state == CellState::Empty
                {
                    match available_grid.0 {
                        Some(available_grid_pos) =>{
                            if cell.grid_pos.unwrap() == available_grid_pos{
                                available_grid.0 = Some(cell.pos);
                            }
                            else {
                                return;
                            }
                        },
                        None => available_grid.0 = Some(cell.pos),
                        
                    }

                    info!("thisP={:?}    currP={:?}", this_player.0, **curr_player);
                    match this_player.0 {
                        CellState::X if *curr_player == CurrentPlayer::X => {
                            next_player.set(CurrentPlayer::O)
                        }
                        CellState::O if *curr_player == CurrentPlayer::O => {
                            next_player.set(CurrentPlayer::X)
                        }
                        _ => return,
                    }
                    update.0 = true;
                    cell.state = CellState::O;
                    break;
                }
            }
        }
    }
}
