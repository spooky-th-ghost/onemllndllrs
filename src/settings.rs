use bevy::prelude::*;

pub struct UserSettingsPlugin;

impl Plugin for UserSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplaySettings::default())
            .add_startup_system(configure_window);
    }
}

#[derive(Resource)]
pub struct DisplaySettings {
    pub resolution: (f32, f32),
    pub cursor_mode: bevy::window::CursorGrabMode,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        DisplaySettings {
            resolution: (1600.0, 900.0),
            cursor_mode: bevy::window::CursorGrabMode::Locked,
        }
    }
}

fn configure_window(
    display_settings: Res<DisplaySettings>,
    mut query: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    let Ok(mut primary) = query.get_single_mut() else {
        return;
    };

    let resolution = display_settings.resolution;
    let cursor_mode = display_settings.cursor_mode;

    primary.title = "1 M L L N D L L R S".to_string();
    // primary.position = WindowPosition::Centered(MonitorSelection::Current);
    // primary.resizable = false;
    // primary.resolution = bevy::window::WindowResolution::new(resolution.0, resolution.1);
    // primary.cursor.grab_mode = cursor_mode;
}
