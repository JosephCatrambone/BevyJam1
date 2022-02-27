use bevy::prelude::*;

use crate::Velocity;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
	fn build(&self, app: &mut App) {

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
	origin: Vec2,
	direction: Vec2,
) {
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: atlas_assets.get_handle(&sprite_sheets.magic_missile_head),
			//transform: Transform::from_scale(Vec3::splat(6.0)),
			transform: Transform {
				translation: Vec3::new(x, y, ENEMY_RENDER_PRIORITY),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Timer::from_seconds(0.1, true))
		.insert(Velocity(Vec3::new(rng.next_f32() - 0.5f32, rng.next_f32() - 0.5f32, 0f32)));
}