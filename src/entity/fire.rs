use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::component::point_light::WithPointLight;
use crate::constant::ENEMY_GROUP;
use crate::constant::ENTITY_GROUP;
use crate::constant::RABBIT_GROUP;
use crate::constant::SENSOR_GROUP;
use crate::constant::WITCH_GROUP;
use crate::entity::EntityDepth;
use crate::states::GameState;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
pub struct Fire {
    /// 燃えているエンティティ
    /// burnable が Some の場合は、炎の座標はそのエンティティの座標と一致します
    burnable: Option<Entity>,
}

#[derive(Default, Component, Reflect)]
pub struct Burnable;

#[derive(Default, Component, Reflect)]
struct GlobalFireSE;

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        GlobalFireSE,
        StateScoped(GameState::InGame),
        AudioPlayer::new(assets.takibi.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        },
    ));
}

fn update_se_volume(
    camera_query: Query<&Transform, With<GameCamera>>,
    sink_query: Query<&AudioSink, With<GlobalFireSE>>,
    fire_query: Query<&Transform, With<Fire>>,
) {
    // setup直後にはSinkがない場合があることに注意
    if let Ok(audio) = sink_query.get_single() {
        let camera_transform = camera_query.single();
        let mut distance = f32::MAX;
        for fire_transform in fire_query.iter() {
            let distance_to_fire = fire_transform
                .translation
                .truncate()
                .distance(camera_transform.translation.truncate());
            if distance_to_fire < distance {
                distance = distance_to_fire;
            }
        }
        audio.set_volume(1.0 - ((500.0 + distance) / 1000.0).min(1.0).max(0.0));
    }
}

pub fn spawn_fire(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    burnable: Option<Entity>,
) {
    commands.spawn((
        Name::new("fire"),
        StateScoped(GameState::InGame),
        Fire { burnable },
        Counter::down(1800),
        EntityDepth::offset(0.00001),
        Visibility::default(),
        Transform::from_translation(position.extend(0.0)),
        CounterAnimated,
        AseSpriteAnimation {
            aseprite: assets.fire.clone(),
            animation: "default".into(),
            ..default()
        },
        WithPointLight {
            radius: 128.0,
            intensity: 3.0,
            falloff: 10.0,
            color: Color::hsl(42.0, 1.0, 0.71),
            animation_offset: rand::random::<u32>() % 1000,
            speed: 0.43,
            amplitude: 0.1,
        },
        (
            Sensor,
            Collider::ball(8.0),
            CollisionGroups::new(
                SENSOR_GROUP,
                ENTITY_GROUP | WITCH_GROUP | ENEMY_GROUP | RABBIT_GROUP,
            ),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::all(),
        ),
    ));
}

fn translate(
    mut fire_query: Query<(&Fire, &mut Transform)>,
    burnable_query: Query<&Transform, Without<Fire>>,
) {
    for (fire, mut fire_transform) in fire_query.iter_mut() {
        if let Some(burnable) = fire.burnable {
            if let Ok(burnable_transform) = burnable_query.get(burnable) {
                fire_transform.translation = burnable_transform.translation;
            }
        }
    }
}

fn despown(mut commands: Commands, query: Query<(Entity, &Counter), With<Fire>>) {
    for (entity, counter) in query.iter() {
        if counter.count <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn ignite(
    mut commands: Commands,
    assets: Res<GameAssets>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
    burnable_query: Query<(Entity, &Transform, &Burnable)>,
    fire_query: Query<(&Fire, &Transform), Without<Burnable>>,
) {
    let context: &RapierContext = rapier_context.single();
    for (_, fire_transform) in fire_query.iter() {
        // それぞれの炎は、フレームごとに1/16の確率で炎を広げる
        if rand::random::<u32>() % 240 == 0 {
            // この炎から周囲32ピクセルにあるエンティティを取得
            let mut entities: Vec<Entity> = Vec::new();
            context.intersections_with_shape(
                fire_transform.translation.truncate(),
                0.0,
                &Collider::ball(32.0),
                QueryFilter {
                    groups: Some(CollisionGroups::new(ENTITY_GROUP, ENTITY_GROUP)),
                    ..default()
                },
                |entity| {
                    entities.push(entity);
                    true
                },
            );

            for entity in entities {
                // Burnableであるものを選択
                if burnable_query.contains(entity) {
                    if let Some(_) = fire_query.iter().find(|(f, _)| f.burnable == Some(entity)) {
                        // すでに着火済み
                        continue;
                    } else {
                        // 着火
                        spawn_fire(
                            &mut commands,
                            &assets,
                            fire_transform.translation.truncate(),
                            Some(entity),
                        );
                        break;
                    }
                }
            }
        }
    }
}

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup);
        app.add_systems(Update, update_se_volume.run_if(in_state(GameState::InGame)));
        app.add_systems(
            FixedUpdate,
            (despown, translate, ignite)
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Fire>();
    }
}
