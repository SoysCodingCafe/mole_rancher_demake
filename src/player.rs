use std::f32::consts::PI;
use std::time::Duration;

use bevy::{
	prelude::*, 
	window::PrimaryWindow, 
	input::mouse::MouseButtonInput, 
};
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};
use crate::loading::{AudioAssets, TextureAssets};
use crate::menu::DeathFadeout;
use crate::molecules::{BulletInfo, Crosses, MoleculeInfo, Reactor, Score};
use crate::GameState;

#[derive(Component)]
pub struct PlayerInfo {
	pub lives: f32,
	pub death_countdown: f32,
	pub time_survived: f32,
	pub score: f32,
	pub vel: Vec2,
	pub acc: f32,
	pub max_vel: f32,
	pub radius: f32,
	pub stun_duration: f32,
	pub invul_duration: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Playing), spawn_player)
			.add_systems(Update, (
				weapon_swing,
				player_movement,
				execute_animations,
				check_player_lives,
			).chain().run_if(in_state(GameState::Playing)))
			.add_systems(OnExit(GameState::Playing), cleanup_game)
		;
	}
}

#[derive(Component)]
pub struct WeaponPivot {
	time_left: f32,
	max_time: f32,
	backswing: f32,
	held: bool,
	swinging: bool,
	pub active: bool,
	clockwise_swing: bool,
}

#[derive(Component)]
pub struct WeaponCollider;

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    _fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, _fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            _fps,
            frame_timer: Self::timer_from_fps(_fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

fn execute_animations(
	time: Res<Time>,
	player_query: Query<&PlayerInfo>,
	mut animation_query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
	let player = player_query.single().expect("Could not find player");
	for (mut config, mut sprite) in &mut animation_query {
		config.frame_timer.tick(time.delta());
		if config.frame_timer.just_finished() {
			if let Some(atlas) = &mut sprite.texture_atlas {
				if atlas.index == config.last_sprite_index || player.vel == Vec2::ZERO {
					atlas.index = config.first_sprite_index;
				} else {
					atlas.index += 1;
				}
			}
		}
		if player.invul_duration > 0.0 {
			let flicker = ((player.invul_duration * 2.0 * 2.0 * PI - PI/2.0).sin() + 1.0)/2.0;
			sprite.color = Color::linear_rgb(1.0, 1.0 - flicker, 1.0 - flicker);
		} else {
			sprite.color = Color::WHITE;
		};
	}
}

fn spawn_player(
	mut commands: Commands,
	textures: Res<TextureAssets>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 12, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
	let animation_config = AnimationConfig::new(0, 11, 16);

	let radius = 24.0;
	commands.spawn((
		Sprite {
				image: textures.rodney.clone(),
				custom_size: Some(Vec2::splat(radius * 2.0)),
				texture_atlas: Some(TextureAtlas {
					layout: texture_atlas_layout.clone(),
					index: animation_config.first_sprite_index,
				}),
				..default()
		},
		animation_config,
		Transform::from_xyz(0.0, 220.0, 100.0),
		PlayerInfo {
			lives: 3.0,
			death_countdown: 0.0,
			time_survived: 0.0,
			score: 0.0,
			vel: Vec2::ZERO,
			acc: 12000.0,
			max_vel: 240.0,
			radius,
			stun_duration: 0.0,
			invul_duration: 0.0,
		},
	)).with_children(move |player| {
		player.spawn((
			Transform::default(),
			Visibility::Visible,
			WeaponPivot {
				time_left: 0.0,
				max_time: 0.3,
				backswing: 0.0,
				held: false,
				swinging: false,
				active: false,
				clockwise_swing: true,
			},
		)).with_children(|weapon_pivot| {
			weapon_pivot.spawn((
				Sprite {
					image: textures.weapon.clone(),
					custom_size: Some(Vec2::new(24.0, 96.0)),
					..default()
				},
				Transform {
					translation: Vec3::new(8.0, 72.0, 0.0),
					..Default::default()
				},
			));
			for i in 0..12 {
				weapon_pivot.spawn((
				Sprite {
					image: textures.circle.clone(),
					custom_size: Some(Vec2::new(12.0, 12.0)),
					..default()
				},
				Transform {
					translation: Vec3::new(8.0, radius + 8.0 * i as f32, 0.0),
					..Default::default()
				},
				Visibility::Hidden,
				WeaponCollider,
			));
			}
		});
	});
}

fn player_movement(
	mut player_query: Query<(&mut PlayerInfo, &mut Transform)>,
	reactor_query: Query<&Transform, (With<Reactor>, Without<PlayerInfo>)>,
	weapon_pivot_query: Query<&WeaponPivot>,
	windows: Query<&Window, With<PrimaryWindow>>,
	time: Res<Time>,
) {
	let (mut player, mut transform) = player_query.single_mut().expect("Could not find player");
	let window = windows.single().expect("Could not find window");
	let window_size = Vec2::new(window.width(), window.height());
	if player.invul_duration > 0.0 {
		player.invul_duration = (player.invul_duration - time.delta_secs()).clamp(0.0, 10.0);
	}
	for weapon_pivot in weapon_pivot_query.iter() {
		if player.stun_duration == 0.0 {
			if let Some(mut target) = window.cursor_position() {
				target -= window_size / 2.0;
				target.y = -target.y;
				let offset = target - transform.translation.xy();

				if !weapon_pivot.swinging && offset.length() >= 10.0 {
					let direction = offset.normalize();
					player.vel = (player.vel + player.acc * direction * time.delta_secs()).clamp_length_max(player.max_vel);
				} else {
					player.vel = Vec2::ZERO;
				}

				if !weapon_pivot.active {
					let angle = offset.y.atan2(offset.x);
					transform.rotation = Quat::from_rotation_z(angle);
				}
				
				let mut move_target: Vec2 = Vec2::ZERO;
				move_target.x = (transform.translation.x + player.vel.x * time.delta_secs()).clamp(-540.0 + 47.0 + 12.0, 540.0 - 43.0 - 12.0);
				move_target.y = (transform.translation.y + player.vel.y * time.delta_secs()).clamp(-405.0 + 76.0 + 12.0, 405.0 - 130.0 - 12.0);

				let reactor_loc = reactor_query.single().expect("Could not find reactor");

				if ((move_target - reactor_loc.translation.xy()).length()) > 24.0 + 64.0 {
					transform.translation = move_target.extend(100.0);
				} else {
					transform.translation = ((move_target - reactor_loc.translation.xy()).normalize() * (24.0 + 64.0)).extend(100.0);
				}
			} else {
				player.vel = Vec2::ZERO;
			}
		} else {
			player.stun_duration = (player.stun_duration - time.delta_secs()).clamp(0.0, 10.0);
			player.vel = Vec2::ZERO;
		}
	}
}

fn check_player_lives(
	mut next_state: ResMut<NextState<GameState>>,
	mut player_query: Query<&mut PlayerInfo>,
	mut death_query: Query<&mut Sprite, With<DeathFadeout>>,
	mut score_query: Query<&mut Score>,
	time: Res<Time>,
) {
	let mut p_info = player_query.single_mut().expect("Could not find player");
	let mut sprite = death_query.single_mut().expect("Could not find death fadeout");
	if p_info.death_countdown > 0.0 {
		sprite.color = Color::linear_rgba(0.0, 0.0, 0.0, 1.0 - p_info.death_countdown/1.5);
		if p_info.invul_duration == 0.0 {p_info.invul_duration = 1.0};
		p_info.death_countdown = (p_info.death_countdown - time.delta_secs()).clamp(0.0, 10.0);
		if p_info.death_countdown == 0.0 {
			next_state.set(GameState::Retry);
		};
	} else if p_info.lives <= 0.0 {
		// println!("Score: {}", p_info.score);
		// println!("Time Survived: {}", p_info.time_survived);
		let mut score = score_query.single_mut().expect("Could not find score");
		if p_info.score > score.highscore {score.highscore = p_info.score};
		if p_info.time_survived > score.hightime {score.hightime = p_info.time_survived}; 
		p_info.death_countdown = 1.5;
	} else {
		p_info.time_survived += time.delta_secs();
	}
}

fn weapon_swing(
	mut mouse_events: EventReader<MouseButtonInput>,
	mut weapon_query: Query<(&mut WeaponPivot, &mut Transform)>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
	mut wind_handle: Local<Handle<AudioInstance>>,
	player_query: Query<&PlayerInfo>,
	time: Res<Time>,
	audio: Res<Audio>,
	sfx: Res<AudioAssets>,
) {
	for event in mouse_events.read() {
		if event.button == MouseButton::Left && event.state.is_pressed() {
			let player = player_query.single().expect("Player not found");
			if player.stun_duration == 0.0 {
				for (mut weapon_pivot, _) in weapon_query.iter_mut() {
					if !weapon_pivot.swinging {
						weapon_pivot.time_left = weapon_pivot.max_time;
						weapon_pivot.held = true;
						weapon_pivot.swinging = true;
						*wind_handle = audio.play(sfx.wind_up.clone()).with_volume(0.1).with_playback_rate(0.875 + rand::random::<f64>()/4.0).handle();
					}
				}
			}
		} else {
			for (mut weapon_pivot, _) in weapon_query.iter_mut() {
				if weapon_pivot.swinging && !weapon_pivot.active {
					weapon_pivot.held = false;
					weapon_pivot.active = true;
					if let Some(instance) = audio_instances.get_mut(&**&mut wind_handle) {
						instance.pause(AudioTween::default());
					}
					audio.play(sfx.bat_swing.clone()).with_volume(0.1).with_playback_rate(0.5 + rand::random::<f64>());
				}
			}
		}
	}

	let backswing_amount = PI/2.0;
	let total_angle = PI;
	let offset_angle = 0.0;

	for (mut weapon_pivot, mut transform) in weapon_query.iter_mut() {
		if weapon_pivot.held {
			weapon_pivot.backswing = (weapon_pivot.backswing + (0.5 - weapon_pivot.backswing) * time.delta_secs() * 4.0).clamp(0.0, 0.5);
			let backswing_angle = if weapon_pivot.clockwise_swing {weapon_pivot.backswing * backswing_amount}
				else {-weapon_pivot.backswing * backswing_amount + total_angle};
			transform.rotation = Quat::from_rotation_z(backswing_angle);
		} else if !weapon_pivot.held && weapon_pivot.swinging {
			weapon_pivot.time_left = (weapon_pivot.time_left - time.delta_secs()).clamp(0.0, 10.0);
			let percent = (weapon_pivot.time_left / weapon_pivot.max_time).clamp(0.0, 1.0);
			let angle = if weapon_pivot.clockwise_swing {((1.0 - percent) * (total_angle + weapon_pivot.backswing)) - (weapon_pivot.backswing + offset_angle)}
				else {percent * (total_angle + weapon_pivot.backswing) - (weapon_pivot.backswing + offset_angle)};
			transform.rotation = Quat::from_rotation_z(-angle);

			let scale = (1.0 + ((1.0 - percent).powf(0.7) * PI).sin()) * (0.65 + weapon_pivot.backswing/4.0) ;
			transform.scale = Vec2::splat((scale).clamp(1.0, 2.0)).extend(0.0);

			if weapon_pivot.swinging && weapon_pivot.time_left == 0.0 {
				transform.rotation = if weapon_pivot.clockwise_swing{Quat::from_rotation_z(total_angle-offset_angle)} else {Quat::from_rotation_z(offset_angle)};
				weapon_pivot.swinging = false;
				weapon_pivot.active = false;
				weapon_pivot.backswing = 0.0;
				weapon_pivot.clockwise_swing = !weapon_pivot.clockwise_swing;
			}
		}
	}
}

fn cleanup_game(
	mut commands: Commands,
	player_query: Query<Entity, With<PlayerInfo>>,
	molecule_query: Query<Entity, (Without<PlayerInfo>, With<MoleculeInfo>)>,
	bullet_query: Query<Entity, (Without<PlayerInfo>, Without<MoleculeInfo>, With<BulletInfo>)>,
	reactor_query: Query<Entity, (Without<PlayerInfo>, Without<MoleculeInfo>, Without<BulletInfo>, With<Reactor>)>,
	crosses_query: Query<Entity, (Without<PlayerInfo>, Without<MoleculeInfo>, Without<BulletInfo>, Without<Reactor>, With<Crosses>)>,
) {
	let p_entity = player_query.single().expect("Could not find player");
	commands.entity(p_entity).despawn();
	for entity in molecule_query.iter() {
		commands.entity(entity).despawn();
	}
	for entity in bullet_query.iter() {
		commands.entity(entity).despawn();
	}
	for entity in reactor_query.iter() {
		commands.entity(entity).despawn();
	}
	for entity in crosses_query.iter() {
		commands.entity(entity).despawn();
	}
}