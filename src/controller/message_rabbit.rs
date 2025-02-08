use crate::actor::witch::Witch;
use crate::actor::Actor;
use crate::actor::ActorAppearanceSprite;
use crate::actor::ActorSpriteGroup;
use crate::camera::GameCamera;
use crate::controller::player::Player;
use crate::physics::identify;
use crate::physics::identify_item;
use crate::physics::IdentifiedCollisionEvent;
use crate::physics::IdentifiedCollisionItem;
use crate::script::cmd::Cmd;
use crate::script::cmd::CmdEvent;
use crate::script::context::JavaScriptContext;
use crate::set::FixedUpdateInGameSet;
use crate::ui::speech_bubble::SpeechBubble;
use crate::ui::spell_list::SpellList;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MessageRabbit {
    pub trigger: String,
    pub aseprite: String,
}

impl MessageRabbit {
    pub fn new(aseprite: &str, senario: &str) -> Self {
        Self {
            aseprite: aseprite.to_string(),
            trigger: senario.to_string(),
        }
    }
}

#[derive(Component)]
pub struct MessageRabbitInnerSensor;

#[derive(Component)]
pub struct MessageRabbitOuterSensor;

fn collision_inner_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    rabbit_query: Query<&MessageRabbit>,
    sensor_query: Query<&Parent, With<MessageRabbitInnerSensor>>,
    mut camera_query: Query<&mut GameCamera>,
    player_query: Query<&Actor, (With<Player>, With<Witch>)>,
    mut script: NonSendMut<JavaScriptContext>,
    mut speech_query: Query<&mut SpeechBubble>,
) {
    let mut speech = speech_query.single_mut();
    for collision_event in collision_events.read() {
        match identify_item(collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionItem::Started(parent, _, _, _) => {
                let mut camera = camera_query.single_mut();
                if camera.target.is_some() {
                    continue;
                }
                let rabbit_entity = parent.get();
                let rabbit = rabbit_query.get(rabbit_entity).unwrap();
                camera.target = Some(rabbit_entity);
                speech.entity = Some(rabbit_entity);
                script.generate(rabbit.trigger.clone());
            }
            _ => {}
        }
    }
}

fn collision_outer_sensor(
    mut collision_events: EventReader<CollisionEvent>,
    mut camera_query: Query<&mut GameCamera>,
    sensor_query: Query<&MessageRabbitOuterSensor>,
    player_query: Query<&Actor, With<Player>>,
    mut cmd_writer: EventWriter<CmdEvent>,
    mut script: NonSendMut<JavaScriptContext>,
    mut spell_list_query: Query<&mut SpellList>,
) {
    for collision_event in collision_events.read() {
        match identify(&collision_event, &sensor_query, &player_query) {
            IdentifiedCollisionEvent::Stopped(..) => {
                let mut camera = camera_query.single_mut();
                camera.target = None;
                cmd_writer.send(CmdEvent(Cmd::Close));
                script.abort();
                spell_list_query.single_mut().open = false;
            }
            _ => {}
        }
    }
}

pub fn update_rabbit_sprite(
    asset_server: Res<AssetServer>,
    mut query: Query<(&Parent, &mut AseSpriteAnimation), With<ActorAppearanceSprite>>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    actor_query: Query<&MessageRabbit>,
) {
    for (parent, mut animation) in query.iter_mut() {
        let parent = group_query.get(parent.get()).unwrap();
        if let Ok(rabbit) = actor_query.get(parent.get()) {
            animation.aseprite = asset_server.load(rabbit.aseprite.clone());
        }
    }
}

pub struct MessageRabbitPlugin;

impl Plugin for MessageRabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                collision_inner_sensor,
                collision_outer_sensor,
                update_rabbit_sprite,
            )
                .in_set(FixedUpdateInGameSet),
        );
    }
}
