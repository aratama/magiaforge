use super::generated::EntityInstance;
use super::generated::LayerDefinition;
use super::generated::LayerInstance;
use super::generated::Level;
use crate::actor::ActorType;
use crate::language::Dict;
use crate::level::world::GameLevel;
use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

pub struct RawLDTKLoaderPlugin;

impl Plugin for RawLDTKLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LDTK>();
        app.register_asset_loader(RawLDTKLoader);
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct LDTK {
    pub coordinate: Arc<crate::ldtk::generated::Coordinate>,
}

impl LDTK {
    pub fn get_level(&self, level: &GameLevel) -> Option<&Level> {
        self.coordinate
            .levels
            .iter()
            .find(|l| l.identifier == level.0)
    }

    #[allow(dead_code)]
    pub fn get_level_by_position(&self, position: Vec2) -> Option<&Level> {
        self.coordinate.levels.iter().find(|l| l.contains(position))
    }

    pub fn get_level_by_iid(&self, iid: &String) -> Option<&Level> {
        self.coordinate.levels.iter().find(|l| l.iid == *iid)
    }

    pub fn get_levels(&self) -> core::slice::Iter<Level> {
        self.coordinate.levels.iter()
    }

    pub fn get_layer_definition(&self, layer_name: &str) -> Option<&LayerDefinition> {
        self.coordinate
            .defs
            .layers
            .iter()
            .find(|l| l.identifier == layer_name)
            .clone()
    }

    pub fn get_tile_mapping(&self, layer_name: &str) -> HashMap<i64, &str> {
        let definition = self.get_layer_definition(layer_name).unwrap();
        definition
            .int_grid_values
            .iter()
            .map(|v| (v.value, v.identifier.as_ref().unwrap().as_str()))
            .collect()
    }

    pub fn get_neighbors(&self, game_level: &GameLevel) -> Vec<GameLevel> {
        let level = self.get_level(game_level).unwrap();
        level
            .neighbours
            .iter()
            .map(|neighbor| {
                let identifier = &self
                    .get_level_by_iid(&neighbor.level_iid)
                    .unwrap()
                    .identifier;
                GameLevel(identifier.clone())
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn find_entity_by_id(&self, entity_iid: &str) -> Option<(String, EntityInstance)> {
        for level in self.coordinate.levels.iter() {
            for layer in level.layer_instances.as_ref().unwrap_or(&vec![]).iter() {
                for entity in layer.entity_instances.iter() {
                    if entity.iid == entity_iid {
                        return Some((level.identifier.clone(), entity.clone()));
                    }
                }
            }
        }
        None
    }
}

impl Level {
    #[allow(dead_code)]
    pub fn get_field_as_string(&self, field_name: &str) -> String {
        self.field_instances
            .iter()
            .find(|f| f.identifier == field_name)
            .as_ref()
            .expect(format!("Field {:?} not found", field_name).as_str())
            .value
            .as_ref()
            .expect(format!("Field {:?} not found", field_name).as_str())
            .as_str()
            .expect("Error occurred in as_str")
            .to_string()
    }

    pub fn get_layer(&self, layer_name: &str) -> Option<&LayerInstance> {
        self.layer_instances
            .as_ref()
            .expect(format!("Error occurred in get_layer for {:?}", layer_name).as_str())
            .iter()
            .find(|l| l.identifier == layer_name)
    }

    pub fn contains(&self, position: Vec2) -> bool {
        let level_rect = Rect::new(
            self.world_x as f32,
            self.world_y as f32,
            self.px_wid as f32,
            self.px_hei as f32,
        );
        level_rect.contains(position)
    }
}

#[derive(Default)]
pub struct RawLDTKLoader;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Could not load asset")]
    #[allow(dead_code)]
    ReadError,
}

impl AssetLoader for RawLDTKLoader {
    type Asset = LDTK;
    type Settings = ();
    type Error = ReadError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let _ = _load_context;

        let mut buf = String::new();
        let output = reader.read_to_string(&mut buf).await;
        if output.is_err() {
            return Err(ReadError::ReadError);
        }
        let coordinate = Arc::new(serde_json::from_str(&buf).unwrap());
        Ok(LDTK { coordinate })
    }
    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

#[derive(serde::Deserialize, Default)]
pub struct LevelCustomFields {
    pub bgm: String,
    pub ja: String,
    pub en: String,
    pub zh_cn: String,
    pub zh_tw: String,
    pub es: String,
    pub fr: String,
    pub pt: String,
    pub de: String,
    pub ko: String,
    pub ru: String,
    pub enemies: u8,
    pub enemy_types: Vec<String>,
    pub brightness: f32,
}

impl LevelCustomFields {
    pub fn get_name(&self) -> Dict<String> {
        Dict {
            ja: self.ja.clone(),
            en: self.en.clone(),
            zh_cn: self.zh_cn.clone(),
            zh_tw: self.zh_tw.clone(),
            es: self.es.clone(),
            fr: self.fr.clone(),
            pt: self.pt.clone(),
            de: self.de.clone(),
            ko: self.ko.clone(),
            ru: self.ru.clone(),
        }
    }

    pub fn get_enemy_types(&self) -> Vec<ActorType> {
        self.enemy_types.iter().map(|t| ActorType::new(t)).collect()
    }
}
