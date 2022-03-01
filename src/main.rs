mod enemy;
mod input;
mod level;
mod player;
mod spells;

use bevy::prelude::*;
use enemy::*;
use input::{input_event_system, touch_system, mouse_click_system};
use std::time::Duration;
use bevy::render::view::VisibleEntities;
use rand::{Rand, Rng, thread_rng};

const WINDOW_SCALE:f32 = 1.0/2.0;
const BACKGROUND_RENDER_PRIORITY:f32 = 0.0;
const PLAYER_RENDER_PRIORITY:f32 = 1.0; // Higher = on top.
const ENEMY_RENDER_PRIORITY:f32 = 1.1; // Slightly higher than player.
const CAMERA_SHAKE_LERP_FACTOR:f32 = 0.1;

// Maybe add https://github.com/Trouv/bevy_ecs_ldtk
// https://github.com/PhaestusFox/bevy_sprite_animation

// Resources
struct SpriteSheets {
	//enemy_material: Handle<ColorMaterial>,
	level_tileset: Handle<TextureAtlas>,
	player_material: Handle<TextureAtlas>,
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
struct DestroyOnOOB; // If assigned to an entity, will get deleted when it moves off camera.

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct GameplayCamera; // Attached to our primary orthographic camera, NOT our UI camera.
// END Components

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
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

		// Technically startup systems, but should happen after startup.
		.add_plugin(level::LevelPlugin)
		.add_plugin(player::PlayerPlugin)
		.add_plugin(enemy::EnemyPlugin)
		.add_plugin(spells::SpellPlugin)

		// Rendering
		.add_system(animate_sprite_system)
		.add_system(clean_oob_components)
		.add_system(apply_screen_shake)
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
	commands.spawn_bundle(camera).insert(GameplayCamera);
	commands.spawn_bundle(UiCameraBundle::default());
	commands.insert_resource(ScreenShake {
		magnitude: 0.0,
		decay: 1.5,  // Larger -> Faster return to static.
		target_offset: Vec2::new(0.0,0.0),
		target_rotation: 0.0
	});

	// Need some RNG?
	// use thread_rng instead of commands.insert_resource(rand::StdRng::new().unwrap());

	// Build Sprite Sheet:
	let level_tileset_handle = atlas_assets.add(TextureAtlas::from_grid(asset_server.load("spritesheet_1x7.png"), Vec2::new(16.0, 16.0), 7, 1));

	let player_texture_atlas_handle = atlas_assets.add(TextureAtlas::from_grid(asset_server.load("player_1x10.png"), Vec2::new(16.0, 16.0), 10, 1));

	let enemy_texture_handle = asset_server.load("enemy_1x4.png");
	let enemy_texture_atlas = TextureAtlas::from_grid(enemy_texture_handle, Vec2::new(16.0, 16.0), 4, 1);
	let enemy_texture_atlas_handle = atlas_assets.add(enemy_texture_atlas);

	let explosion_texture_atlas_handle = atlas_assets.add(TextureAtlas::from_grid(asset_server.load("explosion_1x6.png"), Vec2::new(16.0, 16.0), 6, 1));

	let magic_missile_texture_atlas_handle = atlas_assets.add(TextureAtlas::from_grid(asset_server.load("magic_missile_head.png"), Vec2::new(16.0, 16.0), 3, 1));

	commands.insert_resource(SpriteSheets {
		level_tileset: level_tileset_handle,
		player_material: player_texture_atlas_handle,
		enemy_material: enemy_texture_atlas_handle,
		explosion: explosion_texture_atlas_handle,
		magic_missile: magic_missile_texture_atlas_handle,
	});
}

fn apply_screen_shake(
	mut screen_shake: ResMut<ScreenShake>,
	mut camera_query: Query<(&mut Transform, With<GameplayCamera>)>,
) {
	// Stupid shit hacky camera shake.
	/*
	Rather than use the right thing: real perlin noise or the profoundly stupid thing: randint,
	we're going to split the difference and pick a random point, LERP to it, and select a new random point as a function of the distance.
	If we're on top of the point, our likelihood of keeping the point d(cam, target) is 0.
	If we're a long ways away, our likelihood of keeping the target is 1.0/(1.0+x), which goes to 1.
	The new random point is selected at a distance of log(shake_intensity), which could be problematic because we have no harmonics to make fine jitters.
	*/
	let (mut camera_transform, _) = camera_query.single_mut();
	// If there is no screen shake active, return early.
	if screen_shake.magnitude < 1e-6 {
		screen_shake.magnitude = 0.0;
		return;
	}

	// Camera is Vec3.  Our target is Vec2.  We don't want to mess with the depth of the orthographic camera because everything could get thrown off and now show up.
	// For efficiency, pull out DX/DY.
	let camera_dx = screen_shake.target_offset.x - camera_transform.translation.x;
	let camera_dy = screen_shake.target_offset.y - camera_transform.translation.y;
	// Move closer to new point.  Lazily split x/y rather than cast to Vec2.
	camera_transform.translation.x = camera_transform.translation.x + (camera_dx*(1.0-CAMERA_SHAKE_LERP_FACTOR));
	camera_transform.translation.y = camera_transform.translation.y + (camera_dy*(1.0-CAMERA_SHAKE_LERP_FACTOR));
	// Now maybe move to a new place.
	let distance_squared = camera_dx*camera_dx + camera_dy*camera_dy;
	let keep_target_probability = distance_squared / (1.0 + distance_squared);
	let mut rng = thread_rng();
	if rng.next_f32() > keep_target_probability {
		// Need a new target.
		let log_intensity = screen_shake.magnitude.log2().max(0.0);
		let new_x = 2.0*(rng.next_f32()-0.5) * log_intensity;
		let new_y = 2.0*(rng.next_f32()-0.5) * log_intensity;
		screen_shake.target_offset = Vec2::new(new_x, new_y);
	}
	// Decay
	screen_shake.magnitude /= screen_shake.decay;
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
	mut commands: Commands,
	window_bounds: Res<WindowBounds>, // Eventually we should use the VisibleEntities on the camera.
	mut entity_query: Query<(Entity, &Transform, With<DestroyOnOOB>)>,
) {
	// TODO: Swap this window-based hack with one that uses the real properties, like the visible detector in Camera.
	for (entity, tf, _) in entity_query.iter_mut() {
		let in_bounds = tf.translation.x > window_bounds.left && tf.translation.x < window_bounds.right && tf.translation.y > window_bounds.bottom && tf.translation.y < window_bounds.top;
		if !in_bounds {
			commands.entity(entity).despawn();
		}
	}
}

//struct GreetTimer(Timer);
//app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))  // True means repeat.
//fn greet_enemies(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Transform, With<Enemy>>) {


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