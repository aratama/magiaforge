use super::metamorphosis::cast_metamorphosis;
use super::metamorphosis::Metamorphosed;
use crate::actor::Actor;
use crate::actor::ActorEvent;
use crate::asset::GameAssets;
use crate::controller::player::Player;
use crate::entity::fire::Burnable;
use crate::entity::fire::Fire;
use crate::hud::life_bar::LifeBarResource;
use crate::level::entities::add_default_behavior;
use crate::level::entities::SpawnEntity;
use crate::se::SEEvent;
use crate::se::SE;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_rapier2d::plugin::DefaultRapierContext;
use bevy_rapier2d::plugin::RapierContext;
use bevy_rapier2d::prelude::Collider;
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_rapier2d::prelude::QueryFilter;

/// 木箱やトーチなどの破壊可能なオブジェクトを表すコンポーネントです
/// 弾丸は Breakable コンポーネントを持つエンティティに対してダメージを与えます
/// ただし、ライフがゼロになってもこのコンポーネント自身は自動でdespawnしません
#[derive(Default, Component, Reflect, Clone, Debug)]
pub struct Life {
    /// 破壊可能なオブジェクトのライフ
    pub life: u32,

    pub max_life: u32,

    /// ダメージを受けた時の振動の幅
    pub amplitude: f32,

    /// 次に炎でダメージを受けるまでの間隔
    pub fire_damage_wait: u32,
}

impl Life {
    pub fn new(life: u32) -> Self {
        Self {
            life,
            max_life: life,
            amplitude: 0.0,
            fire_damage_wait: 0,
        }
    }
}

/// ダメージを受けた時に振動するスプライト
#[derive(Default, Component, Reflect)]
pub struct LifeBeingSprite;

fn vibrate_breakabke_sprite(
    time: Res<Time>,
    mut breakable_query: Query<(&mut Life, &Children)>,
    mut breakable_sprite_query: Query<&mut Transform, With<LifeBeingSprite>>,
) {
    for (mut breakable, children) in breakable_query.iter_mut() {
        for child in children {
            if let Ok(mut transform) = breakable_sprite_query.get_mut(*child) {
                transform.translation.x = (time.elapsed_secs() * 56.0).sin() * breakable.amplitude;
            }
            breakable.amplitude *= 0.9;
        }
    }
}

/// 付近に炎がある場合、ダメージを受けます
/// ただし、Burnableである場合はダメージを受けませんが、その代わりに引火することがあり、
/// 引火したあとで Burnable の life がゼロになった場合はエンティティは消滅します
fn fire_damage(
    mut actor_query: Query<(Entity, &Actor, &mut Life, &Transform), Without<Burnable>>,
    fire_query: Query<&mut Transform, (With<Fire>, Without<Life>)>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    mut actor_event: EventWriter<ActorEvent>,
) {
    for (actor_entity, actor, mut life, actor_transform) in actor_query.iter_mut() {
        if life.fire_damage_wait <= 0 && !actor.fire_resistance {
            let mut detected_fires = Vec::<Entity>::new();
            let context = rapier_context.single();

            // 各アクターは、自分の周囲に対して炎を検索します
            // 炎は多数になる可能性があることや、
            // アクターはダメージの待機時間があり大半のフレームでは判定を行わないため、
            // 炎側からアクター側へ判定を行うのではなく、アクター側から炎側へ判定を行ったほうが効率が良くなります
            context.intersections_with_shape(
                actor_transform.translation.truncate(),
                0.0,
                &Collider::ball(12.0),
                QueryFilter {
                    groups: Some(actor.actor_group.to_groups(0.0, 0)),
                    ..default()
                },
                |entity| {
                    if fire_query.contains(entity) {
                        if life.fire_damage_wait <= 0 {
                            detected_fires.push(entity);

                            // 一度炎ダメージを受けたらそれ以上他の炎からダメージを受けることはないため、
                            // 探索を打ち切る
                            return false;
                        }
                    }
                    true // 交差図形の検索を続ける
                },
            );

            for _ in detected_fires {
                actor_event.send(ActorEvent::Damaged {
                    actor: actor_entity,
                    damage: 4,
                    position: actor_transform.translation.truncate(),
                    fire: true,
                    impulse: Vec2::ZERO,
                    stagger: 0,
                    metamorphose: None,
                    dispel: false,
                });
            }
        }

        if 0 < life.fire_damage_wait {
            life.fire_damage_wait -= 1;
        }
    }
}

fn damage(
    mut commands: Commands,
    assets: Res<GameAssets>,
    life_bar_resource: Res<LifeBarResource>,
    mut spawn: EventWriter<SpawnEntity>,
    mut query: Query<(
        &mut Life,
        &Transform,
        Option<&mut ExternalImpulse>,
        Option<&mut Actor>,
        Option<&Player>,
        Option<&mut Metamorphosed>,
    )>,
    mut reader: EventReader<ActorEvent>,
    mut se: EventWriter<SEEvent>,
) {
    for event in reader.read() {
        match event {
            ActorEvent::Damaged {
                actor: actor_entity,
                damage,
                position,
                fire,
                impulse,
                stagger,
                metamorphose,
                dispel,
            } => {
                let Ok((
                    mut life,
                    life_transform,
                    life_impulse,
                    mut actor_optional,
                    _,
                    mut actor_metamorphosis,
                )) = query.get_mut(*actor_entity)
                else {
                    continue;
                };

                if let Some(ref mut actor) = actor_optional {
                    if actor.staggered == 0 || !actor.invincibility_on_staggered {
                        life.life = (life.life as i32 - *damage as i32).max(0) as u32;
                        actor.staggered = (actor.staggered + stagger).min(120);
                    }
                } else {
                    life.life = (life.life as i32 - *damage as i32).max(0) as u32;
                }

                life.amplitude = 6.0;

                se.send(SEEvent::pos(SE::Damage, *position));

                if *fire {
                    life.fire_damage_wait = 60 + (rand::random::<u32>() % 60);
                }

                if let Some(mut life_impulse) = life_impulse {
                    life_impulse.impulse += *impulse;
                }

                let Some(ref mut actor) = actor_optional else {
                    continue;
                };

                if let Some(morphing_to) = metamorphose {
                    if 0 < life.life {
                        let position = life_transform.translation.truncate();
                        let entity = cast_metamorphosis(
                            &mut commands,
                            &assets,
                            &life_bar_resource,
                            &mut se,
                            &mut spawn,
                            actor_entity,
                            actor.clone(),
                            life.clone(),
                            &actor_metamorphosis.as_deref(),
                            position,
                            *morphing_to,
                        );
                        add_default_behavior(&mut commands, *morphing_to, position, entity);
                    }
                } else if let Some(ref mut actor_metamorphosis) = actor_metamorphosis {
                    if *dispel {
                        actor_metamorphosis.count = 0;
                    }
                }
            }
        }
    }
}

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (damage, vibrate_breakabke_sprite, fire_damage)
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
    }
}
