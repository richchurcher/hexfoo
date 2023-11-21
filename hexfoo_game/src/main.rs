use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_plugins(hexfoo_lib::position::PositionPlugin)
        .add_plugins(hexfoo_lib::hex::HexPlugin)
        .run();
}
