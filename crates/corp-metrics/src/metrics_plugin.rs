pub mod metrics {
    use bevy::app::{AppBuilder, Plugin};
    use bevy::asset::AssetServer;
    use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
    use bevy::ecs::{Commands, IntoQuerySystem, Query, Res};
    use bevy::math::Rect;
    use bevy::render::color::Color;
    use bevy::text::TextStyle;
    use bevy::ui::entity::{TextComponents, UiCameraComponents};
    use bevy::ui::widget::Text;
    use bevy::ui::{AlignSelf, PositionType, Style, Val};

    pub struct MetricsPlugin;

    impl Plugin for MetricsPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_startup_system(setup.system())
                .add_system(text_update_system.system());
        }
    }

    fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
        for mut text in &mut query.iter() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.value = format!("FPS: {:.0}", average);
                }
            }
        }
    }

    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(UiCameraComponents::default())
            .spawn(TextComponents {
                style: fps_style(),
                text: fps_text(asset_server),
                ..Default::default()
            });
    }

    fn fps_text(asset_server: Res<AssetServer>) -> Text {
        let font_handle = asset_server
            .load("assets/fonts/Kenney Future Narrow.ttf")
            .unwrap();
        Text {
            value: "FPS".to_string(),
            font: font_handle,
            style: TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
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
