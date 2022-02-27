use std::borrow::Borrow;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::{Health, PLAYER_RENDER_PRIORITY, SpriteSheets, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(player_startup);
		app.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(1.0))
				.with_system(respawn_player)
		);
	}
}

pub struct PlayerState {
	pub alive: bool,
	time_since_death: f32,
	pub position: Vec3,  // We copy from the player's transform so enemies can move towards this.
}

fn player_startup(
	mut commands: Commands,
) {
	commands.insert_resource(PlayerState {
		alive: false,
		time_since_death: 0.0f32,
		position: Vec3::default(),
	});
}

fn respawn_player(
	mut commands: Commands,
	atlas_assets: Res<Assets<TextureAtlas>>,
	sprite_sheets: Res<SpriteSheets>,
	//time: Res<Time>,
	mut player_state: ResMut<PlayerState>,
) {
	if player_state.alive {
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
		.insert(Health(10.0f32));

	player_state.alive = true;
}

fn check_for_player_death() {}