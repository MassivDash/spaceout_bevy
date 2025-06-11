use crate::ship::spaceship::Spaceship;
use bevy::prelude::*;

pub fn move_spaceship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Spaceship)>,
    time: Res<Time>,
) {
    let (mut transform, mut ship) = query.single_mut().unwrap();
    let mut rotation_delta = 0.0;
    let mut speed_delta = 0.0;
    let mut burning_fuel = false;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_delta += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_delta -= 1.0;
    }
    // Only allow speed increase if ArrowUp is pressed and fuel > 0
    if keyboard_input.pressed(KeyCode::ArrowUp) && ship.fuel > 0.0 {
        speed_delta += 1.0;
        burning_fuel = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        speed_delta -= 1.0;
    }
    // Update rotation (tilt)
    let rotation_speed = std::f32::consts::PI; // radians/sec
    transform.rotation = transform.rotation
        * Quat::from_rotation_z(rotation_delta * rotation_speed * time.delta_secs());
    // Update speed (no upper limit)
    if speed_delta > 0.0 && ship.fuel > 0.0 {
        ship.throttle += speed_delta * 200.0 * time.delta_secs();
    } else if speed_delta < 0.0 {
        ship.throttle = (ship.throttle + speed_delta * 200.0 * time.delta_secs()).max(0.0);
    }
    // Burn fuel if accelerating (ArrowUp and fuel > 0)
    if burning_fuel && ship.throttle > 0.0 {
        let burn = 0.25 * ship.throttle / 800.0 * time.delta_secs();
        ship.fuel = (ship.fuel - burn).max(0.0);
    }
    // Move in the direction the spaceship is facing
    let forward = transform.rotation * Vec3::Y;
    transform.translation += forward * ship.throttle * time.delta_secs();
}
