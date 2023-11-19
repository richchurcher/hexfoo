use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct HexBundle {
    bloom_direction: crate::bloom::BloomDirection,
    velocity: Velocity,
}

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hex_spawner).add_plugins(crate::bloom::BloomPlugin);
    }
}

fn hex_spawner(mut commands: Commands) {
    commands.spawn(HexBundle {
        bloom_direction: crate::bloom::BloomDirection::Bright,
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
    });
}
