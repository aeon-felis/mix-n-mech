use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, AssetCollectionApp};
use bevy_yoleck::YoleckLevelIndex;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<GameAssets>();
    }
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 1, rows = 2))]
    #[asset(path = "sprites/player.png")]
    pub player: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 1, rows = 1))]
    #[asset(path = "sprites/platform.png")]
    pub platform: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 1, rows = 2))]
    #[asset(path = "sprites/hover.png")]
    pub hover: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 1, rows = 2))]
    #[asset(path = "sprites/laser.png")]
    pub laser: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 1, rows = 2))]
    #[asset(path = "sprites/stationary.png")]
    pub stationary: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 1, rows = 2))]
    #[asset(path = "sprites/rotator.png")]
    pub rotator: Handle<TextureAtlas>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "levels/index.yoli")]
    pub level_index: Handle<YoleckLevelIndex>,
}
