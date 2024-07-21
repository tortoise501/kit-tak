use bevy::{ecs::query, math::vec3, prelude::*, window::PrimaryWindow};
use serde::{Deserialize, Serialize};
use crate::network::client::AvailableGrid;
use crate::GameState;

pub struct  CellGridPlugin;

impl Plugin for CellGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (initialize_cell_creator,spawn_grid,finish_grid_initializing).chain().run_if(in_state(GameState::StartingGame)))
            .add_systems(Update, (update_cell_textures,validate_gridcells).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, Clone, Copy,Debug,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
pub enum CellState {
    X,
    O,
    Empty,
    Completed
}

#[derive(Component)]
pub struct Grid;
/// Root grid
#[derive(Component)]
struct MainGrid;

#[derive(Bundle)]
struct GridBundle{
    grid: Grid,
    obj: SpriteBundle
}

#[derive(Component,Serialize,Deserialize,Clone)]
pub struct Cell {
    /// Position relative to grid center 
    pub pos: IVec2,
    /// grid positon relative to root grid center
    pub grid_pos:Option<IVec2>,
    /// Current cell state (X, O or empty)
    pub state: CellState,
}

/// True if cell texture must be updated
#[derive(Component)]
pub struct UpdateState(pub bool);

#[derive(Bundle)]
struct CellBundle {
    cell: Cell,
    sprite: SpriteBundle,
    /// Update texture flag
    update_state:UpdateState
}

/// Spawns all needed Entities
fn spawn_grid(mut commands: Commands, cell_spawner:Res<GridCellCreator>) {
    info!("spawning grid");
    let grid: Entity = commands.spawn( (cell_spawner.new_grid(IVec2 { x: 0, y: 0 }),MainGrid)).id();
    for grid_id in 0..=8 {
        info!("adding grid_cell: {}",grid_id);
        let grid_pos = IVec2 {
            x: (grid_id % 3) - 1,
            y: grid_id / 3 - 1,
        };
        let cell_grid = commands.spawn((
            cell_spawner.new_grid(grid_pos),
            Cell {
                grid_pos:None,
                pos: grid_pos,
                state: CellState::Empty
            }
        )).id();
        commands.entity(grid).add_child(cell_grid);
        for cell_id in 0..=8 {
            info!("     adding cell {}",cell_id);
            let pos = IVec2 {
                x: (cell_id % 3) - 1,
                y: cell_id / 3 - 1,
            };
            let cell = commands.spawn(cell_spawner.new_cell(CellState::Empty, pos,Some(grid_pos))).id();
            commands.entity(cell_grid).add_child(cell);
        }
    }
}

/// Debug stuff + main grid sprite resize
fn finish_grid_initializing(
    query: Query<&Transform,With<Cell>>,
    mut grid_q: Query<&mut Sprite,(With<Grid>,Without<MainGrid>)>,
    mut main_grid_q:Query<&mut Sprite,With<MainGrid>>,
    mut next_game_stat: ResMut<NextState<GameState>>,
){
    info!("finishing grid creation");
    for transform in &query{
        info!("cell here: {:?}",transform.translation);
    }
    for mut sprite in &mut grid_q{
        sprite.custom_size = Some(Vec2 { x: 300., y: 300. });
    }
    let mut sprite = main_grid_q.single_mut();
    sprite.custom_size = Some(Vec2{x:900.,y:900.});
    next_game_stat.set(GameState::InGame);
}

/// Updates cells flagged with update flag
fn update_cell_textures(
    mut cell_query: Query<(&mut Handle<Image>,&Cell,&mut UpdateState),Without<Grid>>,
    mut gridcell_query: Query<(&mut Handle<Image>,&Cell),With<Grid>>,
    cell_spawner:Res<GridCellCreator>,
    next_grid_pos: Res<AvailableGrid>,
){
    for (mut texture,cell,mut update) in &mut cell_query{
        if update.0 {
            *texture = cell_spawner.get_texture(cell.state);
            update.0 = false;
        }
    }
    for (mut texture,cell) in &mut gridcell_query{
        *texture = match cell.state {
            CellState::X => cell_spawner.get_texture(cell.state),
            CellState::O => cell_spawner.get_texture(cell.state),
            _ => {
                if let Some(pos) = next_grid_pos.0 {
                    info!("found one");
                    if pos == cell.pos{
                        info!("turning one");
                        cell_spawner.next_grid_texture.clone()
                    }
                    else {
                        cell_spawner.grid_texture.clone()
                    }
                }else {
                    cell_spawner.grid_texture.clone()
                }
            },
        };
    }
}

fn validate_gridcells(
    mut gridcells_q: Query<(Entity,&mut Cell,&Children),With<Grid>>,
    cell_q: Query<&Cell,Without<Grid>>,
    mut commands: Commands,
    // mut main_grid: Query<(&Grid,&Children),With<MainGrid>>,
){
    for (grid,mut cell, children) in &mut gridcells_q {
        if cell.state != CellState::Empty {
            continue;
        }
        let mut state_map = [[CellState::Empty;3];3];
        for child in children {
            let cell = cell_q.get(*child);
            if let Ok(cell) = cell{
                state_map[(cell.pos.x + 1) as usize][(cell.pos.y + 1) as usize] = cell.state;
            }
        }
        for i in 0..3 {
            let mut vertical_count = (0,state_map[i][0]);
            let mut horizontal_count = (0,state_map[0][i]);
            let mut diagonal_count = (0,state_map[1][1]);
            let mut diagonal_count_rev = (0,state_map[1][1]);

            for j in 0..3 {
                if state_map[i][j] != CellState::Empty && state_map[i][j] == vertical_count.1 {
                    vertical_count.0 +=1;
                }
                if state_map[j][i] != CellState::Empty && state_map[j][i] == horizontal_count.1 {
                    horizontal_count.0 +=1;
                }
                if state_map[j][j] != CellState::Empty && state_map[j][j] == diagonal_count.1 {
                    diagonal_count.0 +=1;
                }
                if state_map[j][2-j] != CellState::Empty && state_map[j][2-j] == diagonal_count_rev.1 {
                    diagonal_count_rev.0 +=1;
                }
            }
            if vertical_count.0 == 3 {
                cell.state = vertical_count.1;

            } else if horizontal_count.0 == 3 {
                cell.state = horizontal_count.1;
            } else if diagonal_count.0 == 3 {
                cell.state = diagonal_count.1;
            } else if diagonal_count_rev.0 == 3 {
                cell.state = diagonal_count_rev.1;
            }
            if cell.state != CellState::Empty {
                info!("filled gridcell");
                break;
            }
        }
        if cell.state != CellState::Empty{
            for child in children {
                commands.entity(*child).despawn();
            }
        }
    }
}



/// Creates resource used to spawn cells more efficiently 
fn initialize_cell_creator(asset_server:Res<AssetServer>,mut commands: Commands){
    info!("initializing cell creator");
    commands.insert_resource(GridCellCreator::new(&asset_server));
}

/// Creates cells and grids 
#[derive(Resource)]
struct GridCellCreator{
    pub x_texture:Handle<Image>,
    pub o_texture:Handle<Image>,
    pub empty_texture:Handle<Image>,
    pub grid_texture:Handle<Image>,
    pub next_grid_texture:Handle<Image>,
}

impl GridCellCreator {
    /// Get texture for cell state
    fn get_texture(&self,state:CellState) -> Handle<Image> {
        match state {
            CellState::X => self.x_texture.clone(),
            CellState::O => self.o_texture.clone(),
            CellState::Empty => self.empty_texture.clone(),
            CellState::Completed => self.empty_texture.clone(),
        }
    }
    /// Creates new GridCellCreator
    fn new(asset_server: &Res<AssetServer>) -> GridCellCreator{
        GridCellCreator{
            x_texture: asset_server.load("cell_X.png"),
            o_texture: asset_server.load("cell_O.png"),
            empty_texture: asset_server.load("cell_empty.png"),
            grid_texture: asset_server.load("grid.png"),
            next_grid_texture: asset_server.load("next_grid.png"),
        }
    }
    /// Creates CellBundle 
    fn new_cell(&self,state:CellState,pos:IVec2,grid_pos:Option<IVec2>)-> CellBundle{
        CellBundle {
            cell: Cell { pos,grid_pos,state:CellState::Empty},
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3 {
                    x: pos.x as f32 * 100.,
                    y: pos.y as f32 * 100.,
                    z: -1.,
                }),
                texture: match state {
                    CellState::X => self.x_texture.clone(),
                    CellState::O => self.o_texture.clone(),
                    CellState::Empty => self.empty_texture.clone(),
                    CellState::Completed => self.empty_texture.clone(),
                    
                },
                ..default()
            },
            update_state:UpdateState(false)
        }
    }

    /// Creates GridBundle
    fn new_grid(&self,pos:IVec2)->GridBundle{
        GridBundle{
            grid: Grid,
            obj: SpriteBundle{
                transform: Transform::from_translation(Vec3 {
                    x: pos.x as f32 * 300.,
                    y: pos.y as f32 * 300.,
                    z: -1.,
                }),
                texture: self.grid_texture.clone(),
                ..default()
            },
        }
    }
}