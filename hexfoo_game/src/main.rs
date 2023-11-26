use bevy::prelude::*;

// https://bevy-cheatbook.github.io/programming/states.html
// https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
}

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_plugins(hexfoo_lib::position::PositionPlugin)
        .add_plugins(hexfoo_lib::hex::HexPlugin)
        .add_plugins(hexfoo_lib::bloom::BloomPlugin)
        .run();
}
