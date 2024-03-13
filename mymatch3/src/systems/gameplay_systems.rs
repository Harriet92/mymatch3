use std::ffi::CString;
use bevy::{
    prelude::*
};
use rand::{Rng, thread_rng};

use crate::components::score_components;
use crate::components::gameplay_components;
use crate::components::gui_components::{AnimationIndices, AnimationTimer};
use crate::config::GameplayConfig;

static SPRITES: [&str; 8] = [
    "textures/Gameplay/Gems/Gem_sprite_sheet.png",
"textures/Gameplay/Gems/Gem_4x.png",
"textures/Gameplay/Gems/Gem_5x.png",
"textures/Gameplay/Gems/Gem_8x.png",
"textures/Gameplay/Gems/Gem_r4x.png",
"textures/Gameplay/Gems/Gem_xx.png",
"textures/Gameplay/Gems/Heart_xx.png",
"textures/Gameplay/Gems/Star_5x.png"];

pub fn test_gameplay(mut scoreboard: ResMut<score_components::Scoreboard>,
gameplay_config: Res<GameplayConfig>) {
    scoreboard.score += gameplay_config.increment;
}

pub fn spawn_board(mut commands: Commands, gameplay_config: Res<GameplayConfig>,
                   asset_server: Res<AssetServer>,
                   mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>){
    for x in 0..gameplay_config.board_size {
        for y in 0..gameplay_config.board_size {
            let random_tile_type = thread_rng().gen_range(0..gameplay_config.tile_types_count);
            spawn_tile(&mut commands, x, y, random_tile_type, &asset_server, &mut texture_atlas_layouts);
        }
    }
}

fn spawn_tile(mut commands: &mut Commands, x: usize, y: usize, tile_type: usize, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load(SPRITES[tile_type]);
    let layout = TextureAtlasLayout::from_grid(Vec2::new(512.0, 512.0), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 15 };
    const TILE_SIZE: f32 = 80.;
    commands.spawn((
                       gameplay_components::Tile::new( x, y, tile_type),
                       SpriteSheetBundle {
                           texture,
                           atlas: TextureAtlas {
                               layout: texture_atlas_layout,
                               index: animation_indices.first,
                           },
                           transform: Transform::from_xyz(x as f32 * TILE_SIZE - 400., y as f32 * TILE_SIZE - 300., 1.).with_scale(Vec3::new(0.15, 0.15, 0.3)),
                           ..default()
                       },
                       animation_indices,
                       AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
                   ));
}