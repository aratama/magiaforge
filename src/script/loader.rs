use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::prelude::*;
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct JavaScriptSource {
    pub path: String,
    pub source: String,
}

#[derive(Default)]
pub struct JavaScriptLoader;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Could not read asset")]
    ReadError,
}

impl AssetLoader for JavaScriptLoader {
    type Asset = JavaScriptSource;
    type Settings = ();
    type Error = ReadError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut source = "".to_string();
        if let Err(err) = reader.read_to_string(&mut source).await {
            error!("[javascript_loader] error: {:?}", err);
            return Err(ReadError::ReadError);
        }

        Ok(JavaScriptSource {
            source,
            path: load_context
                .path()
                .as_os_str()
                .to_string_lossy()
                .to_string(),
        })
    }
    fn extensions(&self) -> &[&str] {
        &["js"]
    }
}

pub struct JavaScriptLoaderPlugin;

impl Plugin for JavaScriptLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<JavaScriptSource>();
        app.register_asset_loader(JavaScriptLoader);
    }
}
