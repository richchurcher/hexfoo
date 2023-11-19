use bevy::prelude::*;

pub struct ThisHasNoObviousNameYetPlugin;

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

fn hex_spawner(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

impl Plugin for ThisHasNoObviousNameYetPlugin {
    fn build(&self, app: &mut App) {
        info!("Building ThisHasNoObviousNameYetPlugin...");
        app.add_systems(Startup, hex_spawner);
        app.add_systems(Update, (position_updater, position_reporter));
    }
}
