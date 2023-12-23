use bevy::{prelude::*, winit::WinitWindows};
use winit::window::Icon;
///A bevy window icon plugin. Must be included for the icon to show up.
pub struct WindowIconPlugin{
    icon_path:String,
}
impl WindowIconPlugin{
    //Specify the icon path and that's all.
    pub fn new(icon_path:&str) -> Self{
        Self { icon_path: icon_path.to_string() }
    }
}
impl Plugin for WindowIconPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowIconPath(self.icon_path.clone()))
            .add_systems(Startup, set_window_icon);
    }
}
#[derive(Resource)]
struct WindowIconPath(String);
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    window_icon_path:ResMut<WindowIconPath>,
    mut commands:Commands,
){
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(&window_icon_path.0)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
    commands.remove_resource::<WindowIconPath>();
}