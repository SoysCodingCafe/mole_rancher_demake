use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::{Audio, AudioControl};
use crate::GameState;
use crate::player::{PlayerInfo, WeaponCollider, WeaponPivot};
use crate::loading::{AudioAssets, TextureAssets};

#[derive(Component)]
pub struct MoleculeInfo {
	pub vel: Vec2,
	pub index: usize,
	pub reacted: bool,
	pub reaction_cooldown: f32,
	pub radius: f32,
	pub mass: f32,
}

#[derive(Component)]
pub struct BulletInfo{
	radius: f32,
}

pub struct MoleculesPlugin;

impl Plugin for MoleculesPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnExit(GameState::Menu), spawn_score)
			.add_systems(Update, update_score.run_if(in_state(GameState::Playing)))
			.add_systems(Update, update_highscore.run_if(in_state(GameState::Retry)))
			.add_systems(OnEnter(GameState::Playing), spawn_reactor)
			.add_systems(Update, (
				//level_editor,
				spawn_molecules,
				molecule_movement,
				move_bullet,
				clamp_inside_reactor,
				destroy_molecules,
			).chain().run_if(in_state(GameState::Playing)));
	}
}

#[derive(Component)]
pub struct Reactor;

#[derive(Resource)]
pub struct SpawnTracker {
	timer: f32,
	increment: usize,
	times: Vec<Vec<f32>>,
	indices: Vec<Vec<usize>>,
	velocities: Vec<Vec<Vec2>>,
	track_player: Vec<Vec<bool>>,
	level: usize,
	level_lengths: Vec<usize>,
}

#[derive(Component)]
pub struct Score{
	pub highscore: f32,
	pub hightime: f32,
}

fn spawn_score(
	mut commands: Commands,
) {
	commands.spawn((
		Text2d::new("Score: 0\nTime Survived: 0"),
		Transform::from_xyz(0.0, -405.0 + 32.0, 200.0),
		Score{
			highscore: 0.0,
			hightime: 0.0,
		},
	));
}

fn update_score(
	mut score_query: Query<&mut Text2d, With<Score>>,
	player_query: Query<&PlayerInfo>,
) {
	let player = player_query.single().expect("Could not find player");
	let mut score = score_query.single_mut().expect("Could not find score");
	if player.lives > 0.0 {
		let time_surv = if player.time_survived < 59.0 {format!{"{:.2}s", player.time_survived % 60.0}} 
		else if player.time_survived >= 59.0 && player.time_survived < 60.0 {
			format!{"59s"}
		} else {
			format!{"{:.0}m {:.0}s", (player.time_survived/60.0).floor() % 60.0, player.time_survived.floor() % 60.0}
		};
		score.0 = format!("Score: {}\nTime Survived: {}", player.score, time_surv);
	}
}

fn update_highscore(
	mut score_query: Query<(&mut Text2d, &Score)>,
) {
	let (mut text, score) = score_query.single_mut().expect("Could not find score");
	let time_surv = if score.hightime < 59.5 {format!{"{:.2}s", score.hightime % 60.0}
	} else if score.hightime >= 59.0 && score.hightime < 60.0 {
		format!{"59s"}
	} else {
		format!{"{:.0}m {:.0}s", (score.hightime/60.0).floor() % 60.0, score.hightime.floor() % 60.0}
	};
	text.0 = format!("Highscore: {}\nLongest Time Survived: {:.2}s", score.highscore, time_surv);
}

fn spawn_reactor(
	mut commands: Commands,
	textures: Res<TextureAssets>,
) {
	commands.spawn((Sprite {
		image: textures.circle.clone(),
		custom_size: Some(Vec2::new(128.0, 128.0)),
		..default()
	},
	Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
	Visibility::Hidden,
	Reactor,
	));

	// let mut times = 			vec![1.0,   3.0,   4.0,   5.0,   5.1,   5.2,   5.3,   5.4,   5.5 ];
	// let mut indices = 		vec![4,     4,     0,     0,     0,     0,     0,     0,     0];
	// let mut angles = 			vec![0.0,   180.0, 180.0, 361.0, 361.0, 361.0, 361.0, 361.0, 361.0];
	// let mut velocities = 		vec![100.0, 150.0, 400.0, 260.0, 260.0, 260.0, 260.0, 260.0, 260.0];

	// let mut times = 			vec![1.0];
	// let mut indices = 		vec![0];
	// let mut angles = 			vec![361.0];
	// let mut velocities = 		vec![260.0];

	// let mut times = vec![vec![]];
	// let mut indices = vec![vec![]];
	// let mut velocities = vec![vec![]];
	// let mut track_player = vec![vec![]];

	let times = vec![vec![1.5349127, 2.201693, 2.7517598, 3.168261, 4.352251, 5.0390396, 5.752186, 6.5185313, 8.484857, 8.801608, 9.118443, 10.535108, 10.834695, 11.118693, 11.418816], vec![14.118426, 14.768145, 15.301594, 15.95203, 17.251623, 17.48482, 17.718147, 18.318827, 18.551878, 18.751999, 19.351501, 19.568718, 19.734749, 21.052189, 21.302149, 21.485407]];
	let indices = vec![vec![0, 0, 0, 0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4]];
	let velocities = vec![vec![Vec2::new(9.5, 108.5), Vec2::new(140.5, -6.0), Vec2::new(-7.0, -111.5), Vec2::new(-134.0, -2.5), Vec2::new(15.0, 80.0), Vec2::new(141.0, -14.0), Vec2::new(-18.0, -117.0), Vec2::new(-139.0, -19.0), Vec2::new(127.5, -32.5), Vec2::new(127.5, -32.5), Vec2::new(127.5, -32.5), Vec2::new(127.5, -32.5), Vec2::new(127.5, -32.5), Vec2::new(127.5, -32.5), Vec2::new(127.5, -32.5)], vec![Vec2::new(150.5, -18.0), Vec2::new(-2.5, 94.5), Vec2::new(-120.5, -5.5), Vec2::new(5.0, -113.0), Vec2::new(0.5, 100.0), Vec2::new(0.5, 100.0), Vec2::new(2.0, 100.0), Vec2::new(129.0, 2.0), 
Vec2::new(129.0, 2.0), Vec2::new(129.0, 2.0), Vec2::new(-4.5, -115.0), Vec2::new(-4.5, -115.0), Vec2::new(-4.5, -115.0), Vec2::new(-115.5, -14.0), Vec2::new(-115.5, -14.0), Vec2::new(-115.5, -14.0)]];
	let track_player = vec![vec![false, false, false, false, false, false, false, false, false, false, false, true, true, true, true], vec![false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true]];

	let mut level_lengths = vec![];
	for i in 0..times.len() {
		level_lengths.push(times[i].len());
	}

	commands.insert_resource(SpawnTracker{
		timer: 0.0,
		increment: 0,
		times: times,
		indices: indices,
		velocities: velocities,
		track_player: track_player,
		level: 0,
		level_lengths: level_lengths,
	});
}

fn level_editor(
	mut current_time: Local<f32>,
	mut spawn_tracker: ResMut<SpawnTracker>,
	player_query: Query<&PlayerInfo>,
	windows: Query<&Window, With<PrimaryWindow>>,
	keys: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	*current_time += time.delta_secs();
	let player = player_query.single().expect("Could not find player");
	let window = windows.single().expect("Could not find window");
	let window_size = Vec2::new(window.width(), window.height());
	if let Some(mut target) = window.cursor_position() {
		target -= window_size / 2.0;
		target.y = -target.y;
		let velocity = target / 2.0;
		let level = spawn_tracker.level;
		let tracked = if keys.pressed(KeyCode::Space) {true} else {false};
		if keys.just_pressed(KeyCode::Digit0) {
			spawn_tracker.times[level].push(*current_time);
			spawn_tracker.indices[level].push(0);
			spawn_tracker.velocities[level].push(velocity);
			spawn_tracker.track_player[level].push(tracked);
		}
		if keys.just_pressed(KeyCode::Digit1) {
			spawn_tracker.times[level].push(*current_time);
			spawn_tracker.indices[level].push(1);
			spawn_tracker.velocities[level].push(velocity);
			spawn_tracker.track_player[level].push(tracked);
		}
		if keys.just_pressed(KeyCode::Digit2) {
			spawn_tracker.times[level].push(*current_time);
			spawn_tracker.indices[level].push(2);
			spawn_tracker.velocities[level].push(velocity);
			spawn_tracker.track_player[level].push(tracked);
		}
		if keys.just_pressed(KeyCode::Digit3) {
			spawn_tracker.times[level].push(*current_time);
			spawn_tracker.indices[level].push(3);
			spawn_tracker.velocities[level].push(velocity);
			spawn_tracker.track_player[level].push(tracked);
		}
		if keys.just_pressed(KeyCode::Digit4) {
			spawn_tracker.times[level].push(*current_time);
			spawn_tracker.indices[level].push(4);
			spawn_tracker.velocities[level].push(velocity);
			spawn_tracker.track_player[level].push(tracked);
		}
		if keys.just_pressed(KeyCode::Digit5) {
			spawn_tracker.times[level].push(*current_time);
			spawn_tracker.indices[level].push(5);
			spawn_tracker.velocities[level].push(velocity);
			spawn_tracker.track_player[level].push(tracked);
		}
		if keys.just_pressed(KeyCode::KeyP) {
			println!("{:?}\n{:?}\n{:?}\n{:?}", spawn_tracker.times, spawn_tracker.indices, spawn_tracker.velocities, spawn_tracker.track_player);
		}
		if keys.just_pressed(KeyCode::KeyL) {
			spawn_tracker.level += 1;
			spawn_tracker.times.push(vec![]);
			spawn_tracker.indices.push(vec![]);
			spawn_tracker.velocities.push(vec![]);
			spawn_tracker.track_player.push(vec![]);
		}
	}

}

fn rand_vel() -> Vec2 {
	Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize() * 260.0
}

fn _rand_pos() -> Vec3 {
	(Vec2::new((rand::random::<f32>() - 0.5) * 1080.0, (rand::random::<f32>() - 0.5) * 810.0).clamp_length_min(128.0)).extend(0.0)
}

fn spawn_molecules(
	mut commands: Commands,
	mut spawn_tracker: ResMut<SpawnTracker>,
	reactor_query: Query<&Transform, With<Reactor>>,
	player_query: Query<&Transform, (Without<Reactor>, With<PlayerInfo>)>,
	textures: Res<TextureAssets>,
	time: Res<Time>,
) {
	spawn_tracker.timer += time.delta_secs() * (1.0 + spawn_tracker.level as f32/20.0);
	if spawn_tracker.timer > spawn_tracker.times[spawn_tracker.level][spawn_tracker.increment] {
		let reactor = reactor_query.single().expect("Could not find reactor");
		let player = player_query.single().expect("Could not find player");
		let pos = Vec2::new(reactor.translation.x, reactor.translation.y - 48.0).extend(1.0);
		let index = spawn_tracker.indices[spawn_tracker.level][spawn_tracker.increment];
		let angle = if spawn_tracker.track_player[spawn_tracker.level][spawn_tracker.increment] {(player.translation.xy() - pos.xy()).normalize()} 
		else {spawn_tracker.velocities[spawn_tracker.level][spawn_tracker.increment].normalize()};
		spawn_molecule(&mut commands, &textures, pos, angle * spawn_tracker.velocities[spawn_tracker.level][spawn_tracker.increment].length(), index, get_molecule_radius(index), get_molecule_mass(index));
		if spawn_tracker.increment == spawn_tracker.level_lengths[spawn_tracker.level] - 1 {
			spawn_tracker.increment = 0;
			spawn_tracker.timer = 0.0;
			if spawn_tracker.level == spawn_tracker.level_lengths.len() - 1 {
				spawn_tracker.level += 1;
			} else {
				spawn_tracker.level = 0;
			}
		} else {
			spawn_tracker.increment += 1;
		}
	}
}

fn spawn_molecule(commands: &mut Commands, textures: &Res<TextureAssets>, pos: Vec3, vel: Vec2, index: usize, radius: f32, mass: f32) {
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
			image: textures.circle.clone(),
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

#[derive(Component)]
pub struct Crosses;

fn spawn_cross(commands: &mut Commands, textures: &Res<TextureAssets>, index: f32) {
	commands.spawn((
		Sprite {
			image: textures.cross.clone(),
			custom_size: Some(Vec2::splat(64.0)),
			..default()
		},
		Transform {
			translation: Vec3::new(-300.0 * index + 300.0, 330.0, 200.0),
			..default()
		},
		Crosses,
	));
}

fn spawn_bullet(commands: &mut Commands, textures: &Res<TextureAssets>, pos: Vec3, radius: f32) {
	let colours = [
		Color::hsv(60.0, 0.82, 0.45),
		Color::hsv(53.0, 0.88, 0.74),
		Color::hsv(10.0, 0.77, 0.75),
		Color::hsv(354.0, 0.45, 0.80),
		Color::hsv(281.0, 0.53, 0.32),
		Color::hsv(27.0, 0.47, 0.84),
		Color::hsv(32.0, 0.14, 0.77),
	];
	let colour = colours[5];

	commands.spawn((
		Sprite {
			image: textures.circle.clone(),
			color: colour,
			custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)),
			..default()
		},
		Transform {
			translation: pos,
			..default()
		},
		BulletInfo {
			radius,
		},
	));
}

fn take_damage(
	entity: Entity,
	p_info: &mut PlayerInfo,
	mut commands: &mut Commands,
	textures: &Res<TextureAssets>,
	audio: &Res<Audio>,
	sfx: &Res<AudioAssets>,
) {
	if p_info.invul_duration == 0.0 {
		p_info.invul_duration = 1.0;
		p_info.stun_duration = 0.4;
		p_info.lives -= 1.0;
		spawn_cross(&mut commands, &textures, p_info.lives as f32);
		audio.play(sfx.radiation_hit.clone()).with_volume(0.1).with_playback_rate(1.0 - (2.0 - p_info.lives as f64) * 0.2);
	}
	commands.entity(entity).despawn();
}

fn move_bullet(
	mut commands: Commands,
	mut player_query: Query<(&Transform, &mut PlayerInfo)>,
	mut bullet_query: Query<(Entity, &mut Transform), (With<BulletInfo>, Without<PlayerInfo>)>,
	textures: Res<TextureAssets>,
	audio: Res<Audio>,
	sfx: Res<AudioAssets>,
	time: Res<Time>,
) {
	let (p_transform, mut p_info) = player_query.single_mut().expect("Could not find player");
	for (entity, mut b_transform) in bullet_query.iter_mut() {
		let offset = p_transform.translation.xy() - b_transform.translation.xy();
		if offset.length() < 6.0 + 24.0 {
			take_damage(entity, &mut p_info, &mut commands, &textures, &audio, &sfx);
		} else {
			b_transform.translation = (b_transform.translation.xy() + (120.0 * offset.normalize() * time.delta_secs())).extend(1.0);
		}
	}
}

pub enum ReactionInfo {
	Reaction(Vec<usize>),
	None,
}

fn valid_molecule_combination(a: usize, b: usize) -> ReactionInfo {
	let (a, b) = (a.min(b), a.max(b));
	match a {
		0 => match b {
			0 => ReactionInfo::Reaction(vec![100, 101]),
			1 => ReactionInfo::Reaction(vec![100, 0, 0]),
			2 => ReactionInfo::Reaction(vec![100, 1, 0]),
			3 => ReactionInfo::Reaction(vec![100, 2, 1]),
			4 => ReactionInfo::Reaction(vec![100, 3, 3]),
			_ => ReactionInfo::None,
		}
		1 => match b {
			4 => ReactionInfo::Reaction(vec![100, 101, 101, 101, 101]),
			_ => ReactionInfo::None,
		}
		2 => match b {
			2 => ReactionInfo::Reaction(vec![100, 4]),
			_ => ReactionInfo::None,
		}
		3 => match b {
			_ => ReactionInfo::None,
		}
		4 => match b {
			_ => ReactionInfo::None,
		}
		_ => ReactionInfo::None,
	}
}

fn get_molecule_radius(index: usize) -> f32 {
	match index {
		0 => 6.0,
		1 => 8.0,
		2 => 10.0,
		3 => 12.0,
		4 => 16.0,
		_ => 20.0,
	}
}

fn get_molecule_mass(index: usize) -> f32 {
	match index {
		0 => 6.0,
		1 => 8.0,
		2 => 10.0,
		3 => 12.0,
		4 => 16.0,
		_ => 20.0,
	}
}

fn molecule_movement(
	mut commands: Commands,
	mut molecule_query: Query<(Entity, &mut MoleculeInfo, &mut Transform), Without<PlayerInfo>>,
	mut player_query: Query<(&mut PlayerInfo, &mut Transform)>,
	textures: Res<TextureAssets>,
	audio: Res<Audio>,
	sfx: Res<AudioAssets>,
	time: Res<Time>,
) {
	let mut molecule_count = 0;
	for _ in molecule_query.iter() {
		molecule_count += 1;
	}
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
		if offset.length() <= m_info_a.radius + m_info_b.radius && molecule_count < 200 {
			let info = valid_molecule_combination(m_info_a.index, m_info_b.index);
			match info {
				ReactionInfo::None => (),
				ReactionInfo::Reaction(products) => {
					if m_info_a.reaction_cooldown + m_info_b.reaction_cooldown == 0.0 {
						m_info_a.reacted = true;
						m_info_b.reacted = true;
						m_info_a.reaction_cooldown = 1.0;
						m_info_b.reaction_cooldown = 1.0;
						for output in products {
							let pos = (transform_b.translation.xy() + offset/2.0 + rand::random::<f32>()).extend(0.0);
							if output < 100 {
								let radius = get_molecule_radius(output);
								let mass = get_molecule_mass(output);
								spawn_molecule(&mut commands, &textures, pos, rand_vel(), output, radius, mass);
							} else {
								match output {
									100 => {
										commands.entity(entity_a).despawn();
										commands.entity(entity_b).despawn();
									}
									101 => {
										spawn_bullet(&mut commands, &textures, pos, 6.0);
									}
									_ => (),
								}
							}
						}
					}
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
		let pos = transform.translation.xy();
		if pos.x > 540.0 - 43.0/2.0 - m_info.radius || pos.x < -540.0 + 47.0/2.0 + m_info.radius {
			m_info.vel.x = -m_info.vel.x;
		}
		if pos.y > 405.0 - 130.0/2.0 - m_info.radius || pos.y < -405.0 + 76.0 + m_info.radius {
			m_info.vel.y = -m_info.vel.y;
		}
	}

	let (mut p_info, p_transform) = player_query.single_mut().expect("Could not find player");
	for (entity, _, m_transform) in molecule_query.iter_mut() {
		let offset = p_transform.translation.xy() - m_transform.translation.xy();
		if offset.length() <= p_info.radius + 8.0 {
			take_damage(entity, &mut p_info, &mut commands, &textures, &audio, &sfx);
		}
	}
}

fn clamp_inside_reactor(mut molecule_query: Query<(&MoleculeInfo, &mut Transform)>) {
	for (m_info, mut transform) in molecule_query.iter_mut() {
		let pos = transform.translation.xy();
		if pos.x > 540.0 - 43.0/2.0 - m_info.radius {
			transform.translation.x = 540.0 - 43.0/2.0 - m_info.radius;
		}
		if pos.x < -540.0 + 47.0/2.0 + m_info.radius {
			transform.translation.x = -540.0 + 47.0/2.0 + m_info.radius;
		}
		if pos.y > 405.0 - 130.0/2.0 - m_info.radius {
			transform.translation.y = 405.0 - 130.0/2.0 - m_info.radius;
		}
		if pos.y < -405.0 + 76.0 + m_info.radius {
			transform.translation.y = -405.0 + 76.0 + m_info.radius;
		}
	}
}

fn destroy_molecules(
	mut commands: Commands,
	mut player_query: Query<&mut PlayerInfo>,
	molecule_query: Query<(Entity, &MoleculeInfo, &Transform)>,
	bullet_query: Query<(Entity, &BulletInfo, &Transform), Without<MoleculeInfo>>,
	weapon_collider_query: Query<&GlobalTransform, With<WeaponCollider>>,
	weapon_pivot_query: Query<(&Transform, &WeaponPivot)>,
	audio: Res<Audio>,
	sfx: Res<AudioAssets>,
) {
	let mut p_info = player_query.single_mut().expect("Could not find player");
	for (wp_transform, weapon) in weapon_pivot_query.iter(){
		if weapon.active {
			for (entity, m_info, m_transform) in molecule_query.iter() {
				for w_transform in weapon_collider_query.iter() {
					let offset = m_transform.translation.xy() - w_transform.translation().xy();
					if offset.length() <= m_info.radius + 6.0 * wp_transform.scale.x {
						p_info.score += m_info.index as f32 + 1.0;
						commands.entity(entity).despawn();
						audio.play(sfx.bounce_and_crackle.clone()).with_volume(0.1).with_playback_rate(0.5 + (rand::random::<f64>()));
						break;
					}
				}
			}

			for (entity, b_info, m_transform) in bullet_query.iter() {
				for w_transform in weapon_collider_query.iter() {
					let offset = m_transform.translation.xy() - w_transform.translation().xy();
					if offset.length() <= b_info.radius + 6.0 * wp_transform.scale.x {
						p_info.score += 1.0;
						commands.entity(entity).despawn();
						break;
					}
				}
			}
		}
	}
}