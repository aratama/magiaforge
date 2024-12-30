use bevy::prelude::*;
use serde::*;
use std::ops;

use crate::config::GameConfig;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Languages {
    Ja,
    En,
}

#[derive(Debug, Clone, Copy)]
pub struct Dict<T: ToString> {
    pub ja: T,
    pub en: T,
}

impl ops::Add<Dict<String>> for Dict<String> {
    type Output = Dict<String>;

    fn add(self, rhs: Dict<String>) -> Dict<String> {
        Dict {
            ja: format!("{}{}", self.ja, rhs.ja),
            en: format!("{}{}", self.en, rhs.en),
        }
    }
}

impl ops::AddAssign<Dict<String>> for Dict<String> {
    fn add_assign(&mut self, rhs: Dict<String>) {
        self.ja = format!("{}{}", self.ja, rhs.ja);
        self.en = format!("{}{}", self.en, rhs.en);
    }
}

impl Dict<&'static str> {
    pub fn get(&self, lang: Languages) -> String {
        match lang {
            Languages::Ja => self.ja.to_string(),
            Languages::En => self.en.to_string(),
        }
    }

    pub fn to_string(&self) -> Dict<String> {
        Dict {
            ja: self.ja.to_string(),
            en: self.en.to_string(),
        }
    }
}

impl Dict<String> {
    pub fn empty() -> Dict<String> {
        Dict {
            ja: "".to_string(),
            en: "".to_string(),
        }
    }

    pub fn literal<T: ToString>(str: T) -> Dict<String> {
        Dict {
            ja: str.to_string(),
            en: str.to_string(),
        }
    }

    pub fn get(&self, lang: Languages) -> String {
        match lang {
            Languages::Ja => self.ja.to_string(),
            Languages::En => self.en.to_string(),
        }
    }
}

#[derive(Component, Debug)]
#[require(Text)]
pub struct M18NTtext(pub Dict<String>);

impl M18NTtext {
    pub fn empty() -> Self {
        Self(Dict { ja: "", en: "" }.to_string())
    }
}

fn update_text(mut text_query: Query<(&mut Text, &M18NTtext)>, config: Res<GameConfig>) {
    if config.is_changed() || config.is_added() {
        for (mut text, m18n) in text_query.iter_mut() {
            text.0 = m18n.0.get(config.language);
        }
    }
}

fn update_text_on_change(
    mut text_query: Query<(&mut Text, &M18NTtext), Changed<M18NTtext>>,
    config: Res<GameConfig>,
) {
    for (mut text, m18n) in text_query.iter_mut() {
        text.0 = m18n.0.get(config.language);
    }
}

pub struct LanguagePlugin;

impl Plugin for LanguagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_text, update_text_on_change));
    }
}
