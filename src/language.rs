use crate::asset::GameAssets;
use crate::config::GameConfig;
use crate::states::GameState;
use bevy::prelude::*;
use serde::*;
use std::ops;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, strum::EnumIter, PartialEq, Eq)]
pub enum Languages {
    /// 日本語
    Ja,

    /// English
    En,

    /// 简体中文
    ZhCn,

    /// 繁体中文
    ZhTw,

    /// Español
    Es,

    /// Français
    Fr,

    /// Português
    Pt,

    /// Русский
    Ru,

    /// Deutsch
    De,

    /// 한국어
    Ko,
}

#[derive(Debug, Clone, Copy)]
pub struct Dict<T: ToString> {
    pub ja: T,
    pub en: T,
    pub zh_cn: T,
    pub zh_tw: T,
    pub es: T,
    pub fr: T,
    pub pt: T,
    pub ru: T,
    pub de: T,
    pub ko: T,
}

impl ops::Add<Dict<String>> for Dict<String> {
    type Output = Dict<String>;

    fn add(self, rhs: Dict<String>) -> Dict<String> {
        Dict {
            ja: format!("{}{}", self.ja, rhs.ja),
            en: format!("{}{}", self.en, rhs.en),
            zh_cn: format!("{}{}", self.zh_cn, rhs.zh_cn),
            zh_tw: format!("{}{}", self.zh_tw, rhs.zh_tw),
            es: format!("{}{}", self.es, rhs.es),
            fr: format!("{}{}", self.fr, rhs.fr),
            pt: format!("{}{}", self.pt, rhs.pt),
            ru: format!("{}{}", self.ru, rhs.ru),
            de: format!("{}{}", self.de, rhs.de),
            ko: format!("{}{}", self.ko, rhs.ko),
        }
    }
}

impl ops::AddAssign<Dict<String>> for Dict<String> {
    fn add_assign(&mut self, rhs: Dict<String>) {
        self.ja = format!("{}{}", self.ja, rhs.ja);
        self.en = format!("{}{}", self.en, rhs.en);
        self.zh_cn = format!("{}{}", self.zh_cn, rhs.zh_cn);
        self.zh_tw = format!("{}{}", self.zh_tw, rhs.zh_tw);
        self.es = format!("{}{}", self.es, rhs.es);
        self.fr = format!("{}{}", self.fr, rhs.fr);
        self.pt = format!("{}{}", self.pt, rhs.pt);
        self.ru = format!("{}{}", self.ru, rhs.ru);
        self.de = format!("{}{}", self.de, rhs.de);
        self.ko = format!("{}{}", self.ko, rhs.ko);
    }
}

impl Dict<&'static str> {
    pub fn get(&self, lang: Languages) -> String {
        match lang {
            Languages::Ja => self.ja.to_string(),
            Languages::En => self.en.to_string(),
            Languages::ZhCn => self.zh_cn.to_string(),
            Languages::ZhTw => self.zh_tw.to_string(),
            Languages::Es => self.es.to_string(),
            Languages::Fr => self.fr.to_string(),
            Languages::Pt => self.pt.to_string(),
            Languages::Ru => self.ru.to_string(),
            Languages::De => self.de.to_string(),
            Languages::Ko => self.ko.to_string(),
        }
    }

    pub fn to_string(&self) -> Dict<String> {
        Dict {
            ja: self.ja.to_string(),
            en: self.en.to_string(),
            zh_cn: self.zh_cn.to_string(),
            zh_tw: self.zh_tw.to_string(),
            es: self.es.to_string(),
            fr: self.fr.to_string(),
            pt: self.pt.to_string(),
            ru: self.ru.to_string(),
            de: self.de.to_string(),
            ko: self.ko.to_string(),
        }
    }
}

impl Dict<String> {
    pub fn empty() -> Dict<String> {
        Dict {
            ja: "".to_string(),
            en: "".to_string(),
            zh_cn: "".to_string(),
            zh_tw: "".to_string(),
            es: "".to_string(),
            fr: "".to_string(),
            pt: "".to_string(),
            ru: "".to_string(),
            de: "".to_string(),
            ko: "".to_string(),
        }
    }

    pub fn literal<T: ToString>(str: T) -> Dict<String> {
        Dict {
            ja: str.to_string(),
            en: str.to_string(),
            zh_cn: str.to_string(),
            zh_tw: str.to_string(),
            es: str.to_string(),
            fr: str.to_string(),
            pt: str.to_string(),
            ru: str.to_string(),
            de: str.to_string(),
            ko: str.to_string(),
        }
    }

    pub fn get(&self, lang: Languages) -> String {
        match lang {
            Languages::Ja => self.ja.to_string(),
            Languages::En => self.en.to_string(),
            Languages::ZhCn => self.zh_cn.to_string(),
            Languages::ZhTw => self.zh_tw.to_string(),
            Languages::Es => self.es.to_string(),
            Languages::Fr => self.fr.to_string(),
            Languages::Pt => self.pt.to_string(),
            Languages::Ru => self.ru.to_string(),
            Languages::De => self.de.to_string(),
            Languages::Ko => self.ko.to_string(),
        }
    }
}

#[derive(Component, Debug)]
#[require(Text)]
pub struct M18NTtext(pub Dict<String>);

impl M18NTtext {
    pub fn empty() -> Self {
        Self(Dict::empty())
    }
}

fn update_text_on_change_config(
    mut text_query: Query<(&mut Text, &mut TextFont, &M18NTtext)>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
) {
    if config.is_changed() || config.is_added() {
        update_text_and_font(&mut text_query, &config, &assets);
    }
}

fn update_text_on_change_text(
    mut text_query: Query<(&mut Text, &mut TextFont, &M18NTtext)>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
) {
    update_text_and_font(&mut text_query, &config, &assets);
}

fn update_text_and_font(
    text_query: &mut Query<(&mut Text, &mut TextFont, &M18NTtext)>,
    config: &Res<GameConfig>,
    assets: &Res<GameAssets>,
) {
    for (mut text, mut font, m18n) in text_query.iter_mut() {
        text.0 = m18n.0.get(config.language);
        font.font = language_to_font(assets, config.language);
    }
}

pub fn language_to_font(assets: &GameAssets, language: Languages) -> Handle<Font> {
    match language {
        Languages::Ja => assets.noto_sans_jp.clone(),
        Languages::En => assets.noto_sans_jp.clone(),
        Languages::ZhCn => assets.noto_sans_sc.clone(),
        Languages::ZhTw => assets.noto_sans_tc.clone(),
        Languages::Es => assets.noto_sans_jp.clone(),
        Languages::Fr => assets.noto_sans_jp.clone(),
        Languages::Pt => assets.noto_sans_jp.clone(),
        Languages::Ru => assets.noto_sans_jp.clone(),
        Languages::De => assets.noto_sans_jp.clone(),
        Languages::Ko => assets.noto_sans_kr.clone(),
    }
}

pub struct LanguagePlugin;

impl Plugin for LanguagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_text_on_change_config, update_text_on_change_text).run_if(
                in_state(GameState::InGame)
                    .or(in_state(GameState::MainMenu))
                    .or(in_state(GameState::NameInput))
                    .or(in_state(GameState::Ending)),
            ),
        );
    }
}
