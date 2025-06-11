use crate::ship::spaceship::Spaceship;
use bevy::color::palettes::css::{DARK_CYAN, DARK_GRAY, YELLOW};
use bevy::prelude::*;

#[derive(Component)]
pub struct SidePanelRoot;

pub fn spaceship_ui_panel(
    q: Query<&Spaceship>,
    mut commands: Commands,
    root_query: Query<Entity, With<SidePanelRoot>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(root) = root_query.single() {
        commands.entity(root).despawn();
    }
    let ship = q.single().unwrap();
    let bar_width = 200.0;
    let bar_height = 24.0;
    let margin = 8.0;
    let font_size = 18.0;
    let text_font = TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        ..default()
    };
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(24.0),
                top: Val::Px(24.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(margin),
                ..default()
            },
            BackgroundColor(Color::NONE),
            SidePanelRoot,
        ))
        .with_children(|parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands| {
            // Speed indicator only (no throttle bar)
            parent
                .spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexEnd,
                        margin: UiRect::right(Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(Color::from(DARK_GRAY).with_alpha(0.7)),
                ))
                .with_children(|col| {
                    // Speed indicator
                    col.spawn((
                        Text::new(format!("Speed: {:.0}", ship.throttle)),
                        text_font.clone(),
                        Node {
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                    ));
                });
            // Stats panel
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        row_gap: Val::Px(margin),
                        ..default()
                    },
                    BackgroundColor(Color::BLACK.with_alpha(0.7)),
                ))
                .with_children(|panel| {
                    spawn_bar(
                        panel,
                        "Fuel",
                        ship.fuel,
                        YELLOW.into(),
                        bar_width,
                        bar_height,
                        font_size,
                        &text_font,
                    );
                    spawn_bar(
                        panel,
                        "Hull",
                        ship.hull,
                        DARK_CYAN.into(),
                        bar_width,
                        bar_height,
                        font_size,
                        &text_font,
                    );
                    spawn_bar(
                        panel,
                        "Shields",
                        ship.shields,
                        DARK_GRAY.into(),
                        bar_width,
                        bar_height,
                        font_size,
                        &text_font,
                    );
                    spawn_bar(
                        panel,
                        "Weapons",
                        ship.weapons as f32 / 10.0,
                        Color::hsl(0.0, 0.75, 0.5),
                        bar_width,
                        bar_height,
                        font_size,
                        &text_font,
                    );
                });
        });
}

pub fn spawn_bar(
    parent: &mut bevy::ecs::hierarchy::ChildSpawnerCommands,
    label: &str,
    value: f32,
    color: Color,
    width: f32,
    height: f32,
    _font_size: f32, // remove unused warning
    text_font: &TextFont,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::from(DARK_GRAY).with_alpha(0.7)),
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new(label),
                text_font.clone(),
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(height),
                    ..default()
                },
            ));
            bar.spawn((
                Node {
                    width: Val::Px(width - 90.0),
                    height: Val::Px(height - 8.0),
                    margin: UiRect::left(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ))
            .with_children(|bar_bg| {
                bar_bg.spawn((
                    Node {
                        width: Val::Px((width - 90.0) * value.clamp(0.0, 1.0)),
                        height: Val::Px(height - 8.0),
                        ..default()
                    },
                    BackgroundColor(color),
                ));
            });
        });
}
