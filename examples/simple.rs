use bevy::prelude::*;
use bevy_window_icon::WindowIconPlugin;
pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WindowIconPlugin::new("examples/assets/icon.png")
        ))
        .run();
}