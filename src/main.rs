use bevy::color::palettes::css::{DARK_CYAN, DARK_GRAY, YELLOW};
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use rand::Rng;

mod base;
mod player;
use base::{animate_base, spawn_base};
use player::{Spaceship, spawn_spaceship};

use crate::base::Base;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const STAR_LAYERS: usize = 3;
const STARS_PER_LAYER: usize = 100;
const STAR_COLORS: [Color; STAR_LAYERS] = [
    Color::BLACK,
    Color::srgb(0.2, 0.2, 0.2), // dark gray
    Color::srgb(0.5, 0.5, 0.5), // gray
];
const STAR_PARALLAX: [f32; STAR_LAYERS] = [0.2, 0.5, 0.8];

#[derive(Component)]
struct Star {
    layer: usize,
    base_pos: Vec2,
}

fn spawn_starfield(commands: &mut Commands) {
    let mut rng = rand::rng();
    for layer in 0..STAR_LAYERS {
        for _ in 0..STARS_PER_LAYER {
            let x = rng.random_range(-2000.0..2000.0);
            let y = rng.random_range(-2000.0..2000.0);
            let size = rng.random_range(1.0..3.0) * (layer as f32 + 1.0);
            commands.spawn((
                Sprite {
                    color: STAR_COLORS[layer],
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(x, y, -100.0 - layer as f32)),
                Star {
                    layer,
                    base_pos: Vec2::new(x, y),
                },
            ));
        }
    }
}

fn parallax_starfield(
    mut q: ParamSet<(
        Query<&Transform, With<Spaceship>>,
        Query<(&Star, &mut Transform)>,
    )>,
) {
    let player_pos = q.p0().single().unwrap().translation.truncate();
    for (star, mut transform) in &mut q.p1() {
        let parallax = STAR_PARALLAX[star.layer];
        let offset = player_pos * (1.0 - parallax);
        transform.translation.x = star.base_pos.x + offset.x;
        transform.translation.y = star.base_pos.y + offset.y;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, // not mut
) {
    spawn_starfield(&mut commands);
    // Camera
    commands.spawn((Camera2d, Transform::default()));
    // Spaceship
    spawn_spaceship(&mut commands, &asset_server, &mut meshes, &mut materials);
    // Base (earth sprite)
    spawn_base(&mut commands, &asset_server, atlas_layouts);
}

// System to control the player triangle with rotation and throttle
fn move_spaceship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Spaceship)>,
    time: Res<Time>,
) {
    let (mut transform, mut ship) = query.single_mut().unwrap();
    let mut rotation_delta = 0.0;
    let mut throttle_delta = 0.0;
    let mut burning_fuel = false;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_delta += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_delta -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) && ship.fuel > 0.0 {
        throttle_delta += 1.0;
        burning_fuel = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        throttle_delta -= 1.0;
    }
    // Update rotation (tilt)
    let rotation_speed = std::f32::consts::PI; // radians/sec
    transform.rotation = transform.rotation
        * Quat::from_rotation_z(rotation_delta * rotation_speed * time.delta_secs());
    // Update throttle
    let prev_throttle = ship.throttle;
    ship.throttle = (ship.throttle + throttle_delta * 200.0 * time.delta_secs()).clamp(0.0, 800.0);
    // Burn fuel if accelerating
    if burning_fuel && ship.throttle > 0.0 {
        let burn = 0.25 * ship.throttle / 800.0 * time.delta_secs();
        ship.fuel = (ship.fuel - burn).max(0.0);
        // If fuel runs out, throttle drops to 0
        if ship.fuel <= 0.0 {
            ship.throttle = 0.0;
        }
    }
    // Move in the direction the spaceship is facing
    let forward = transform.rotation * Vec3::Y;
    transform.translation += forward * ship.throttle * time.delta_secs();
}

// System to refuel when visiting the base
fn refuel_on_base_visit(
    mut ship_query: Query<(&mut Spaceship, &Transform)>,
    base_query: Query<&Transform, With<Base>>,
) {
    let (mut ship, ship_transform) = ship_query.single_mut().unwrap();
    let base_transform = base_query.single().unwrap();
    let dist = ship_transform
        .translation
        .truncate()
        .distance(base_transform.translation.truncate());
    if dist < 80.0 {
        ship.fuel = 1.0;
    }
}

// Camera follow and zoom logic
fn camera_follow_and_zoom(
    mut q: ParamSet<(
        Query<&Transform, With<Spaceship>>,
        Query<&Transform, With<Base>>,
        Query<&mut Transform, With<Camera2d>>,
    )>,
) {
    let player_pos = q.p0().single().unwrap().translation.truncate();
    let base_pos = q.p1().single().unwrap().translation.truncate();
    let out_of_bounds = (player_pos - base_pos).length() > 400.0;
    let target_zoom = if out_of_bounds { 6.0 } else { 2.0 };
    let zoom_speed = 5.0; // Higher is snappier, lower is smoother
    for mut cam_transform in q.p2().iter_mut() {
        // Smoothly interpolate the zoom
        let current_zoom = cam_transform.scale.x;
        let new_zoom = current_zoom + (target_zoom - current_zoom) * zoom_speed * 0.016; // 0.016 ~ 60fps
        cam_transform.translation.x = player_pos.x;
        cam_transform.translation.y = player_pos.y;
        cam_transform.scale = Vec3::splat(new_zoom);
    }
}

#[derive(Component)]
struct SidePanelRoot;

// Fix: Accept the correct parent type for .with_children closure (ChildSpawnerCommands)
fn spaceship_ui_panel(
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
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(margin),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.7)),
            SidePanelRoot,
        ))
        .with_children(|parent: &mut ChildSpawnerCommands| {
            spawn_bar(
                parent,
                "Fuel",
                ship.fuel,
                YELLOW.into(),
                bar_width,
                bar_height,
                font_size,
                &text_font,
            );
            spawn_bar(
                parent,
                "Hull",
                ship.hull,
                DARK_CYAN.into(),
                bar_width,
                bar_height,
                font_size,
                &text_font,
            );
            spawn_bar(
                parent,
                "Shields",
                ship.shields,
                DARK_GRAY.into(),
                bar_width,
                bar_height,
                font_size,
                &text_font,
            );
            spawn_bar(
                parent,
                "Weapons",
                ship.weapons as f32 / 10.0,
                Color::hsl(0.0, 0.75, 0.5),
                bar_width,
                bar_height,
                font_size,
                &text_font,
            );
        });
}

// Update spawn_bar to accept ChildSpawnerCommands
fn spawn_bar(
    parent: &mut ChildSpawnerCommands,
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                parallax_starfield,
                move_spaceship,
                camera_follow_and_zoom,
                animate_base,
                spaceship_ui_panel,
                refuel_on_base_visit,
            ),
        )
        .run();
}
