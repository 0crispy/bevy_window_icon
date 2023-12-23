# `bevy_window_icon`
[![Crates.io](https://img.shields.io/crates/v/bevy_window_icon)](https://crates.io/crates/bevy_window_icon)
A very simple crate to set the window icon.
## How to use
```rust
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
```
That's literally it.
## Bevy Compatibility
|bevy|bevy_easy_localize|
|---|---|
|0.11|0.1|