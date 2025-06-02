use crate::GameState;
use bevy::prelude::*;

// Plugin for handling the main physics logic 
// and molecule spawning
pub struct MoleculesPlugin;

impl Plugin for MoleculesPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Playing),
			spawn_molecules,
		)
			.add_systems(Update, (
				molecule_movement,
			).run_if(in_state(GameState::Playing))
		)
		;
	}
}

#[derive(Component)]
pub struct MoleculeInfo {
	vel: Vec2,
	index: usize,
	reacted: bool,
	radius: f32,
	mass: f32,
}

fn spawn_molecules(
	mut commands: Commands,
) {
	for i in 0..30 {
		spawn_molecule(&mut commands, i as f32*Vec3::new(16.0, 0.0, 0.0), Vec2::new(i as f32*16.0, i as f32*32.0), i, 16.0, 16.0);
		println!("Molecule spawned!");
	}
}

fn spawn_molecule(commands: &mut Commands, mol_pos: Vec3, mol_vel: Vec2, mol_index: usize, mol_radius: f32, mol_mass: f32) {
	commands.spawn((
			Sprite{
				custom_size: Some(Vec2::new(mol_radius, mol_radius)),
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
				radius: mol_radius,
				mass: mol_mass,
			}
		));
}

fn move_molecule(
	time: Res<Time>,
	mut molecule_query: Query<(&mut MoleculeInfo, &mut Transform)>,
) {
	for (mut molecule, mut transform) in molecule_query.iter_mut() {

		transform.translation.x += molecule.vel.x * time.delta_secs();
		transform.translation.y += molecule.vel.y * time.delta_secs();

		if transform.translation.x.abs() > 300.0 {molecule.vel.x = -molecule.vel.x};
		if transform.translation.y.abs() > 300.0 {molecule.vel.y = -molecule.vel.y};
	}
}

fn valid_molecule_combination(mol_a: usize, mol_b: usize) -> Vec<usize> {
	let (mol_a, mol_b) = (mol_a.min(mol_b), mol_a.max(mol_b));
	match mol_a {
		0 => match mol_b {
			0 => vec![1],
			1 => vec![2, 2],
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
	mut molecule_query: Query<(Entity, &mut MoleculeInfo, &mut Transform)>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
) {
	let mut iter = molecule_query.iter_combinations_mut();
	while let Some([
		(entity_a, mut m_info_a, mut transform_a),
		(entity_b, mut m_info_b, mut transform_b),
	]) = iter.fetch_next() {
		// Skip over molecule pairs which are not in the same reactor or that have already reacted
		if m_info_a.reacted || m_info_b.reacted {
			continue;
		};
		let offset = transform_a.translation.xy() - transform_b.translation.xy();
		// Molecule collision check takes place here
		if offset.length() <= m_info_a.radius + m_info_b.radius {
			let products = valid_molecule_combination(m_info_a.index, m_info_b.index);
			if !products.is_empty() {
				m_info_a.reacted = true;
				m_info_b.reacted = true;
				for output in products {
					println!("Spawning a {output}");
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
	// Edge collision takes place here
	for (_, mut m_info, mut transform) in molecule_query.iter_mut() {
		m_info.reacted = false;
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
}