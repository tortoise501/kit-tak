use crate::grid_cell::*;
use bevy::{prelude::*, utils::info, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ThisPlayer(CellState::O))
            .insert_resource(ClickProgress::None)
            .insert_resource(AvailableGrid(None))
            .insert_state(CurrentPlayer::O)
            .add_systems(Update, (handle_mouse_clicks,occupy_cell,prevent_available_grid_lock).chain());
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

#[derive(Resource)]
enum ClickProgress {
    None,
    Clicked(Entity ,CellState),
}


fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    cell_q: Query<(Entity, &Cell, &GlobalTransform), Without<Grid>>,
    this_player: Res<ThisPlayer>,
    mut click_progress: ResMut<ClickProgress>,
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
            for (entity,  cell, transform) in &cell_q {
                if (transform.translation().x - world_position.x).abs() < 45.
                    && (transform.translation().y - world_position.y).abs() < 45.
                    && cell.state == CellState::Empty
                {
                    *click_progress = ClickProgress::Clicked(entity, this_player.0);
                    break;
                }
            }
        }
    }
}


fn occupy_cell (
    mut cell_q: Query<(Entity, &mut Cell, &mut UpdateState, &GlobalTransform), Without<Grid>>,
    mut available_grid:ResMut<AvailableGrid>,
    mut click_progress: ResMut<ClickProgress>,
    curr_player: Res<State<CurrentPlayer>>,
    mut next_player: ResMut<NextState<CurrentPlayer>>,

) {
    if let ClickProgress::Clicked(clicked_entity, change_to_state) = *click_progress {
        if curr_player.to_state() != change_to_state {
            *click_progress = ClickProgress::None;
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
                        *click_progress = ClickProgress::None;
                        return;
                    }
                },
            }
        }
    }
}