mod enemy;
mod input;
mod spells;

use std::borrow::BorrowMut;
use bevy::prelude::*;
use enemy::*;
use input::{input_event_system, touch_system, mouse_click_system};
use std::time::Duration;
use rand::{Rand, Rng};

const WINDOW_SCALE:f32 = 1.0/5.0;
const BACKGROUND_RENDER_PRIORITY:f32 = 0.0;
const PLAYER_RENDER_PRIORITY:f32 = 1.0; // Higher = on top.

// Maybe add https://github.com/Trouv/bevy_ecs_ldtk
// https://github.com/PhaestusFox/bevy_sprite_animation

// Resources
struct SpriteSheets {
	//enemy_material: Handle<ColorMaterial>,
	//tilsets: Handle<TextureAtlas>,
	enemy_material: Handle<TextureAtlas>,
	explosion: Handle<TextureAtlas>,
	magic_missile: Handle<TextureAtlas>,
}

#[derive(Default)]
struct ScreenShake {
	magnitude: f32,
	decay: f32,
	target_offset: Vec2,
	target_rotation: f32,
}

struct WindowBounds {
	left: f32,
	right: f32,
	top: f32,
	bottom: f32,
	// origin: Vec2,
	width: f32,
	height: f32
}
// END Resources

// Components:
#[derive(Component)]
struct Player;

#[derive(Component)]
struct DestroyOnOOB; // If assigned to an entity, will get deleted when it moves off camera.

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Velocity(Vec3);
// END Components

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(enemy::EnemyPlugin)
		.insert_resource(ClearColor(Color::BLACK))
		.insert_resource(WindowDescriptor {
			width: 1920.0,
			height: 1080.0,
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
		// Yeet
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
	commands.insert_resource(WindowBounds {
		left: -width/2.0 * WINDOW_SCALE,
		right: width/2.0 * WINDOW_SCALE,
		top: height/2.0 * WINDOW_SCALE,
		bottom: -height/2.0 * WINDOW_SCALE,
		width: width * WINDOW_SCALE,
		height: height * WINDOW_SCALE
	});

	// Spawn the cameras
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.orthographic_projection.scale = WINDOW_SCALE;
	//camera.camera.far = 10.0;
	commands.spawn_bundle(camera);
	commands.spawn_bundle(UiCameraBundle::default());
	commands.insert_resource(ScreenShake::default());

	// Need some RNG?
	// use thread_rng instead of commands.insert_resource(rand::StdRng::new().unwrap());

	// Build Sprite Sheet:
	let enemy_texture_handle = asset_server.load("enemy_1x4.png");
	let enemy_texture_atlas = TextureAtlas::from_grid(enemy_texture_handle, Vec2::new(16.0, 16.0), 4, 1);
	let enemy_texture_atlas_handle = atlas_assets.add(enemy_texture_atlas);

	let explosion_texture_atlas_handle = atlas_assets.add(TextureAtlas::from_grid(asset_server.load("explosion_1x6.png"), Vec2::new(16.0, 16.0), 6, 1));

	let magic_missile_texture_atlas_handle = atlas_assets.add(TextureAtlas::from_grid(asset_server.load("magic_missile_head.png"), Vec2::new(16.0, 16.0), 3, 1));

	commands.insert_resource(SpriteSheets {
		enemy_material: enemy_texture_atlas_handle,
		explosion: explosion_texture_atlas_handle,
		magic_missile: magic_missile_texture_atlas_handle,
	});
}

fn apply_screen_shake(
	time: Res<Time>,
	mut screen_shake: ResMut<ScreenShake>,
) {

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

fn movement(
	time: Res<Time>,
	mut query: Query<(&mut Transform, &Velocity)>
) {
	let dt = time.delta();
	for (mut tf, velocity) in query.iter_mut() {
		tf.translation += velocity.0 * dt.as_secs_f32();
	}
}

fn clean_oob_components(
	camera: Res<Camera>,
	mut query: Query<(Entity, &Transform, With<DestroyOnOOB>)>,
)

//struct GreetTimer(Timer);
//app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))  // True means repeat.
//fn greet_enemies(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Transform, With<Enemy>>) {

//fn move_enemies(time: Res<Time>, query: Query<(&mut Transform, &Velocity), With<Enemy>>) {



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