use serde::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Languages {
    Ja,
    En,
}

impl Languages {
    pub fn m17n(&self, ja: String, en: String) -> String {
        match self {
            Languages::Ja => ja,
            Languages::En => en,
        }
    }
}

#[derive(Debug)]
pub struct Dict {
    pub ja: &'static str,
    pub en: &'static str,
}

impl Dict {
    pub fn get(&self, lang: Languages) -> &str {
        match lang {
            Languages::Ja => self.ja,
            Languages::En => self.en,
        }
    }
}
