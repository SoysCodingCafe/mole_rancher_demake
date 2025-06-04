use bevy::prelude::*;
use crate::GameState;
use crate::player::{PlayerInfo, Weapon};

#[derive(Component)]
pub struct MoleculeInfo {
	pub vel: Vec2,
	pub index: usize,
	pub reacted: bool,
	pub reaction_cooldown: f32,
	pub radius: f32,
	pub mass: f32,
}

pub struct MoleculesPlugin;

impl Plugin for MoleculesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Playing), spawn_molecules)
			.add_systems(Update, (
				molecule_movement,
				clamp_inside_reactor,
			).chain().run_if(in_state(GameState::Playing)));
	}
}

fn spawn_molecules(mut commands: Commands, asset_server: Res<AssetServer>) {
	for _ in 0..10 {
		spawn_molecule(&mut commands, &asset_server, rand_pos(), rand_vel(), 0, 8.0, 16.0);
	}
}

fn spawn_molecule(commands: &mut Commands, asset_server: &AssetServer, pos: Vec3, vel: Vec2, index: usize, radius: f32, mass: f32) {
	let texture = asset_server.load("textures/circle.png");
	let colours = [
		Color::hsv(60.0, 0.82, 0.45),
		Color::hsv(53.0, 0.88, 0.74),
		Color::hsv(10.0, 0.77, 0.75),
		Color::hsv(354.0, 0.45, 0.80),
		Color::hsv(281.0, 0.53, 0.32),
		Color::hsv(27.0, 0.47, 0.84),
		Color::hsv(32.0, 0.14, 0.77),
	];
	let colour = colours[index];

	commands.spawn((
		Sprite {
			image: texture,
			color: colour,
			custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
			..default()
		},
		Transform {
			translation: pos,
			..default()
		},
		MoleculeInfo {
			vel,
			index,
			reacted: true,
			reaction_cooldown: 0.5,
			radius,
			mass,
		},
	));
}

fn molecule_movement(
	mut commands: Commands,
	mut molecule_query: Query<(Entity, &mut MoleculeInfo, &mut Transform), Without<PlayerInfo>>,
	mut player_query: Query<(&mut PlayerInfo, &mut Transform)>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
) {
	let mut iter = molecule_query.iter_combinations_mut();
	while let Some([
		(entity_a, mut m_info_a, mut transform_a),
		(entity_b, mut m_info_b, mut transform_b),
	]) = iter.fetch_next()
	{
		if m_info_a.reacted || m_info_b.reacted {
			continue;
		}
		let offset = transform_a.translation.xy() - transform_b.translation.xy();
		if offset.length() <= m_info_a.radius + m_info_b.radius {
			let products = valid_molecule_combination(m_info_a.index, m_info_b.index);
			if !products.is_empty() && m_info_a.reaction_cooldown + m_info_b.reaction_cooldown == 0.0 {
				m_info_a.reacted = true;
				m_info_b.reacted = true;
				m_info_a.reaction_cooldown = 1.0;
				m_info_b.reaction_cooldown = 1.0;
				for output in products {
					let pos = ((transform_a.translation.xy() + transform_b.translation.xy()) / 2.0).extend(0.0);
					let radius = (m_info_a.radius + m_info_b.radius) / 2.0;
					let mass = ((m_info_a.mass + m_info_b.mass) / 2.0).clamp(4.0, 16.0);
					spawn_molecule(&mut commands, &asset_server, pos, rand_vel(), output, radius, mass);
				}
			}

			let relative_velocity = m_info_a.vel - m_info_b.vel;
			let dp = offset * relative_velocity.dot(offset) / (offset.length_squared() * (m_info_a.mass + m_info_b.mass));
			m_info_a.vel -= 2.0 * m_info_b.mass * dp;
			m_info_b.vel += 2.0 * m_info_a.mass * dp;

			let push = (offset.normalize() * 1.01 * (m_info_a.radius + m_info_b.radius) - offset).extend(0.0);
			transform_a.translation += push;
			transform_b.translation -= push;
		}
	}

	for (_, mut m_info, mut transform) in molecule_query.iter_mut() {
		m_info.reacted = false;
		m_info.reaction_cooldown = (m_info.reaction_cooldown - time.delta_secs()).clamp(0.0, 10.0);
		transform.translation.x += m_info.vel.x * time.delta_secs();
		transform.translation.y += m_info.vel.y * time.delta_secs();
		let pos = transform.translation.xy().abs();
		if pos.x > 540.0 - m_info.radius {
			m_info.vel.x = -m_info.vel.x;
		}
		if pos.y > 405.0 - m_info.radius {
			m_info.vel.y = -m_info.vel.y;
		}
	}

	let (mut p_info, mut p_transform) = player_query.single_mut().expect("Could not find player");
	for (entity, _, m_transform) in molecule_query.iter_mut() {
		let offset = p_transform.translation.xy() - m_transform.translation.xy();
		if offset.length() <= p_info.radius + 8.0 {
			p_info.stun_duration = 0.4;
			commands.entity(entity).despawn();
		}
	}
}

fn clamp_inside_reactor(mut molecule_query: Query<(&MoleculeInfo, &mut Transform)>) {
	for (m_info, mut transform) in molecule_query.iter_mut() {
		let offset = transform.translation.xy().abs();
		if offset.x > 540.0 - m_info.radius {
			transform.translation.x = transform.translation.x.signum() * (540.0 - m_info.radius);
		}
		if offset.y > 405.0 - m_info.radius {
			transform.translation.y = transform.translation.y.signum() * (405.0 - m_info.radius);
		}
	}
}

fn rand_vel() -> Vec2 {
	Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize() * 120.0
}

fn rand_pos() -> Vec3 {
	(Vec2::new((rand::random::<f32>() - 0.5) * 1080.0, (rand::random::<f32>() - 0.5) * 810.0).clamp_length_min(128.0)).extend(0.0)
}

fn valid_molecule_combination(a: usize, b: usize) -> Vec<usize> {
	let (a, b) = (a.min(b), a.max(b));
	match a {
		0 => match b {
			0 => vec![1],
			1 => vec![2],
			_ => vec![3],
		}
		1 => match b {
			_ => vec![],
		}
		_ => vec![],
	}
}
