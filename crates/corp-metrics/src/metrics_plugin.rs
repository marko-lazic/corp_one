pub mod metrics {
    use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
    use bevy::prelude::*;

    pub struct MetricsPlugin;

    struct FpsText;

    impl Plugin for MetricsPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_startup_system(setup.system())
                .add_system(text_update_system.system());
        }
    }

    fn text_update_system(
        diagnostics: Res<Diagnostics>,
        mut query: Query<&mut Text, With<FpsText>>,
    ) {
        for mut text in query.iter_mut() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.value = format!("FPS: {:.2}", average);
                }
            }
        }
    }

    fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(CameraUiBundle::default())
            .spawn(TextBundle {
                style: fps_style(),
                text: fps_text(asset_server),
                ..Default::default()
            })
            .with(FpsText);
    }

    fn fps_text(asset_server: Res<AssetServer>) -> Text {
        let font_handle = asset_server.load("fonts/Kenney Future Narrow.ttf");
        Text {
            value: "FPS".to_string(),
            font: font_handle,
            style: TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                alignment: Default::default(),
            },
        }
    }

    fn fps_style() -> Style {
        Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        }
    }
}
