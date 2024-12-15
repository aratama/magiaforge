use crate::{
    controller::player::Player,
    entity::{dropped_item::DroppedItemEntity, rabbit::Rabbit},
    se::{SECommand, SE},
    set::GameSet,
    speech_bubble::{SpeechBubble, SpeechEvent},
    states::{GameMenuState, GameState},
    ui::interaction_marker::InteractionMarker,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// ソートしてプレイヤーに最も近いエンティティを取得
pub fn pick_nearest_drop_item(entities: &mut Vec<(Entity, Vec2)>, from: Vec2) -> Option<Entity> {
    entities.sort_by(|(_, a), (_, b)| {
        let a = a.distance(from);
        let b = b.distance(from);
        a.total_cmp(&b)
    });

    if let Some((nearest, _)) = entities.first() {
        return Some(*nearest);
    } else {
        return None;
    }
}

fn pick_up(
    mut player_query: Query<(&mut Player, &Transform)>,
    interactive_query: Query<(&Parent, &GlobalTransform), With<InteractionMarker>>,
    dropped_spell_query: Query<&DroppedItemEntity>,
    rabbit_query: Query<&Rabbit>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut global: EventWriter<SECommand>,
    state: Res<State<GameMenuState>>,
    mut speech_writer: EventWriter<SpeechEvent>,
) {
    if keys.just_pressed(KeyCode::KeyE) && *state.get() == GameMenuState::Closed {
        if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
            // ソートして最も距離が近いものを選択

            let origin = player_transform.translation.truncate();

            // センサーに含まれるすべてのエンティティを抽出
            let mut interactives = Vec::new();
            for (parent, transform) in interactive_query.iter() {
                if transform.translation().truncate().distance(origin) < 64.0 {
                    interactives.push((parent.get(), transform.translation().truncate()));
                }
            }

            if let Some(nearest_parent) = pick_nearest_drop_item(&mut interactives, origin) {
                if let Ok(DroppedItemEntity { item_type, .. }) =
                    dropped_spell_query.get(nearest_parent)
                {
                    if player.inventory.insert(*item_type) {
                        commands.entity(nearest_parent).despawn_recursive();
                        global.send(SECommand::new(SE::PickUp));
                        // エンティティを削除すれば Stopped イベントが発生してリストから消えるので、
                        // ここで削除する必要はない
                        // entities.remove(entity);
                    } else {
                        warn!("Inventory is full");
                    }
                } else if let Ok(_rabbit) = rabbit_query.get(nearest_parent) {
                    speech_writer.send(SpeechEvent::Speech(
                        "やあ、君か。\nなにか買っていくかい？".to_string(),
                    ));
                } else {
                    warn!("no item for nrearest {:?}", nearest_parent);
                }
            } else {
                warn!("no nearest. interactives: {:?}", interactives.len());
            }
        }
    }
}

pub struct EntityPickerPlugin;

impl Plugin for EntityPickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            pick_up
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
