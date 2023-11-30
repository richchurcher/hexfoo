use bevy::prelude::*;

pub struct PositionPlugin;

#[derive(Component, Debug)]
pub struct Position { pub x: f32, pub y: f32 }

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

fn position_reporter(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        println!("entity {:?} :: {:?}", entity, transform);
    }
}

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        info!("Building PositionPlugin...");
        app.add_systems(Update, position_reporter);
    }
}
