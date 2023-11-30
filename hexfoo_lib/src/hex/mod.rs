use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::f32::consts::PI;

use crate::movement::Velocity;

const BOUNDS: Vec2 = Vec2::new(640.0, 640.0);
const HEX_Z: f32 = 100.;
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Component)]
pub struct Hex;

#[derive(Bundle)]
struct HexBundle {
    bloom_direction: crate::bloom::BloomDirection,
    hex: Name,
    player: AnimationPlayer,
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
    let hex = Name::new("hex");
    let mut player = AnimationPlayer::default();

    let shape = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(100., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(1.25, 1.4, 1.1))),
        transform: Transform::from_translation(Vec3::new(200., 0., HEX_Z)),
        ..default()
    };

    commands
        .spawn(HexBundle {
            bloom_direction: crate::bloom::BloomDirection::Bright,
            hex,
            shape,
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            player,
        })
        .insert(Hex);
}

fn hex_mover(
    mut animations: ResMut<Assets<AnimationClip>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut AnimationPlayer, &mut Name, &mut Transform), With<Hex>>,
) {
    let (mut player, mut name, mut transform) = query.single_mut();

    let extents = Vec3::from((BOUNDS / 2.0, HEX_Z));

    let mut x_translation = 0.;
    let mut y_translation = 0.;
    let mut z_rotation = 0.;
    let current_rotation = transform.rotation;

    let mut animation = AnimationClip::default();

    animation.add_curve_to_path(
        EntityPath {
            parts: vec![name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.25, 0.5],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::IDENTITY,
            ]),
        },
    );

    let mut spinamation = AnimationClip::default();
    spinamation.add_curve_to_path(
        EntityPath {
            parts: vec![name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.25],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Z, PI / 2.),
                Quat::IDENTITY,
            ]),
        },
    );

    let mut reverse_spinamation = AnimationClip::default();
    reverse_spinamation.add_curve_to_path(
        EntityPath {
            parts: vec![name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.25, 0.5],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Z, (PI / 2.) * -1.),
                Quat::IDENTITY,
            ]),
        },
    );


    let animation_handle = animations.add(animation);
    let spinamation_handle = animations.add(spinamation);
    let reverse_spinamation_handle = animations.add(reverse_spinamation);

    if keyboard_input.just_pressed(KeyCode::W) {
        // y_translation += 200.0;
        transform.translation.y += 200.;
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        transform.translation.x -= 170.;
        // x_translation -= 200.0;
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        transform.translation.y -= 200.;
        // y_translation -= 200.0;
    }

    if keyboard_input.just_pressed(KeyCode::D) {
        transform.translation.x += 170.;
        // x_translation += 200.0;
    }

    if keyboard_input.pressed(KeyCode::E) {
        // z_rotation -= 5.;
        player.play(reverse_spinamation_handle.clone());
    }

    if keyboard_input.pressed(KeyCode::Q) {
        // z_rotation += 5.;
        player.play(spinamation_handle.clone());
    }

    if keyboard_input.just_released(KeyCode::Minus) {
        player.play(animation_handle.clone());
    }

    // transform.translation.x += x_translation * time.delta_seconds();
    // transform.translation.y += y_translation * time.delta_seconds();
    // transform.rotate_z(z_rotation * time.delta_seconds());
    // transform.translation = transform.translation.min(extents).max(-extents);
}
