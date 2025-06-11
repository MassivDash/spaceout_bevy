use bevy::prelude::*;

#[derive(Component)]
pub struct Base;

pub fn spawn_base(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    _atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, // No longer needed
) {
    // Load the PNG image for the base
    let texture_handle = asset_server.load("earth.png"); // Use your PNG file name here

    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(0.0, 0.0, -1.0)),
        Base,
    ));
}
