use bevy::prelude::*;

pub struct TextDisplayPlugin;

impl Plugin for TextDisplayPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(spawn_ui_text);
		app.add_system(update_ui_text);
	}
}

#[derive(Component, Clone)]
pub struct UIText {
	text: String,
	color: Color,
	size: f32,
	fade_time_in_seconds: f32,
}

impl Default for UIText {
	fn default() -> Self {
		UIText {
			text: String::new(),
			color: Color::WHITE,
			size: 10.0f32,
			fade_time_in_seconds: 1.0f32,
		}
	}
}

impl UIText {
	pub fn new(txt: String, color: Color, size: f32, fade_time_in_seconds: f32) -> Self {
		UIText {
			text: txt,
			color,
			size,
			fade_time_in_seconds
		}
	}

	pub fn from_string(txt: String) -> Self {
		UIText {
			text: txt,
			..Default::default()
		}
	}
}

fn spawn_ui_text(
	mut commands: Commands,
	mut asset_server: ResMut<AssetServer>,
	unspawned_text: Query<(Entity, &UIText, Without<Style>)>,
) {
	// This is slightly inefficient because we're removing text and adding it again instead of just adding the component, but...
	for (entity, txt, _) in unspawned_text.iter() {
		commands.spawn_bundle(
			TextBundle {
				style: Style {
					align_self: AlignSelf::Center,
					position_type: PositionType::Absolute,
					position: Rect {
						left: Val::Percent(10.0),
						top: Val::Percent(100.0),
						//top: Val::Auto,
						//bottom: Val::Px(-50.0),
						right: Val::Auto,
						bottom: Val::Auto,
					},
					..Default::default()
				},
				// Use the `Text::with_section` constructor
				text: Text::with_section(
					txt.text.clone(),
					TextStyle {
						font: asset_server.load("OpenSans-Regular.ttf"),
						font_size: 100.0,
						color: Color::WHITE,
					},
					// Note: You can use `Default::default()` in place of the `TextAlignment`
					TextAlignment {
						horizontal: HorizontalAlign::Center,
						vertical: VerticalAlign::Center,
						//..Default::default()
					},
				),
				..Default::default()
			}
		).insert(txt.clone());
		// Unspawn the old entry:
		commands.entity(entity).despawn();
	}
}

// This is hacky and bad.  Replace it.
fn update_ui_text(
	mut commands: Commands,
	mut text: Query<(Entity, &UIText, &mut Style)>,
) {
	for (entity, ui_text, mut style) in text.iter_mut() {
		let dist_to_top = if let Val::Percent(p) = style.position.top {
			p - 50.0
		} else {
			0.0 // This can't happen.
		};
		style.position.top += -dist_to_top*0.05;
		//style.position.bottom += dist_to_top*0.1;

		if dist_to_top < 1.5e-3 {
			// Start to fade away.
			commands.entity(entity).despawn();
			info!("Despawning text.");
		}
	}
}

/*
 commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
*/