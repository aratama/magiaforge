// 使われていません

use aseprite_loader::loader::AsepriteFile;
use bevy::{
    asset::{io::Reader, AssetLoader, RenderAssetUsages},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use std::sync::Arc;
use thiserror::Error;

pub struct RawAsepriteLoaderPlugin;

impl Plugin for RawAsepriteLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<RawAseprite>();
        app.register_asset_loader(RawAsepriteLoader);
    }
}

//@todo: if this can be serialized, we basicly have a intermediate binary
//represantion and can make use of the asset prepocessor. No longer need
//to ship or bundle aseprite binaries into your release.
#[derive(Asset, TypePath, Debug)]
pub struct RawAseprite {
    #[allow(dead_code)]
    pub aseprite: Arc<AsepriteFile<'static>>,
}

impl RawAseprite {
    #[allow(dead_code)]
    pub fn get_layer_by_name(&self, name: &str, frame_index: usize) -> Option<Image> {
        let Some(first_frame) = self.aseprite.frames.get(frame_index) else {
            return None;
        };
        let Some(cel) = first_frame.cels.iter().find(|cel| {
            let layer = self.aseprite.layers.get(cel.layer_index).unwrap();
            layer.name == name
        }) else {
            return None;
        };

        let compressed_image = &self.aseprite.images[cel.image_index];

        let mut cel_target = vec![0; usize::from(cel.size.0) * usize::from(cel.size.1) * 4];
        self.aseprite
            .load_image(cel.image_index, &mut cel_target)
            .unwrap();

        let (full_image_width, full_image_height) = self.aseprite.size();
        let mut full_image = Image::new(
            Extent3d {
                width: full_image_width as u32,
                height: full_image_height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![0; full_image_width as usize * full_image_height as usize * 4],
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );

        for y in 0..compressed_image.width {
            for x in 0..compressed_image.height {
                let i = y as usize * compressed_image.width as usize + x as usize;
                let r = cel_target[i * 4 + 0];
                let g = cel_target[i * 4 + 1];
                let b = cel_target[i * 4 + 2];
                let a = cel_target[i * 4 + 3];
                let color = Color::srgba_u8(r, g, b, a);
                let _ = full_image.set_color_at(
                    x as u32 + cel.origin.0 as u32,
                    y as u32 + cel.origin.1 as u32,
                    color,
                );
            }
        }

        Some(full_image)
    }
}

#[derive(Default)]
pub struct RawAsepriteLoader;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Could not load asset")]
    ReadError,
}

impl AssetLoader for RawAsepriteLoader {
    type Asset = RawAseprite;
    type Settings = ();
    type Error = ReadError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let _ = _load_context;
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| ReadError::ReadError)
            .unwrap();
        let aseprite = Arc::new(AsepriteFile::load(Box::leak(bytes.into_boxed_slice())).unwrap());
        Ok(RawAseprite { aseprite })
    }
    fn extensions(&self) -> &[&str] {
        &["aseprite", "ase"]
    }
}
