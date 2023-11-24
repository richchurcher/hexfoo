use bevy::prelude::*;

use crate::movement::Velocity;

pub struct PositionPlugin;

#[derive(Component, Debug)]
pub struct Position { x: f32, y: f32 }

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

fn position_updater(mut query: Query<(&mut Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.value.x;
        position.y += velocity.value.y;
    }
}

fn position_reporter(query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        println!("entity {:?} :: {:?}", entity, position);
    }
}

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        info!("Building PositionPlugin...");
        app.add_systems(Update, (position_updater, position_reporter));
    }
}
