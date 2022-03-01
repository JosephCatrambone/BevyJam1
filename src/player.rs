use std::borrow::Borrow;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::{Health, PLAYER_RENDER_PRIORITY, SpriteSheets, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		//app.add_startup_system(player_startup);
		app.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(1.0))
				.with_system(respawn_player)
		);
		app.add_system(check_for_player_death);
	}
}

#[derive(Component)]
pub struct Player;

fn respawn_player(
	mut commands: Commands,
	atlas_assets: Res<Assets<TextureAtlas>>,
	sprite_sheets: Res<SpriteSheets>,
	//time: Res<Time>,
	player_query: Query<With<Player>>,
) {
	if let Some(_) = player_query.iter().next() {
		return; // Nothing to do.
	}

	//let player_texture_atlas = atlas_assets.get(&sprite_sheets.player_material).expect("Player texture is not loaded.");
	//let num_faces = player_texture_atlas.len();
	let num_faces = 10;

	let mut sb = SpriteSheetBundle {
		texture_atlas: atlas_assets.get_handle(&sprite_sheets.player_material),
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, PLAYER_RENDER_PRIORITY),
			..Default::default()
		},
		..Default::default()
	};

	// Randomly assign a player face:
	let mut rng = thread_rng();
	sb.sprite.index = rng.gen_range(0, num_faces);

	// Spawn!
	commands
		.spawn_bundle(sb)
		.insert(Health(10.0f32))
		.insert(Player);
}

fn check_for_player_death(
	mut commands: Commands,
	query: Query<(Entity, &Health, With<Player>)>,
) {
	//let (entity, player_health, _) = query.single();
	if let Some((entity, player_health, _)) = query.iter().next() {
		if player_health.0 <= 0.0 {
			// Player is dead.  :'(
			commands.entity(entity).despawn();
		}
	}
}