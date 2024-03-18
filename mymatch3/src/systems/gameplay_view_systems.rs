use bevy::{
    prelude::*
};

use crate::components::gameplay_components;
use crate::components::view_components::{AnimationIndices, AnimationTimer};
use crate::config::GameplayViewConfig;

static SPRITES: [&str; 8] = [
    "textures/Gameplay/Gems/Gem_sprite_sheet.png",
"textures/Gameplay/Gems/Gem_4x.png",
"textures/Gameplay/Gems/Gem_5x.png",
"textures/Gameplay/Gems/Gem_8x.png",
"textures/Gameplay/Gems/Gem_r4x.png",
"textures/Gameplay/Gems/Gem_xx.png",
"textures/Gameplay/Gems/Heart_xx.png",
"textures/Gameplay/Gems/Star_5x.png"];

//pub fn load_sprite_atlasses()
//TODO: Loading sprites at start

pub fn spawn_tile_images(mut query: Query<(Entity, &gameplay_components::Tile, &gameplay_components::NeedsView)>,
                     mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     gameplay_view_config: Res<GameplayViewConfig>,
                     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    for (e, mut tile, _needs_view) in &mut query {
        commands.entity(e).remove::<gameplay_components::NeedsView>();

        let texture = asset_server.load(SPRITES[tile.tile_type]);
        let layout = TextureAtlasLayout::from_grid(Vec2::splat(gameplay_view_config.sprite_size), 4, 4, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 15 };

        let x_pos = tile.x as f32 * gameplay_view_config.tile_size - gameplay_view_config.translation;
        let y_pos = tile.y as f32 * gameplay_view_config.tile_size - gameplay_view_config.translation;

        commands.spawn((
            gameplay_components::Tile::new(tile.x, tile.y, tile.tile_type),
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_xyz(x_pos, y_pos, 1.).with_scale(Vec3::splat(gameplay_view_config.sprite_scale)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        ));
    }
}