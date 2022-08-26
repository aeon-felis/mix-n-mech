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
    #[asset(path = "sprites/player.png")]
    pub player: Handle<Image>,
    // #[asset(path = "sprites/hands.png")]
    // pub hands: Handle<Image>,
    // #[asset(path = "sprites/zombie.png")]
    // pub zombie: Handle<Image>,
    // #[asset(path = "sprites/wifi.png")]
    // pub wifi: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 2, rows = 1))]
    // #[asset(path = "sprites/door.png")]
    // pub door: Handle<TextureAtlas>,
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "levels/index.yoli")]
    pub level_index: Handle<YoleckLevelIndex>,
}
