use bevy::asset::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "asset.aseprite")]
    pub asset: Handle<Aseprite>,

    #[asset(path = "level.aseprite")]
    pub level: Handle<Aseprite>,

    #[asset(path = "tile.png")]
    pub tile: Handle<Image>,

    #[asset(path = "player.aseprite")]
    pub player: Handle<Aseprite>,

    #[asset(path = "slime.aseprite")]
    pub slime: Handle<Aseprite>,

    #[asset(path = "dageki.ogg")]
    pub dageki: Handle<AudioSource>,

    #[asset(path = "shibafu.ogg")]
    pub shibafu: Handle<AudioSource>,

    #[asset(path = "hiyoko.ogg")]
    pub hiyoko: Handle<AudioSource>,

    #[asset(path = "title.png")]
    pub title: Handle<Image>,
}
