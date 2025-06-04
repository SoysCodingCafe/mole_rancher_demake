use bevy::{
	prelude::*, 
	window::PrimaryWindow, 
	input::mouse::MouseButtonInput, 
	ecs::system::IntoSystem
};
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlayerControlSet;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Playing), spawn_player)
			.add_systems(Update, IntoSystem::into_system(player_movement).in_set(PlayerControlSet))
			.add_systems(Update, IntoSystem::into_system(weapon_swing).in_set(PlayerControlSet))
			.add_systems(Update, IntoSystem::into_system(weapon_swing_update).in_set(PlayerControlSet))
			.configure_sets(Update, PlayerControlSet.run_if(in_state(GameState::Playing)));
	}
}


#[derive(Component)]
pub struct Weapon;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
	let player_texture = asset_server.load("textures/player.png");
	let weapon_texture = asset_server.load("textures/weapon.png");
	let radius = 24.0;
	commands.spawn((
		Transform::from_xyz(0.0, 0.0, 100.0),
		GlobalTransform::default(),
		Visibility::Visible,
		PlayerInfo {
			vel: Vec2::ZERO,
			acc: 12000.0,
			max_vel: 240.0,
			radius,
			mass: 99999.0,
			stun_duration: 0.0,
		},
	)).with_children(move |parent| {
		parent.spawn((
			Sprite {
				image: player_texture.clone(),
				custom_size: Some(Vec2::splat(radius * 2.0)),
				..default()
			},
			Transform::default(),
			GlobalTransform::default(),
		));
		parent.spawn((
			Transform::from_translation(Vec3::new(8.0, 24.0, 1.0)),
			GlobalTransform::default(),
			Weapon,
		)).with_children(|weapon| {
			weapon.spawn((
				Sprite {
					image: weapon_texture.clone(),
					custom_size: Some(Vec2::new(8.0, 32.0)),
					..default()
				},
				Transform {
					translation: Vec3::new(0.0, 48.0, 0.0),
					scale: Vec3::splat(3.0),
					..Default::default()
				},
				GlobalTransform::default(),
			));
		});
	});
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

		let offset = target - transform.translation.xy();

		let angle = offset.y.atan2(offset.x);
		transform.rotation = Quat::from_rotation_z(angle);

		if player.stun_duration == 0.0 {
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

#[derive(Component)]
struct WeaponSwing {
	progress: f32,
	duration: f32,
}

fn weapon_swing(
	mut mouse_events: EventReader<MouseButtonInput>,
	mut query: Query<(Entity, &Transform), (With<Weapon>, Without<WeaponSwing>)>,
	mut commands: Commands,
) {
	for event in mouse_events.read() {
		if event.button == MouseButton::Left && event.state.is_pressed() {
			for (entity, _) in query.iter_mut() {
				commands.entity(entity).insert(WeaponSwing {
					progress: 0.0,
					duration: 0.25,
				});
			}
		}
	}
}

fn weapon_swing_update(
	mut query: Query<(Entity, &mut Transform, &mut WeaponSwing), With<Weapon>>,
	time: Res<Time>,
	mut commands: Commands,
) {
	for (entity, mut transform, mut swing) in query.iter_mut() {
		swing.progress += time.delta_secs();
		let t = (swing.progress / swing.duration).clamp(0.0, 1.0);
		let angle = if t < 0.5 {
			170.0 * (t / 0.5)
		} else {
			170.0 * (1.0 - (t - 0.5) / 0.5)
		};
		transform.rotation = Quat::from_rotation_z(-angle.to_radians());
		if swing.progress >= swing.duration {
			transform.rotation = Quat::IDENTITY;
			commands.entity(entity).remove::<WeaponSwing>();
		}
	}
}
