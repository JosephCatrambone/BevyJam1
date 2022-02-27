use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::{DestroyOnOOB, ENEMY_RENDER_PRIORITY, SpriteSheets, Velocity};
use crate::player::PlayerState;

const MAGIC_MISSILE_SPEED:f32 = 100.0f32;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(cast_magic_missile);
	}
}

#[derive(Component)]
pub struct SpellEffect {
	pub base_damage: f32,
	pub element_type: usize, // Swap this with enum.
}

// We could make this system listen for button inputs OR we could define a function that spawns the spell.
fn cast_magic_missile(
	mut commands: Commands,
	windows: Res<Windows>,
	mouse_button_input: Res<Input<MouseButton>>,
	atlas_assets: Res<Assets<TextureAtlas>>,
	sprite_sheets: Res<SpriteSheets>,
	player_state: Res<PlayerState>, // Used to give us direction for the attack.
) {
	if mouse_button_input.just_pressed(MouseButton::Left) {
		let window = windows.get_primary().expect("Could not acquire primary game window.");
		if let Some(mouse_position) = window.cursor_position() {
			// Mouse Position is in Window space.
			let delta = Vec3::new(((mouse_position.x/window.width())-0.5) - player_state.position.x, ((mouse_position.y/window.height())-0.5) - player_state.position.y, 0.0).normalize() * MAGIC_MISSILE_SPEED;

			commands
				.spawn_bundle(SpriteSheetBundle {
					texture_atlas: atlas_assets.get_handle(&sprite_sheets.magic_missile),
					transform: Transform {translation: player_state.position, ..Default::default()
					},
					..Default::default()
				})
				.insert(DestroyOnOOB)
				.insert(Timer::from_seconds(0.1, true))
				.insert(Velocity(delta))
				.insert(SpellEffect {
					base_damage: 1.0f32,
					element_type: 0,
				});
		}
	}

}