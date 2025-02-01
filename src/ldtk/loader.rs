use super::generated::{EntityInstance, LayerDefinition, LayerInstance, Level};
use crate::page::in_game::GameLevel;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt},
    prelude::*,
};
use std::{collections::HashMap, sync::Arc};
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
}

impl Level {
    pub fn get_layer(&self, layer_name: &str) -> Option<&LayerInstance> {
        self.layer_instances
            .as_ref()
            .unwrap()
            .iter()
            .find(|l| l.identifier == layer_name)
    }
}

impl EntityInstance {
    pub fn get_value_as_str(&self, identifier: &str) -> String {
        self.field_instances
            .iter()
            .find(|f| f.identifier == identifier)
            .as_ref()
            .expect(format!("{:?} not found", identifier).as_str())
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
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
