use crate::loading::TextureAssets;
use crate::menu::DeathFadeout;
use crate::GameState;

use bevy::prelude::*;

pub struct RetryPlugin;

impl Plugin for RetryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Retry), setup_retry)
            .add_systems(Update, (click_retry_button, death_fadein).run_if(in_state(GameState::Retry)))
            .add_systems(OnExit(GameState::Retry), cleanup_retry)
			;
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Retry;

fn setup_retry(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
		Sprite {
            image: textures.ditheredbackground.clone(),
            color: Color::linear_rgba(1.0, 1.0, 1.0, 1.0),
			custom_size: Some(Vec2::new(1080.0, 810.0)),
			..default()
		},
		Transform {
			translation: Vec3::new(0.0, 0.0, 1.0),
			..default()
		},
        Retry,
	));
    for y in (-810.0 as i32 / 2..=810.0 as i32 / 2).step_by(40.0 as usize) {
        commands.spawn((
            Sprite {
                color: Color::linear_rgb(0.4, 0.64, 0.72),
                custom_size: Some(Vec2::new(1080.0, 2.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, y as f32, 5.0),
                ..default()
            },
            Retry,
        ));
    }
    for x in (-1080.0 as i32 / 2..=1080.0 as i32 / 2).step_by(40.0 as usize) {
        commands.spawn((
            Sprite {
                color: Color::linear_rgb(0.4, 0.64, 0.72),
                custom_size: Some(Vec2::new(2.0, 810.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(x as f32, 0.0, 5.0),
                ..default()
            },
            Retry,
        ));
    }
    commands
	.spawn((
		Node {
			position_type: PositionType::Absolute,
			left: Val::Percent(50.0),
			top: Val::Percent(50.0),
			width: Val::Px(202.0),
			height: Val::Px(50.0),
			margin: UiRect {
				left: Val::Px(-101.0),
				top: Val::Px(-25.0),
				..default()
			},
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..default()
		},
		Retry,
	))
	.with_children(|children| {
		let button_colors = ButtonColors::default();
		children
			.spawn((
				Button,
				Node {
					width: Val::Px(202.0),
					height: Val::Px(50.0),
					border: UiRect::all(Val::Px(2.0)),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					..Default::default()
				},
				BorderColor(Color::linear_rgb(0.4, 0.64, 0.72)),
				BackgroundColor(button_colors.normal),
				button_colors,
				ChangeState(GameState::Playing),
			))
			.with_child((
				Text::new("RETRY"),
				TextFont {
					font_size: 35.0,
					..default()
				},
				TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
			));
	});
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            Retry,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
			children
                .spawn((
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                ));
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/SoysCodingCafe/mole_rancher_demake"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Source Code Available Here"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_retry_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn death_fadein(
	time: Res<Time>,
	mut death_query: Query<&mut Sprite, With<DeathFadeout>>,
) {
	let mut d_sprite = death_query.single_mut().expect("Could not find death fadeout");
	if d_sprite.color.alpha() > 0.0 {
		d_sprite.color = Color::linear_rgba(0.0, 0.0, 0.0, (d_sprite.color.alpha() - time.delta_secs()).clamp(0.0, 1.0));
	};
}

fn cleanup_retry(mut commands: Commands, retry: Query<Entity, With<Retry>>) {
    for entity in retry.iter() {
        commands.entity(entity).despawn();
    }
}
