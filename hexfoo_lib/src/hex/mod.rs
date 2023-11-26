use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::movement::Velocity;

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Hex;

#[derive(Bundle)]
struct HexBundle {
    bloom_direction: crate::bloom::BloomDirection,
    shape: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
}

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hex_spawner).add_systems(Update, hex_mover);
    }
}

fn hex_spawner(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(100., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(6.25, 7.4, 7.1))),
        transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
        ..default()
    };

    commands
        .spawn(HexBundle {
            bloom_direction: crate::bloom::BloomDirection::Bright,
            shape,
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
        })
        .insert(Hex);
}

fn hex_mover(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Hex>>,
) {
    let mut transform = query.single_mut();

    let mut x_translation = 0.;
    let mut y_translation = 0.;
    let mut z_rotation = 0.;

    if keyboard_input.pressed(KeyCode::W) {
        y_translation += 200.0;
    }

    if keyboard_input.pressed(KeyCode::A) {
        x_translation -= 200.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        y_translation -= 200.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        x_translation += 200.0;
    }

    if keyboard_input.pressed(KeyCode::E) {
        z_rotation -= 5.;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        z_rotation += 5.;
    }

    transform.translation.x += x_translation * time.delta_seconds();
    transform.translation.y += y_translation * time.delta_seconds();
    transform.rotate_z(z_rotation * 1. * time.delta_seconds());

    // bound the ship within the invisible level bounds
    // let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    // transform.translation = transform.translation.min(extents).max(-extents);
}
