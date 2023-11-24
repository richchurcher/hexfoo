use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::position::Position;
use crate::movement::Velocity;

const STARTING_VELOCITY: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const STARTING_POSITION: Position = Position::new(0.0, 0.0);

#[derive(Bundle)]
struct HexBundle {
    bloom_direction: crate::bloom::BloomDirection,
    position: Position,
    shape: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
}

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hex_spawner);
    }
}

fn hex_spawner(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(100., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(6.25, 7.4, 7.1))),
        transform: Transform::from_translation(Vec3::new(50., 50., 0.)),
        ..default()
    };

    commands.spawn(HexBundle {
        bloom_direction: crate::bloom::BloomDirection::Bright,
        position: STARTING_POSITION,
        shape,
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
    });
}
