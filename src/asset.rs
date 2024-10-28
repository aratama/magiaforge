use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "DotGothic16-Regular.ttf")]
    pub dotgothic: Handle<Font>,

    #[asset(path = "asset.aseprite")]
    pub asset: Handle<Aseprite>,

    #[asset(path = "level.aseprite")]
    pub level: Handle<Aseprite>,

    #[asset(path = "entity/player.aseprite")]
    pub player: Handle<Aseprite>,

    #[asset(path = "entity/slime.aseprite")]
    pub slime: Handle<Aseprite>,

    #[asset(path = "title.aseprite")]
    pub title: Handle<Aseprite>,
}
