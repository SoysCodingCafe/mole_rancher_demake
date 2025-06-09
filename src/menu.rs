use crate::loading::TextureAssets;
use crate::GameState;
use crate::postprocess::PostProcessSettings;

use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), (setup_menu, spawn_background))
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
			;
    }
}

#[derive(Component)]
pub struct DeathFadeout;


fn spawn_background(mut commands: Commands, textures: Res<TextureAssets>) {
	commands.spawn((
		Sprite {
            image: textures.background.clone(),
			custom_size: Some(Vec2::new(1080.0, 810.0)),
			..default()
		},
		Transform {
			translation: Vec3::new(0.0, 0.0, 0.0),
			..default()
		}
	));
	commands.spawn((
		Sprite {
            color: Color::linear_rgba(0.0, 0.0, 0.0, 0.0),
			custom_size: Some(Vec2::new(1080.0, 810.0)),
			..default()
		},
		Transform {
			translation: Vec3::new(0.0, 0.0, 999.0),
			..default()
		},
		DeathFadeout,
	));
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgba(0.0, 0.0, 0.0, 0.5),
            hovered: Color::linear_rgba(0.1, 0.1, 0.1, 0.5),
        }
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
pub struct MainCamera;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
	commands.spawn((
        Camera2d,
		Transform::from_xyz(0.0, 0.0, 1000.0),
        MainCamera,
        PostProcessSettings {
            intensity: 0.025,
            scanline_freq: 202.5,
            line_intensity: 0.1,
            ..default()
        },
    ));
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
        Menu,
	));
    commands.spawn((
		Sprite {
            image: textures.title.clone(),
            color: Color::linear_rgba(1.0, 1.0, 1.0, 1.0),
			//custom_size: Some(Vec2::new(1080.0, 810.0)),
			..default()
		},
		Transform {
			translation: Vec3::new(0.0, 0.0, 10.0),
			..default()
		},
        Menu,
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
            Menu,
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
            Menu,
        ));
    }
    commands
	.spawn((
		Node {
			position_type: PositionType::Absolute,
			left: Val::Percent(50.0),
			top: Val::Percent(85.0),
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
		Menu,
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
				Text::new("PLAY"),
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
            Menu,
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
                    BorderRadius::MAX,
                    ButtonColors {
                        normal: Color::linear_rgba(0.0, 0.0, 0.0, 0.5),
                        hovered: Color::linear_rgba(0.1, 0.1, 0.1, 0.5),
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
                    BorderRadius::MAX,
                    ButtonColors {
                        normal: Color::linear_rgba(0.0, 0.0, 0.0, 0.5),
                        hovered: Color::linear_rgba(0.1, 0.1, 0.1, 0.5),
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

fn click_play_button(
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

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
