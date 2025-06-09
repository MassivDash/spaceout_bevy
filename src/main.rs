use bevy::prelude::*;
mod base;
mod player;
use base::{Base, spawn_base};
use player::{Player, spawn_player};

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn((Camera2d, Transform::default()));
    // Player
    spawn_player(&mut commands, &mut meshes, &mut materials);
    // Base
    spawn_base(&mut commands, &mut meshes, &mut materials);
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
    let target_zoom = if out_of_bounds { 4.0 } else { 1.0 };
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
        .add_systems(Update, (move_player, camera_follow_and_zoom))
        .run();
}
