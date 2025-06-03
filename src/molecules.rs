use crate::GameState;
use bevy::{prelude::*, window::PrimaryWindow};

// Plugin for handling the main physics logic 
// and molecule spawning
pub struct MoleculesPlugin;

impl Plugin for MoleculesPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Playing), (
			spawn_player,
			spawn_molecules,
		).chain())
			.add_systems(Update, (
				player_movement,
				molecule_movement,
				clamp_inside_reactor,
			).chain().run_if(in_state(GameState::Playing))
		)
		;
	}
}

#[derive(Component)]
pub struct MoleculeInfo {
	vel: Vec2,
	index: usize,
	reacted: bool,
	reaction_cooldown: f32,
	radius: f32,
	mass: f32,
}

#[derive(Component)]
pub struct PlayerInfo {
	vel: Vec2,
	acc: f32,
	max_vel: f32,
	radius: f32,
	mass: f32,
	stun_duration: f32,
}

fn spawn_player(
	mut commands: Commands,
) {
	let radius = 24.0;
	commands.spawn((
		Sprite{
			custom_size: Some(Vec2::new(radius*2.0, radius*2.0)),
			..default()
		},
		Transform{
			translation: Vec3::new(0.0, 0.0, 100.0),
			..default()
		},
		PlayerInfo{
			vel: Vec2::ZERO,
			acc: 12000.0,
			max_vel: 240.0,
			radius: radius,
			mass: 99999.0,
			stun_duration: 0.0,
		}
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
		target = target - (window_size / 2.0);
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

	transform.translation.x = (transform.translation.x + player.vel.x * time.delta_secs()).clamp(-640.0, 640.0);
	transform.translation.y = (transform.translation.y + player.vel.y * time.delta_secs()).clamp(-360.0, 360.0);
}

fn spawn_molecules(
	mut commands: Commands,
) {
	for mut i in 0..10 {
		i += 1;
		spawn_molecule(&mut commands, rand_pos(), rand_vel(), 0, 8.0, 16.0);
		println!("Molecule spawned!");
	}
}

fn rand_vel() -> Vec2 {
	Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize() * 120.0
}

fn rand_pos() -> Vec3 {
	(Vec2::new((rand::random::<f32>() - 0.5) * 1280.0, (rand::random::<f32>() - 0.5) * 720.0).clamp_length_min(128.0)).extend(0.0)
}

fn spawn_molecule(commands: &mut Commands, mol_pos: Vec3, mol_vel: Vec2, mol_index: usize, mol_radius: f32, mol_mass: f32) {
	commands.spawn((
			Sprite{
				custom_size: Some(Vec2::new(mol_radius*2.0, mol_radius*2.0)),
				..default()
			},
			Transform{
				translation: mol_pos,
				..default()
			},
			MoleculeInfo{
				vel: mol_vel,
				index: mol_index,
				reacted: false,
				reaction_cooldown: 0.5,
				radius: mol_radius,
				mass: mol_mass,
			}
		));
}

fn valid_molecule_combination(mol_a: usize, mol_b: usize) -> Vec<usize> {
	let (mol_a, mol_b) = (mol_a.min(mol_b), mol_a.max(mol_b));
	match mol_a {
		0 => match mol_b {
			0 => vec![1],
			1 => vec![1],
			_ => vec![],
		}
		1 => match mol_b {
			_ => vec![],
		}
		_ => vec![],
	}
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
	]) = iter.fetch_next() {
		if m_info_a.reacted || m_info_b.reacted {
			continue;
		};
		let offset = transform_a.translation.xy() - transform_b.translation.xy();
		if offset.length() <= m_info_a.radius + m_info_b.radius {
			let products = valid_molecule_combination(m_info_a.index, m_info_b.index);
			if !products.is_empty() && (m_info_a.reaction_cooldown + m_info_b.reaction_cooldown) == 0.0 {
				m_info_a.reacted = true;
				m_info_b.reacted = true;
				m_info_a.reaction_cooldown = 1.0;
				m_info_b.reaction_cooldown = 1.0;
				for output in products {
					if rand::random::<f32>() < 0.5 {
						let new_pos = ((transform_a.translation.xy() + transform_b.translation.xy())/2.0).extend(0.0);
						spawn_molecule(&mut commands, new_pos, rand_vel(), output, 8.0, 16.0);
						println!("Spawning a {output}");
					}
				}
			};

			let relative_velocity = m_info_a.vel - m_info_b.vel;
			let dp = offset * relative_velocity.dot(offset) / ((offset.length_squared()) * (m_info_a.mass + m_info_b.mass));

			m_info_a.vel -= 2.0 * m_info_b.mass * dp;
			m_info_b.vel += 2.0 * m_info_a.mass * dp;

			let push = (offset.normalize() * 1.01 * (m_info_a.radius + m_info_b.radius) - offset).extend(0.0);
			transform_a.translation += push;
			transform_b.translation -= push;
		}
	}

	let origin = Vec2::new(0.0, 0.0);
	let dimensions = Vec2::new(1280.0, 720.0);
	for (_, mut m_info, mut transform) in molecule_query.iter_mut() {
		m_info.reacted = false;
		m_info.reaction_cooldown = (m_info.reaction_cooldown - time.delta_secs()).clamp(0.0, 10.0);
		let target = Vec2::new(
			transform.translation.x + m_info.vel.x * time.delta_secs(), 
			transform.translation.y + m_info.vel.y * time.delta_secs()
		);
		
		let offset = (target - origin).abs();
		if offset.x > dimensions.x / 2.0 - m_info.radius {
			m_info.vel.x = -m_info.vel.x;
		}
		if offset.y > dimensions.y / 2.0 - m_info.radius {
			m_info.vel.y = -m_info.vel.y;
		}

		transform.translation.x = transform.translation.x + m_info.vel.x * time.delta_secs();
		transform.translation.y = transform.translation.y + m_info.vel.y * time.delta_secs();
	}

	let (mut p_info, mut p_transform) = player_query.single_mut().expect("Could not find player");
	for (entity, mut m_info, mut m_transform) in molecule_query.iter_mut() {
		let offset = p_transform.translation.xy() - m_transform.translation.xy();
		if offset.length() <= m_info.radius + p_info.radius {
			p_info.stun_duration = 0.4;
			commands.entity(entity).despawn();
		}
	}
}

fn clamp_inside_reactor(
	mut molecule_query: Query<(&MoleculeInfo, &mut Transform)>,
) {
	for (m_info, mut transform) in molecule_query.iter_mut() {
		let offset = transform.translation.xy().abs();
		if offset.x > 640.0 - m_info.radius {
			transform.translation.x = transform.translation.x.signum() * 640.0 - m_info.radius;
		}
		if offset.y > 360.0 - m_info.radius{
			transform.translation.y = transform.translation.y.signum() * 360.0 - m_info.radius;
		}
	}
}