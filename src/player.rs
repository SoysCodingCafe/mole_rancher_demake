use std::f32::consts::PI;

use bevy::{
	prelude::*, 
	window::PrimaryWindow, 
	input::mouse::MouseButtonInput, 
	ecs::system::IntoSystem
};
use crate::loading::TextureAssets;
use crate::GameState;

#[derive(Component)]
pub struct PlayerInfo {
	pub vel: Vec2,
	pub acc: f32,
	pub max_vel: f32,
	pub radius: f32,
	pub stun_duration: f32,
}

pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlayerControlSet;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Playing), spawn_player)
			.add_systems(Update, IntoSystem::into_system(player_movement).in_set(PlayerControlSet))
			.add_systems(Update, IntoSystem::into_system(weapon_swing).in_set(PlayerControlSet))
			.configure_sets(Update, PlayerControlSet.run_if(in_state(GameState::Playing)));
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

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
	let radius = 24.0;
	commands.spawn((
		Sprite {
				image: textures.player.clone(),
				custom_size: Some(Vec2::splat(radius * 2.0)),
				..default()
		},
		Transform::from_xyz(0.0, 0.0, 100.0),
		PlayerInfo {
			vel: Vec2::ZERO,
			acc: 12000.0,
			max_vel: 240.0,
			radius,
			stun_duration: 0.0,
		},
	)).with_children(move |player| {
		player.spawn((
			Transform::default(),
			Visibility::Visible,
			WeaponPivot {
				time_left: 0.0,
				max_time: 0.6,
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
	weapon_pivot_query: Query<&WeaponPivot>,
	windows: Query<&Window, With<PrimaryWindow>>,
	time: Res<Time>,
) {
	let (mut player, mut transform) = player_query.single_mut().expect("Could not find player");
	let window = windows.single().expect("Could not find window");
	let window_size = Vec2::new(window.width(), window.height());
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
				
				transform.translation.x = (transform.translation.x + player.vel.x * time.delta_secs()).clamp(-540.0, 540.0);
				transform.translation.y = (transform.translation.y + player.vel.y * time.delta_secs()).clamp(-405.0, 405.0);
			}
		} else {
			player.stun_duration = (player.stun_duration - time.delta_secs()).clamp(0.0, 10.0);
			player.vel = Vec2::ZERO;
		}
	}
}

fn weapon_swing(
	mut mouse_events: EventReader<MouseButtonInput>,
	mut query: Query<(&mut WeaponPivot, &mut Transform)>,
	time: Res<Time>,
) {
	for event in mouse_events.read() {
		if event.button == MouseButton::Left && event.state.is_pressed() {
			for (mut weapon_pivot, _) in query.iter_mut() {
				if !weapon_pivot.swinging {
					weapon_pivot.time_left = weapon_pivot.max_time;
					weapon_pivot.held = true;
					weapon_pivot.swinging = true;
				}
			}
		} else {
			for (mut weapon_pivot, _) in query.iter_mut() {
				if weapon_pivot.swinging {
					weapon_pivot.held = false;
					weapon_pivot.active = true;
				}
			}
		}
	}

	for (mut weapon_pivot, mut transform) in query.iter_mut() {
		if weapon_pivot.held {
			weapon_pivot.backswing = (weapon_pivot.backswing + time.delta_secs() * 40.0).clamp(0.0, 0.5);
			let backswing_angle = if weapon_pivot.clockwise_swing {weapon_pivot.backswing * 40.0}
				else {-weapon_pivot.backswing * 40.0 + 180.0};
			transform.rotation = Quat::from_rotation_z(backswing_angle.to_radians());
		} else if !weapon_pivot.held && weapon_pivot.swinging {
			weapon_pivot.time_left = (weapon_pivot.time_left - time.delta_secs()).clamp(0.0, 10.0);
			let percent = (weapon_pivot.time_left / weapon_pivot.max_time).clamp(0.0, 1.0);
			let angle = if weapon_pivot.clockwise_swing {((1.0 - percent) * (PI + weapon_pivot.backswing)) - weapon_pivot.backswing}
				else {percent * (PI + weapon_pivot.backswing) - weapon_pivot.backswing};
			transform.rotation = Quat::from_rotation_z(-angle);

			let scale = (1.0 + ((1.0 - percent).powf(0.7) * PI).sin()) * 0.65;
			transform.scale = Vec2::splat((scale).clamp(1.0, 2.0)).extend(0.0);

			if weapon_pivot.swinging && weapon_pivot.time_left == 0.0 {
				transform.rotation = if weapon_pivot.clockwise_swing{Quat::from_rotation_z(PI)} else {Quat::IDENTITY};
				weapon_pivot.swinging = false;
				weapon_pivot.active = false;
				weapon_pivot.backswing = 0.0;
				weapon_pivot.clockwise_swing = !weapon_pivot.clockwise_swing;
			}
		}
	}
}
