use bevy::{
    prelude::*
};

use crate::components::gameplay_components;
use crate::components::view_components::{AnimationIndices, AnimationTimer};

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
                     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    for (e, mut tile, _needs_view) in &mut query {
        commands.entity(e).remove::<gameplay_components::NeedsView>();

        let texture = asset_server.load(SPRITES[tile.tile_type]);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(512.0, 512.0), 4, 4, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 15 };
        const TILE_SIZE: f32 = 80.;

        commands.spawn((
            gameplay_components::Tile::new(tile.x, tile.y, tile.tile_type),
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_xyz(tile.x as f32 * TILE_SIZE - 400., tile.y as f32 * TILE_SIZE - 300., 1.).with_scale(Vec3::new(0.15, 0.15, 0.3)),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        ));
    }
}