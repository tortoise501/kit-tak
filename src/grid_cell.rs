use bevy::{math::vec3, prelude::*};

#[derive(Component, Clone, Copy)]
pub enum CellState {
    X,
    O,
}

#[derive(Component)]
pub struct Grid;

#[derive(Bundle)]
pub struct GridBundle{
    grid: Grid,
    obj:SpatialBundle
}
impl GridBundle {
    fn from_ivec2(pos:IVec2)->GridBundle{
        GridBundle{
            grid: Grid,
            obj: SpatialBundle{
                transform: Transform::from_translation(Vec3 {
                    x: pos.x as f32 * 300.,
                    y: pos.y as f32 * 300.,
                    z: 0.,
                }),
                ..default()
            },
        }
    }
}


#[derive(Component)]
pub struct Cell {
    pos: IVec2,
}

#[derive(Bundle)]
pub struct CellBundle {
    cell: Cell,
    sprite: SpriteBundle,
}
impl CellBundle {
    fn from_ivec2(pos: IVec2, asset_server: &Res<AssetServer>) -> CellBundle {
        CellBundle {
            cell: Cell { pos },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3 {
                    x: pos.x as f32 * 100.,
                    y: pos.y as f32 * 100.,
                    z: 0.,
                }),
                texture: asset_server.load("cell_X.png"),
                ..default()
            },
        }
    }
}

pub fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grid: Entity = commands.spawn((GridBundle::from_ivec2(IVec2 { x: 0, y: 0 }))).id();
    for i in 0..=8 {
        info!("adding grid_cell: {}",i);
        let pos = IVec2 {
            x: (i % 3) - 1,
            y: i / 3 - 1,
        };
        let cell_grid = commands.spawn((
            GridBundle::from_ivec2(pos),
            Cell {
                pos,
            }
        )).id();
        commands.entity(grid).add_child(cell_grid);
        for j in 0..=8 {
            info!("     adding cell {}",j);
            let cell = commands.spawn(CellBundle::from_ivec2(
                IVec2 {
                    x: (j % 3) - 1,
                    y: j / 3 - 1,
                },
                &asset_server,
            )).id();
            commands.entity(cell_grid).add_child(cell);
        }
    }
}

pub fn check_cells(query: Query<&Transform,With<Cell>>){
    for transform in &query{
        info!("cell here: {:?}",transform.translation);
    }
}