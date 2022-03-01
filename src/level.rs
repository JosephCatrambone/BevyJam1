use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::{BACKGROUND_RENDER_PRIORITY, SpriteSheets};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(initialize_level_plugin);
		app.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(1.0))
				.with_system(regenerate_level)
		);
	}
}

// Singleton resource.
struct Level {
	needs_regeneration: bool,
	width: usize,  // In tiles.
	height: usize,  // In tiles.
	tile_width: usize,
	tile_height: usize,
}

#[derive(Component)]
struct Tile(u32);

fn initialize_level_plugin(
	mut commands: Commands,
) {
	commands.insert_resource(Level {
		needs_regeneration: true,
		width: 50,
		height: 50,
		tile_width: 16,
		tile_height: 16,
	})
}

fn regenerate_level(
	mut commands: Commands,
	mut level: ResMut<Level>,
	mut atlas_assets: ResMut<Assets<TextureAtlas>>,
	sprite_sheet: Res<SpriteSheets>,
	tiles: Query<(Entity, With<Tile>)>,
) {
	if !level.needs_regeneration {
		return;
	}

	// First, clear all the tiles.
	for (entity, _) in tiles.iter() {
		commands.entity(entity).despawn();
	}

	let mut rng = thread_rng();

	// Spawn new tiles with 4-way symmetry.
	for y in 0i32..(level.height/2) as i32 {
		for x in 0i32..(level.width/2) as i32 {
			let mut spawn_cmd = |x_pos, y_pos, tile_id| {
				let mut bundle = SpriteSheetBundle {
					texture_atlas: atlas_assets.get_handle(&sprite_sheet.level_tileset),
					transform: Transform {
						translation: Vec3::new((x_pos as f32) * level.tile_width as f32, (y_pos as f32) * level.tile_height as f32, BACKGROUND_RENDER_PRIORITY),
						..Default::default()
					},
					..Default::default()
				};
				bundle.sprite.index = tile_id;
				commands.spawn_bundle(bundle)
				.insert(Tile(0));
			};
			let tile_id = 1+(rng.next_u32()%6) as usize;
			spawn_cmd(x, y, tile_id);
			spawn_cmd(-x, y, tile_id);
			spawn_cmd(x, -y, tile_id);
			spawn_cmd(-x, -y, tile_id);
		}
	}

	level.needs_regeneration = false;
}