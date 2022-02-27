use bevy::prelude::*;
use rand::{Rng, thread_rng};
use std::time::{Duration, Instant};
use bevy::core::FixedTimestep;
use bevy::sprite::collide_aabb::collide;

use crate::{Health, SpriteSheets, Velocity, WindowBounds};
use crate::spells::SpellEffect;

const ENEMY_RENDER_PRIORITY:f32 = 1.1; // Slightly higher than player.

// Public Access:
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup_enemy);
		app.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(1.0))
				.with_system(spawn_enemy)
		);
		app.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(3.0))
				.with_system(complete_wave)
		);
		app.add_system(apply_spell_effects);
	}
}

// Resources:
struct Wave(u32);

struct ActiveEnemiesInWave(u32); // We make this a separate trait so we can lock it independently.

struct PendingEnemiesInWave(u32);

// Components:
#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemyMoveTarget(Vec2);

// Systems:
fn setup_enemy(
	mut commands: Commands,
) {
	commands.insert_resource(Wave(0));
	commands.insert_resource(ActiveEnemiesInWave(0));
	commands.insert_resource(PendingEnemiesInWave(1));
}

fn spawn_enemy(
	mut commands: Commands,
	mut pending_enemies: ResMut<PendingEnemiesInWave>,
	mut active_enemies: ResMut<ActiveEnemiesInWave>,
	sprite_sheets: Res<SpriteSheets>,
	atlas_assets: Res<Assets<TextureAtlas>>,
	window: Res<WindowBounds>,
) {
	if pending_enemies.0 > 0 {
		let mut rng = thread_rng();
		//let x = rng.gen::<f32>() * 10f32;
		//let y = rng.gen::<f32>() * 10f32;
		let x = rng.gen_range::<f32>(window.left, window.right);
		let y = rng.gen_range(window.bottom, window.top);

		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: atlas_assets.get_handle(&sprite_sheets.enemy_material),
				//transform: Transform::from_scale(Vec3::splat(6.0)),
				transform: Transform {
					translation: Vec3::new(x, y, ENEMY_RENDER_PRIORITY),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Timer::from_seconds(0.1, true))
			.insert(Health(10.0f32))
			.insert(Velocity(Vec3::new(rng.next_f32() - 0.5f32, rng.next_f32() - 0.5f32, 0f32)));
		pending_enemies.0 -= 1;
		active_enemies.0 += 1;
	}
}

fn complete_wave(
	mut wave: ResMut<Wave>,
	mut pending_enemies: ResMut<PendingEnemiesInWave>,
	active_enemies: Res<ActiveEnemiesInWave>,
) {
	// This does nothing but bump our wave count and reset the number of enemies.
	if pending_enemies.0 == 0 && active_enemies.0 == 0 {
		println!("Pending enemies: {}  Active enemies: {}", &pending_enemies.0, &active_enemies.0);
		wave.0 += 1;
		pending_enemies.0 = (1+wave.0)*2;
		println!("Starting wave {}.", &wave.0);
	}
}

fn apply_spell_effects(
	mut enemy_query: Query<(&Transform, &mut Health, With<Enemy>)>,
	spell_query: Query<(&Transform, &SpellEffect)>,
) {
	// We should consider adding 'sprite' to this fray so we can compare the sizes.
	for (enemy_transform, mut health, _) in enemy_query.iter_mut() {
		for (spell_transform, spell_effect) in spell_query.iter() {
			let enemy_size = Vec2::new(enemy_transform.scale.x, enemy_transform.scale.y);
			let spell_size = Vec2::new(spell_transform.scale.x, spell_transform.scale.y);
			let collision = collide(
				enemy_transform.translation,
				enemy_size,
				spell_transform.translation,
				spell_size
			);
			if let Some(_) = collision {
				// TODO: We should match the type of the collision to the enemy resistance even before we do this.
				health.0 -= spell_effect.base_damage;
			}
		}
	}
}

// Maybe we should do this when we apply damage?  That's the only time it can happen, right?
// Or we can make this global and do death counts for everything.
fn count_and_remove_dead_enemies(
	mut commands: Commands,
	mut active_enemes: ResMut<ActiveEnemiesInWave>,
	query: Query<(Entity, &Health, With<Enemy>)>,
) {
	for (entity, health, _) in query.iter() {
		if health.0 <= 0.0 {
			commands.entity(entity).remove::<Enemy>();
			active_enemes.0 -= 1;
		}
	}
}