use crate::asset::GameAssets;
use crate::config::GameConfig;
use bevy::prelude::*;
use serde::*;
use std::ops;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Languages {
    /// 日本語
    Ja,

    /// English
    En,

    /// 简体中文
    ZhCn,

    /// español
    Es,

    /// Français
    Fr,
}

#[derive(Debug, Clone, Copy)]
pub struct Dict<T: ToString> {
    pub ja: T,
    pub en: T,
    pub zh_cn: T,
    pub es: T,
    pub fr: T,
}

impl ops::Add<Dict<String>> for Dict<String> {
    type Output = Dict<String>;

    fn add(self, rhs: Dict<String>) -> Dict<String> {
        Dict {
            ja: format!("{}{}", self.ja, rhs.ja),
            en: format!("{}{}", self.en, rhs.en),
            zh_cn: format!("{}{}", self.zh_cn, rhs.zh_cn),
            es: format!("{}{}", self.es, rhs.es),
            fr: format!("{}{}", self.fr, rhs.fr),
        }
    }
}

impl ops::AddAssign<Dict<String>> for Dict<String> {
    fn add_assign(&mut self, rhs: Dict<String>) {
        self.ja = format!("{}{}", self.ja, rhs.ja);
        self.en = format!("{}{}", self.en, rhs.en);
        self.zh_cn = format!("{}{}", self.zh_cn, rhs.zh_cn);
        self.es = format!("{}{}", self.es, rhs.es);
        self.fr = format!("{}{}", self.fr, rhs.fr);
    }
}

impl Dict<&'static str> {
    pub fn get(&self, lang: Languages) -> String {
        match lang {
            Languages::Ja => self.ja.to_string(),
            Languages::En => self.en.to_string(),
            Languages::ZhCn => self.zh_cn.to_string(),
            Languages::Es => self.es.to_string(),
            Languages::Fr => self.fr.to_string(),
        }
    }

    pub fn to_string(&self) -> Dict<String> {
        Dict {
            ja: self.ja.to_string(),
            en: self.en.to_string(),
            zh_cn: self.zh_cn.to_string(),
            es: self.es.to_string(),
            fr: self.fr.to_string(),
        }
    }
}

impl Dict<String> {
    pub fn empty() -> Dict<String> {
        Dict {
            ja: "".to_string(),
            en: "".to_string(),
            zh_cn: "".to_string(),
            es: "".to_string(),
            fr: "".to_string(),
        }
    }

    pub fn literal<T: ToString>(str: T) -> Dict<String> {
        Dict {
            ja: str.to_string(),
            en: str.to_string(),
            zh_cn: str.to_string(),
            es: str.to_string(),
            fr: str.to_string(),
        }
    }

    pub fn get(&self, lang: Languages) -> String {
        match lang {
            Languages::Ja => self.ja.to_string(),
            Languages::En => self.en.to_string(),
            Languages::ZhCn => self.zh_cn.to_string(),
            Languages::Es => self.es.to_string(),
            Languages::Fr => self.fr.to_string(),
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
        update_text(&mut text_query, &config, &assets);
    }
}

fn update_text_on_change_text(
    mut text_query: Query<(&mut Text, &mut TextFont, &M18NTtext)>,
    config: Res<GameConfig>,
    assets: Res<GameAssets>,
) {
    update_text(&mut text_query, &config, &assets);
}

fn update_text(
    text_query: &mut Query<(&mut Text, &mut TextFont, &M18NTtext)>,
    config: &Res<GameConfig>,
    assets: &Res<GameAssets>,
) {
    for (mut text, mut font, m18n) in text_query.iter_mut() {
        text.0 = m18n.0.get(config.language);
        font.font = language_to_font(assets, config);
    }
}

pub fn language_to_font(assets: &GameAssets, config: &GameConfig) -> Handle<Font> {
    match config.language {
        Languages::Ja => assets.noto_sans_jp.clone(),
        Languages::En => assets.noto_sans_jp.clone(),
        Languages::ZhCn => assets.noto_sans_sc.clone(),
        Languages::Es => assets.noto_sans_jp.clone(),
        Languages::Fr => assets.noto_sans_jp.clone(),
    }
}

pub struct LanguagePlugin;

impl Plugin for LanguagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_text_on_change_config, update_text_on_change_text),
        );
    }
}
