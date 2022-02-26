mod input;

use std::borrow::BorrowMut;
use bevy::prelude::*;
use enemy::*;
use input::{input_event_system, touch_system, mouse_click_system};
use std::time::Duration;
use rand::{Rand, Rng};

const BACKGROUND_RENDER_PRIORITY:f32 = 0.0;
const PLAYER_RENDER_PRIORITY:f32 = 1.0; // Higher = on top.

// Maybe add https://github.com/Trouv/bevy_ecs_ldtk
// https://github.com/PhaestusFox/bevy_sprite_animation

// Resources
struct SpriteSheets {
	//enemy_material: Handle<ColorMaterial>,
	//tilsets: Handle<TextureAtlas>,
	enemy_material: Handle<TextureAtlas>,
}

#[derive(Default)]
struct WorldState {
	enemy_move_target: Vec2,
	last_spawn_timeago: Duration,
	wave_number: u64,
	unspawned_in_wave: u16,
	alive_in_wave: u16,
}
// END Resources

// Components:
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Velocity(Vec3);
// END Components

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(ClearColor(Color::BLACK))
		.insert_resource(WindowDescriptor {
			width: 192.0,
			height: 108.0,
			title: "".to_string(),
			resizable: true,
			decorations: false,
			cursor_visible: true,
			cursor_locked: false,
			..Default::default()
		})
		.add_startup_system(setup)

		// Rendering
		.add_system(animate_sprite_system)
		// Movement
		.add_system(movement)
		// Inputs:
		.add_system(input_event_system)
		.add_system(touch_system)
		.add_system(mouse_click_system)
		// Gameplay
		.add_system(spawn_enemy)

		.run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut atlas_assets: ResMut<Assets<TextureAtlas>>,
	mut windows: ResMut<Windows>,
) {
	// Get our window bounds for aspect ratio.
	let window = windows.get_primary().unwrap();
	let width = window.width();
	let height = window.height();

	// Spawn the cameras
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.orthographic_projection.scale /= 10.0;  // Make everything 10x bigger.
	//camera.camera.far = 10.0;
	commands.spawn_bundle(camera);
	commands.spawn_bundle(UiCameraBundle::default());

	// Need some RNS?
	commands.insert_resource(rand::StdRng::new().unwrap());

	// Setup our world.
	commands.insert_resource(WorldState {
		enemy_move_target: Default::default(),
		last_spawn_timeago: Duration::from_secs_f32(0.0f32),
		wave_number: 0,
		unspawned_in_wave: 10,
		alive_in_wave: 0
	});

	// Load enemy sprite atlas:
	let enemy_texture_handle = asset_server.load("enemy_1x4.png");
	let enemy_texture_atlas = TextureAtlas::from_grid(enemy_texture_handle, Vec2::new(16.0, 16.0), 4, 1);
	let enemy_texture_atlas_handle = atlas_assets.add(enemy_texture_atlas);
	commands.insert_resource(SpriteSheets {
		enemy_material: enemy_texture_atlas_handle,
	});
}

fn spawn_enemy(
	mut commands: Commands,
	timer: Res<Time>,
	mut rng: ResMut<rand::StdRng>,
	sprite_sheets: Res<SpriteSheets>,
	atlas_assets: Res<Assets<TextureAtlas>>,
	mut world_state: ResMut<WorldState>,
) {
	world_state.last_spawn_timeago += timer.delta();

	if world_state.last_spawn_timeago > Duration::from_secs_f32(1.0/(1.0+world_state.wave_number as f32)) {
		world_state.last_spawn_timeago = Duration::from_secs(0);
		world_state.unspawned_in_wave -= 1;

		if world_state.unspawned_in_wave == 0 && world_state.alive_in_wave == 0 {
			world_state.wave_number += 1;
			world_state.unspawned_in_wave = ((1 + world_state.wave_number) * 2).min(1000) as u16;
		}

		let x = rng.gen::<f32>() * 10f32;
		let y = rng.gen::<f32>() * 10f32;

		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: atlas_assets.get_handle(&sprite_sheets.enemy_material),
				//transform: Transform::from_scale(Vec3::splat(6.0)),
				transform: Transform {
					translation: Vec3::new(x, y, PLAYER_RENDER_PRIORITY),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Timer::from_seconds(0.1, true));
	}
}

fn animate_sprite_system(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
	for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
		}
	}
}

struct GreetTimer(Timer);
//app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))  // True means repeat.
//fn greet_enemies(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Transform, With<Enemy>>) {

//fn move_enemies(time: Res<Time>, query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
fn movement(
	time: Res<Time>,
	mut query: Query<(&mut Transform, &Velocity)>
) {
	let dt = time.delta();
	for (mut tf, velocity) in query.iter_mut() {
		tf.translation += velocity.0 * dt.as_secs_f32();
	}
}


/*
// Figuring out sprite sheets and Assets vs Handles.
struct SpriteSheets {
	map_tiles: Handle<TextureAtlas>,
}

fn use_sprites(
	handles: Res<SpriteSheets>,
	atlases: Res<Assets<TextureAtlas>>,
	images: Res<Assets<Image>>,
) {
	// Could be `None` if the asset isn't loaded yet
	if let Some(atlas) = atlases.get(&handles.map_tiles) {
		// do something with the texture atlas
	}

	// Can use a path instead of a handle
	if let Some(map_tex) = images.get("map.png") {
		// if "map.png" was loaded, we can use it!
	}
}
*/