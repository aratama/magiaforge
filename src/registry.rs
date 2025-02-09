pub mod actor;
pub mod game;
pub mod spell;
pub mod tile;

use crate::actor::ActorType;
use crate::asset::GameAssets;
use crate::hud::life_bar::LifeBarResource;
use crate::ldtk::generated::EntityInstance;
use crate::ldtk::generated::Level;
use crate::ldtk::loader::LevelCustomFields;
use crate::ldtk::loader::LDTK;
use crate::level::tile::Tile;
use crate::level::world::GameLevel;
use crate::spell::Spell;
use crate::spell::SpellProps;
use actor::ActorPropsByType;
use actor::ActorRegistry;
use bevy::asset::AssetPath;
use bevy::asset::Assets;
use bevy::asset::Handle;
use bevy::audio::AudioSource;
use bevy::ecs::system::SystemParam;
use bevy::log::warn;
use bevy::prelude::Res;
use game::BGMProps;
use game::GameRegistry;
use serde_json::from_value;
use spell::SpellRegistry;
use std::sync::LazyLock;
use tile::TileRegistry;
use tile::TileType;
use tile::TileTypeProps;

/// アセットの config.*.ron から読み取る設定を取得するシステムパラメータです
/// また、GameAssets も参照できます
/// この構造体がゲーム内から変更されることはありません
#[derive(SystemParam)]
pub struct Registry<'w> {
    pub assets: Res<'w, GameAssets>,
    ldtk_assets: Res<'w, Assets<LDTK>>,
    pub life_bar_resource: Res<'w, LifeBarResource>,

    game: Res<'w, Assets<GameRegistry>>,
    tile: Res<'w, Assets<TileRegistry>>,
    spell: Res<'w, Assets<SpellRegistry>>,
    actor: Res<'w, Assets<ActorRegistry>>,
}

impl<'w> Registry<'w> {
    pub fn ldtk(&self) -> &LDTK {
        self.ldtk_assets.get(self.assets.ldtk_level.id()).unwrap()
    }

    pub fn game(&self) -> &GameRegistry {
        self.game.get(&self.assets.game_registry).unwrap()
    }

    pub fn tile(&self) -> &TileRegistry {
        self.tile.get(&self.assets.tile_registry).unwrap()
    }

    pub fn spell(&self) -> &SpellRegistry {
        self.spell.get(&self.assets.spell_registry).unwrap()
    }

    pub fn actor(&self) -> &ActorRegistry {
        self.actor.get(&self.assets.actor_registry).unwrap()
    }

    pub fn get_actor_props(&self, actor_type: &ActorType) -> &ActorPropsByType {
        let constants = self.actor.get(&self.assets.actor_registry).unwrap();
        &constants
            .actors
            .get(&actor_type.0)
            .expect(format!("ActorType {:?} not found", actor_type.0).as_str())
    }

    pub fn get_tile(&self, tile: &Tile) -> &TileTypeProps {
        let constants = self.tile.get(&self.assets.tile_registry).unwrap();
        &constants
            .tile_types
            .get(&tile.0)
            .expect(format!("Tile {:?} not found", tile.0).as_str())
    }

    pub fn get_spell_props(&self, Spell(spell_type): &Spell) -> &SpellProps {
        let constants = self.spell.get(&self.assets.spell_registry).unwrap();
        &constants
            .spells
            .get(spell_type)
            .expect(&format!("spell '{}' not found", spell_type))
    }

    pub fn get_spells(&self) -> Vec<Spell> {
        self.spell().spells.keys().map(|k| Spell::new(k)).collect()
    }

    pub fn get_level(&self, level: &GameLevel) -> LevelCustomFields {
        let ldtk = self.ldtk();
        let Some(level) = ldtk.get_level(level) else {
            warn!("Level {:?} not found", level);
            return LevelCustomFields::default();
        };
        let map = serde_json::Map::from_iter(level.field_instances.iter().map(|v| {
            (
                v.identifier.clone(),
                v.value.as_ref().unwrap_or(&serde_json::Value::Null).clone(),
            )
        }));
        from_value(serde_json::Value::Object(map)).expect("Error occurred in get_level")
    }

    pub fn get_bgm(&self, handle: &Handle<AudioSource>) -> &BGMProps {
        if let Some(path) = handle.path() {
            let name = path_to_string(path);
            self.game()
                .bgms
                .get(&name)
                .expect(&format!("BGM '{}' not found", name))
        } else {
            warn!("no path in audio handle");
            static DEFAULT_BGM_PROPS: LazyLock<BGMProps> = LazyLock::new(|| BGMProps {
                author: "".to_string(),
                title: "".to_string(),
                appendix: "".to_string(),
            });
            &DEFAULT_BGM_PROPS
        }
    }

    #[allow(dead_code)]
    pub fn get_level_by_iid(&self, iid: &str) -> Level {
        let ldtk = self.ldtk();
        ldtk.coordinate
            .levels
            .iter()
            .find(|l| l.iid == iid)
            .expect(&format!("Level {:?} not found", iid))
            .clone()
    }

    pub fn get_level_by_entity_iid(&self, entity_iid: &str) -> Option<(Level, EntityInstance)> {
        let ldtk = self.ldtk();
        for l in ldtk.coordinate.levels.iter() {
            for layer in l.layer_instances.as_ref().unwrap_or(&vec![]).iter() {
                for entity in layer.entity_instances.iter() {
                    if entity.iid == entity_iid {
                        return Some((l.clone(), entity.clone()));
                    }
                }
            }
        }
        None
    }

    pub fn get_wall_tiles(&self) -> Vec<Tile> {
        self.tile()
            .tile_types
            .iter()
            .filter_map(|(name, props)| {
                if props.tile_type == TileType::Wall {
                    Some(Tile::new(name))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_surface_tiles(&self) -> Vec<Tile> {
        self.tile()
            .tile_types
            .iter()
            .filter_map(|(name, props)| {
                if props.tile_type == TileType::Surface {
                    Some(Tile::new(name))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn path_to_string(path: &AssetPath) -> String {
    path.path().to_str().unwrap_or("").to_string()
}
