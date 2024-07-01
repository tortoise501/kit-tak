use bevy::{math::vec3, prelude::*, window::PrimaryWindow};

pub struct  CellGridPlugin;

impl Plugin for CellGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initialize_cell_creator,spawn_grid,check_cells).chain())
            .add_systems(Update, (update_cell_textures,handle_mouse_clicks));
    }
}


#[derive(Component, Clone, Copy)]
enum CellState {
    X,
    O,
    Empty
}

#[derive(Component)]
struct Grid;

#[derive(Component)]
struct MainGrid;

#[derive(Bundle)]
struct GridBundle{
    grid: Grid,
    obj: SpriteBundle
}

#[derive(Component)]
struct Cell {
    pos: IVec2,
    state: CellState,
}

#[derive(Component)]
struct UpdateState(bool);

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
        let pos = IVec2 {
            x: (grid_id % 3) - 1,
            y: grid_id / 3 - 1,
        };
        let cell_grid = commands.spawn((
            cell_spawner.new_grid(pos),
            Cell {
                pos,
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
            let cell = commands.spawn(cell_spawner.new_cell(CellState::Empty, pos)).id();
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
    fn new_cell(&self,state:CellState,pos:IVec2)-> CellBundle{
        CellBundle {
            cell: Cell { pos,state:CellState::Empty},
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

fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cell_q: Query<(&mut Cell,&mut UpdateState,&GlobalTransform),Without<Grid>>
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
            for (mut cell, mut update, transform) in &mut cell_q{
                if (transform.translation().x - world_position.x).abs() < 45. && (transform.translation().y - world_position.y).abs() < 45. {
                    update.0 = true;
                    cell.state = CellState::O;
                }
            }
        }
    }
}