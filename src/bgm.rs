use bevy::prelude::*;

use crate::asset::GameAssets;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum BGMType {
    Boubaku,
    Dokutsu,
    Saihate,
    Arechi,
    EndingBgm,
    Touha,
    Mori,
    Meikyu,
    Shiden,
    MidnightForest,
    Deamon,
    Action,
    Decisive,
    Enjin,
    Sacred,
    FinalBattle,
    HumanVsMachine,
}

impl BGMType {
    pub fn to_source(&self, assets: &Res<GameAssets>) -> Handle<AudioSource> {
        match self {
            BGMType::Boubaku => assets.boubaku.clone(),
            BGMType::Dokutsu => assets.dokutsu.clone(),
            BGMType::Saihate => assets.saihate.clone(),
            BGMType::Arechi => assets.arechi.clone(),
            BGMType::EndingBgm => assets.ending_bgm.clone(),
            BGMType::Touha => assets.touha.clone(),
            BGMType::Mori => assets.mori.clone(),
            BGMType::Meikyu => assets.meikyu.clone(),
            BGMType::Shiden => assets.shiden.clone(),
            BGMType::MidnightForest => assets.midnight_forest.clone(),
            BGMType::Deamon => assets.deamon.clone(),
            BGMType::Action => assets.action.clone(),
            BGMType::Decisive => assets.decisive.clone(),
            BGMType::Enjin => assets.enjin.clone(),
            BGMType::Sacred => assets.sacred.clone(),
            BGMType::FinalBattle => assets.final_battle.clone(),
            BGMType::HumanVsMachine => assets.human_vs_machine.clone(),
        }
    }
}
