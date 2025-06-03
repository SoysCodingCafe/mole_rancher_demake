use crate::GameState;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

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
	mut commands: Commands, asset_server: Res<AssetServer>,
) {
	let texture = asset_server.load("textures/circle.png");
	let colours = [
	Color::hsv(60.0, 0.82, 0.45),  // Shrek Green
	Color::hsv(53.0, 0.88, 0.74),  // Piss Yellow
	Color::hsv(10.0, 0.77, 0.75),  // Bromine Orange
	Color::hsv(354.0, 0.45, 0.80), // Hello Kitty Pink
	Color::hsv(281.0, 0.53, 0.32), // Bruise Purple
	Color::hsv(27.0, 0.47, 0.84),  // Sandy Cheeks Tan
	Color::hsv(32.0, 0.14, 0.77),  // Old T-shirt White
	];

	for i in 0..30 {
		let pos = i as f32 * Vec3::new(16.0, 0.0, 0.0);
		let vel = Vec2::new(i as f32 * 16.0, i as f32 * 32.0);
		spawn_molecule(&mut commands, pos, vel, i, 16.0, 16.0, texture.clone(), &colours);
		println!("Molecule spawned!");
	}
}

fn spawn_molecule(commands: &mut Commands, mol_pos: Vec3, mol_vel: Vec2, mol_index: usize, mol_radius: f32, mol_mass: f32, texture_handle: Handle<Image>, colours: &[Color]) {
	// Chooses a random colour for the molecule
	let mut rng = thread_rng();
	let colour = *colours.choose(&mut rng).unwrap_or(&Color::WHITE);
	
	commands.spawn((
			Sprite{
				image: texture_handle,
				color: colour,
				..default()
			},
			Transform {
				translation: mol_pos,
				scale: Vec3::splat(mol_radius / 32.0),
				..default()
			},
			MoleculeInfo {
				vel: mol_vel,
				index: mol_index,
				reacted: false,
				radius: mol_radius,
				mass: mol_mass,
			},
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
	let dimensions = Vec2::new(1080.0, 810.0);
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