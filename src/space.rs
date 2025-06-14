use crate::GameState;
use crate::planets::base::{Base, spawn_base};
use crate::planets::moon::{Moon, spawn_moon};
use crate::planets::sun::{Sun, rotate_sun, spawn_sun};
use crate::ship::action_menu::{
    ActionMenuTarget, action_menu_button_system, show_action_menu_system,
};
use crate::ship::movement::move_spaceship;
use crate::ship::spaceship::{Spaceship, spawn_spaceship};
use crate::ship::ui::spaceship_ui_panel;
use bevy::prelude::*;
use rand::Rng;

const STAR_LAYERS: usize = 3;
const STARS_PER_LAYER: usize = 100;
const STAR_COLORS: [Color; STAR_LAYERS] = [
    Color::BLACK,
    Color::srgb(0.2, 0.2, 0.2),
    Color::srgb(0.5, 0.5, 0.5),
];
const STAR_PARALLAX: [f32; STAR_LAYERS] = [0.2, 0.5, 0.8];

#[derive(Component)]
struct Star {
    layer: usize,
    base_pos: Vec2,
}

#[derive(Component)]
struct OnSpaceScreen;

pub fn space_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Space), setup_space)
        .add_systems(
            Update,
            (
                parallax_starfield,
                move_spaceship,
                camera_follow_and_zoom.after(parallax_starfield), // <-- add .after here
                spaceship_ui_panel,
                refuel_on_base_visit,
                rotate_sun,
                sun_proximity_damage,
                show_action_menu_system,
                action_menu_button_system,
            )
                .run_if(in_state(GameState::Space)),
        )
        .add_systems(OnExit(GameState::Space), despawn_space_entities);
}

fn setup_space(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_starfield(&mut commands);
    // Camera
    commands.spawn((Camera2d, Transform::default(), OnSpaceScreen));
    // Spaceship
    spawn_spaceship(&mut commands, &asset_server, &mut meshes, &mut materials);
    // Base (earth sprite)
    let base_entity = spawn_base(&mut commands, &asset_server, atlas_layouts);
    commands.entity(base_entity).insert(ActionMenuTarget {
        label: "Base".to_string(),
    });
    // Moon (moon sprite)
    let moon_entity = spawn_moon(&mut commands, &asset_server);
    commands.entity(moon_entity).insert(ActionMenuTarget {
        label: "Moon".to_string(),
    });
    // Sun (sun sprite)
    spawn_sun(&mut commands, &asset_server);
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
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(x, y, -100.0 - layer as f32)),
                Star {
                    layer,
                    base_pos: Vec2::new(x, y),
                },
                OnSpaceScreen,
            ));
        }
    }
}

fn parallax_starfield(
    mut param_set: ParamSet<(
        Query<&Transform, With<Spaceship>>,
        Query<(&Star, &mut Transform)>,
    )>,
) {
    if let Ok(player_transform) = param_set.p0().single() {
        let player_pos = player_transform.translation.truncate();
        for (star, mut transform) in param_set.p1().iter_mut() {
            let parallax = STAR_PARALLAX[star.layer];
            let offset = player_pos * (1.0 - parallax);
            transform.translation.x = star.base_pos.x + offset.x;
            transform.translation.y = star.base_pos.y + offset.y;
        }
    }
}

fn refuel_on_base_visit(
    mut ship_query: Query<(&mut Spaceship, &Transform)>,
    base_query: Query<&Transform, With<Base>>,
    moon_query: Query<&Transform, With<Moon>>,
) {
    if let (Ok((mut ship, ship_transform)), Ok(base_transform), Ok(moon_transform)) = (
        ship_query.single_mut(),
        base_query.single(),
        moon_query.single(),
    ) {
        let ship_pos = ship_transform.translation.truncate();
        let base_pos = base_transform.translation.truncate();
        let moon_pos = moon_transform.translation.truncate();
        if ship_pos.distance(base_pos) < 150.0 || ship_pos.distance(moon_pos) < 100.0 {
            ship.fuel = 1.0;
        }
    }
}

fn camera_follow_and_zoom(
    mut param_set: ParamSet<(
        Query<&Transform, With<Spaceship>>,
        Query<&Transform, With<Base>>,
        Query<&mut Transform, With<Camera2d>>,
    )>,
) {
    // Get player position
    let player_pos = if let Ok(player_transform) = param_set.p0().single() {
        player_transform.translation.truncate()
    } else {
        return;
    };

    // Get base position
    let base_pos = if let Ok(base_transform) = param_set.p1().single() {
        base_transform.translation.truncate()
    } else {
        return;
    };

    let out_of_bounds = (player_pos - base_pos).length() > 400.0;
    let target_zoom = if out_of_bounds { 6.0 } else { 2.0 };
    let zoom_speed = 5.0;
    for mut cam_transform in param_set.p2().iter_mut() {
        let current_zoom = cam_transform.scale.x;
        let new_zoom = current_zoom + (target_zoom - current_zoom) * zoom_speed * 0.016;
        cam_transform.translation.x = player_pos.x;
        cam_transform.translation.y = player_pos.y;
        cam_transform.scale = Vec3::splat(new_zoom);
    }
}

fn sun_proximity_damage(
    mut ship_query: Query<(&mut Spaceship, &Transform)>,
    sun_query: Query<&Transform, With<Sun>>,
    time: Res<Time>,
    sun_damage_warning: Option<ResMut<crate::space::SunDamageWarning>>,
) {
    if let (Ok((mut ship, ship_transform)), Ok(sun_transform), Some(mut warning)) = (
        ship_query.single_mut(),
        sun_query.single(),
        sun_damage_warning,
    ) {
        let ship_pos = ship_transform.translation.truncate();
        let sun_pos = sun_transform.translation.truncate();
        let dist = ship_pos.distance(sun_pos);
        let damage_radius = 600.0;
        let damage_per_sec = 0.25;
        if dist < damage_radius {
            ship.hull = (ship.hull - damage_per_sec * time.delta().as_secs_f32()).max(0.0);
            warning.0 = true;
        } else {
            warning.0 = false;
        }
    }
}

#[derive(Resource, Default)]
pub struct SunDamageWarning(pub bool);

fn despawn_space_entities(mut commands: Commands, q: Query<Entity, With<OnSpaceScreen>>) {
    for entity in &q {
        commands.entity(entity).despawn();
    }
}
