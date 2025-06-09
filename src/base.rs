use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Base;

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }
    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(5.0 / (fps as f32)),
            TimerMode::Repeating,
        )
    }
}

pub fn spawn_base(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Correct asset path: relative to assets root, no leading slash
    let texture_handle = asset_server.load("earthspin-sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(48, 48), 10, 10, None, None);
    let layout_handle = atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 93, 24); // 94 frames, 24 FPS
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: animation_config.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(0.0, 0.0, -1.0)),
        Base,
        animation_config,
    ));
}

// Animate the earth base
pub fn animate_base(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Base>>,
) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                if atlas.index >= config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
