// Use the same UI idioms as ui.rs: Node, Button, Text, TextFont, TextColor, etc.
use bevy::{color::palettes::basic::*, prelude::*};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ActionMenuTarget {
    pub label: String,
}

#[derive(Component)]
pub struct ActionMenuRoot;

pub fn show_action_menu_system(
    ship_query: Query<&Transform, With<crate::ship::spaceship::Spaceship>>,
    target_query: Query<(&Transform, &ActionMenuTarget)>,
    mut commands: Commands,
    menu_query: Query<Entity, With<ActionMenuRoot>>,
    asset_server: Res<AssetServer>,
) {
    let ship_transform = match ship_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };
    let ship_pos = ship_transform.translation.truncate();
    let mut show_menu = None;
    for (target_transform, target) in &target_query {
        let target_pos = target_transform.translation.truncate();
        if ship_pos.distance(target_pos) < 120.0 {
            show_menu = Some(target.label.clone());
            break;
        }
    }
    if let Some(label) = show_menu {
        // Show menu if not already present
        if menu_query.single().is_err() {
            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(40.0),
                        top: Val::Percent(40.0),
                        width: Val::Px(320.0),
                        height: Val::Px(200.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE.with_alpha(0.95)),
                    ActionMenuRoot,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(format!("{} Actions", label)),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::BLACK.into()),
                    ));
                    parent
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect::top(Val::Px(24.0)),
                            ..default()
                        },))
                        .with_children(|button_parent| {
                            button_parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
                                        border: UiRect::all(Val::Px(5.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BorderColor(Color::BLACK),
                                    BackgroundColor(NORMAL_BUTTON),
                                    BorderRadius::MAX,
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        Text::new("Dock"),
                                        TextFont {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 33.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        TextShadow::default(),
                                    ));
                                });
                        });
                });
        }
    } else {
        // Hide menu if present
        if let Ok(entity) = menu_query.single() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn action_menu_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<bevy::prelude::NextState<crate::GameState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Dock".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                next_state.set(crate::GameState::Docked); // Switch to docked scene
            }
            Interaction::Hovered => {
                **text = "Dock".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Dock".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
