use std::time::Duration;

use std::fmt::Write;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
pub struct FpsPlugin;

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::RED;
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

const STRING_FORMAT: &str = "FPS: ";
const STRING_INITIAL: &str = "FPS: ...";
const STRING_MISSING: &str = "FPS: ???";

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(spawn_text)
            .add_system(update)
            .init_resource::<FpsState>();
    }
}

#[derive(Resource)]
pub struct FpsState {
    pub timer: Timer,
    pub update_now: bool,
}

impl Default for FpsState {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating),
            update_now: true,
        }
    }
}

impl FpsState {
    pub fn enable(&mut self) {
        self.timer.unpause();
        self.update_now = true;
    }

    pub fn disable(&mut self) {
        self.timer.pause();
        self.update_now = true;
    }

    pub fn enabled(&self) -> bool {
        !self.timer.paused()
    }
}

fn update(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    state_resource: Option<ResMut<FpsState>>,
    mut text_query: Query<&mut Text, With<FpsText>>,
) {
    if let Some(mut state) = state_resource {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.paused() {
                // Time is paused so remove text
                for mut text in text_query.iter_mut() {
                    let value = &mut text.sections[0].value;
                    value.clear();
                }
            } else {
                let fps_diags = extract_fps(&diagnostics);

                for mut text in text_query.iter_mut() {
                    let value = &mut text.sections[0].value;
                    value.clear();

                    if let Some(fps) = fps_diags {
                        write!(value, "{}{:.0}", STRING_FORMAT, fps).unwrap();
                    } else {
                        value.clear();
                        write!(value, "{}", STRING_MISSING).unwrap();
                    }
                }
            }
        }
    }
}

#[derive(Component)]
pub struct FpsText;

fn extract_fps(diagnostics: &Res<Diagnostics>) -> Option<f64> {
    diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font/arcade-classic.ttf");
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: STRING_INITIAL.to_string(),
                    style: TextStyle {
                        font,
                        font_size: FONT_SIZE,
                        color: FONT_COLOR,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        },
        FpsText,
    ));
}
