use bevy::prelude::*;

mod docked;
mod menu;
mod planets;
mod ship;
mod space;
mod splash;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Space,
    Docked,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(space::SunDamageWarning::default()) // <-- Add this line
        .add_systems(Startup, setup_camera)
        .add_plugins((
            splash::splash_plugin,
            menu::menu_plugin,
            space::space_plugin,
            // docked::docked_plugin,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
