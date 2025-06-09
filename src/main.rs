use bevy::prelude::*;
mod base;
mod player;
use base::{Base, animate_base, spawn_base};
use player::{Player, spawn_player};
use rand::Rng;

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
        Query<&Transform, With<Player>>,
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
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_starfield(&mut commands);
    // Camera
    commands.spawn((Camera2d, Transform::default()));
    // Player
    spawn_player(&mut commands, &asset_server, &mut meshes, &mut materials);
    // Base (earth sprite)
    spawn_base(&mut commands, &asset_server, atlas_layouts);
}

// System to control the player triangle with rotation and throttle
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player_query.single_mut().unwrap();
    let mut rotation_delta = 0.0;
    let mut throttle_delta = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_delta += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_delta -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        throttle_delta += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        throttle_delta -= 1.0;
    }
    // Update rotation (tilt)
    let rotation_speed = std::f32::consts::PI; // radians/sec
    transform.rotation = transform.rotation
        * Quat::from_rotation_z(rotation_delta * rotation_speed * time.delta_secs());
    // Update throttle
    player.throttle =
        (player.throttle + throttle_delta * 200.0 * time.delta_secs()).clamp(0.0, 800.0);
    // Move in the direction the triangle is facing
    let forward = transform.rotation * Vec3::Y;
    transform.translation += forward * player.throttle * time.delta_secs();
}

// Camera follow and zoom logic
fn camera_follow_and_zoom(
    mut q: ParamSet<(
        Query<&Transform, With<Player>>,
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                parallax_starfield,
                move_player,
                camera_follow_and_zoom,
                animate_base,
            ),
        )
        .run();
}
