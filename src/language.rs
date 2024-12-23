use serde::*;

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
    pub fn get(&self, lang: Languages) -> String {
        match lang {
            Languages::Ja => self.ja.to_string(),
            Languages::En => self.en.to_string(),
        }
    }
}
