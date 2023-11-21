use bevy::prelude::*;

pub struct PositionPlugin;

#[derive(Component, Debug)]
struct Position { x: f32, y: f32 }

#[derive(Component, Debug)]
struct Velocity { x: f32, y: f32 }

fn position_updater(mut query: Query<(&mut Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
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
