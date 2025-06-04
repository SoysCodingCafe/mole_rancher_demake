use bevy::{prelude::*, window::PrimaryWindow};
use crate::GameState;

#[derive(Component)]
pub struct PlayerInfo {
	pub vel: Vec2,
	pub acc: f32,
	pub max_vel: f32,
	pub radius: f32,
	pub mass: f32,
	pub stun_duration: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Playing), spawn_player)
			.add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
	}
}

fn spawn_player(mut commands: Commands) {
	let radius = 24.0;
	commands.spawn((
		Sprite {
			custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
			..default()
		},
		Transform {
			translation: Vec3::new(0.0, 0.0, 100.0),
			..default()
		},
		PlayerInfo {
			vel: Vec2::ZERO,
			acc: 12000.0,
			max_vel: 240.0,
			radius,
			mass: 99999.0,
			stun_duration: 0.0,
		},
	));
}

fn player_movement(
	mut player_query: Query<(&mut PlayerInfo, &mut Transform)>,
	windows: Query<&Window, With<PrimaryWindow>>,
	time: Res<Time>,
) {
	let (mut player, mut transform) = player_query.single_mut().expect("Could not find player");
	let window = windows.single().expect("Could not find window");
	let window_size = Vec2::new(window.width(), window.height());
	if let Some(mut target) = window.cursor_position() {
		target -= window_size / 2.0;
		target.y = -target.y;
		if player.stun_duration == 0.0 {
			let offset = target - transform.translation.xy();
			if offset.length() >= 10.0 {
				let direction = offset.normalize();
				player.vel = (player.vel + player.acc * direction * time.delta_secs()).clamp_length_max(player.max_vel);
			} else {
				player.vel = Vec2::ZERO;
			}
		} else {
			player.stun_duration = (player.stun_duration - time.delta_secs()).clamp(0.0, 10.0);
			player.vel = Vec2::ZERO;
		}
	}

	transform.translation.x = (transform.translation.x + player.vel.x * time.delta_secs()).clamp(-540.0, 540.0);
	transform.translation.y = (transform.translation.y + player.vel.y * time.delta_secs()).clamp(-405.0, 405.0);
}
