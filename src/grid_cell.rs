use bevy::{math::vec3, prelude::*, window::PrimaryWindow};

pub struct  CellGridPlugin;

impl Plugin for CellGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initialize_cell_creator,spawn_grid,check_cells).chain())
            .add_systems(Update, (update_cell_textures));
    }
}


#[derive(Component, Clone, Copy,Debug,PartialEq,Eq)]
pub enum CellState {
    X,
    O,
    Empty
}

#[derive(Component)]
pub struct Grid;

#[derive(Component)]
struct MainGrid;

#[derive(Bundle)]
struct GridBundle{
    grid: Grid,
    obj: SpriteBundle
}

#[derive(Component)]
pub struct Cell {
    pub pos: IVec2,
    pub grid_pos:Option<IVec2>,
    pub state: CellState,
}

#[derive(Component)]
pub struct UpdateState(pub bool);

#[derive(Bundle)]
struct CellBundle {
    cell: Cell,
    sprite: SpriteBundle,
    update_state:UpdateState
}

fn spawn_grid(mut commands: Commands, cell_spawner:Res<CellCreator>) {
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

fn check_cells(query: Query<&Transform,With<Cell>>,mut main_grid_q:Query<&mut Sprite,With<MainGrid>>){
    for transform in &query{
        info!("cell here: {:?}",transform.translation);
    }
    let mut sprite = main_grid_q.single_mut();
    sprite.custom_size = Some(Vec2{x:900.,y:900.});
}

fn update_cell_textures(mut cell_query: Query<(&mut Handle<Image>,&Cell,&mut UpdateState)>,cell_spawner:Res<CellCreator>){
    for (mut texture,cell,mut update) in &mut cell_query{
        if update.0 {
            *texture = cell_spawner.get_texture(cell.state);
            update.0 = false;
        }
    }
}

fn initialize_cell_creator(asset_server:Res<AssetServer>,mut commands: Commands){
    commands.insert_resource(CellCreator::new(&asset_server));
}
#[derive(Resource)]
struct CellCreator{
    x_texture:Handle<Image>,
    o_texture:Handle<Image>,
    empty_texture:Handle<Image>,
    grid_texture:Handle<Image>
}

impl CellCreator {
    fn get_texture(&self,state:CellState) -> Handle<Image> {
        match state {
            CellState::X => self.x_texture.clone(),
            CellState::O => self.o_texture.clone(),
            CellState::Empty => self.empty_texture.clone(),
        }
    }
    fn new(asset_server: &Res<AssetServer>) -> CellCreator{
        CellCreator{
            x_texture: asset_server.load("cell_X.png"),
            o_texture: asset_server.load("cell_O.png"),
            empty_texture: asset_server.load("cell_empty.png"),
            grid_texture: asset_server.load("grid.png"),
        }
    }
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
                },
                ..default()
            },
            update_state:UpdateState(false)
        }
    }
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