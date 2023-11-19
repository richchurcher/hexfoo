use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_plugins(hexfoo_lib::this_has_no_obvious_name_yet::ThisHasNoObviousNameYetPlugin)
        .add_plugins(hexfoo_lib::hex::HexPlugin)
        .run();
}
