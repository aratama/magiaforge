use crate::asset::GameAssets;
use crate::camera::GameCamera;
use crate::collision::ENTITY_GROUPS;
use crate::collision::SENSOR_GROUPS;
use crate::component::counter::Counter;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::falling::Falling;
use crate::component::point_light::WithPointLight;
use crate::level::tile::Tile;
use crate::page::in_game::LevelSetup;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

/// 炎のエンティティ
/// 炎には次の効果があります
/// 1. 何らかの Burnable を燃料として燃えている場合は、そのエンティティの life を減少させます
/// 2. 付近の Burnable ではないが Life であるエンティティに、一定時間ごとにダメージを与えます
/// 3. 1フレームごとに120分の1の確率で、付近の Burnable を燃料として新たな炎を生成します
///
/// Burnableを設定したコンポーネントは life が0になると消滅しますが、その挙動は fire.rs では実装されません
/// 各エンティティの性質に合わせて、消滅処理を実装してください
#[derive(Default, Component, Reflect)]
pub struct Fire {
    /// 燃料となっているエンティティ
    /// burnable が Some の場合は、炎の座標はそのエンティティの座標と一致します
    burnable: Option<Entity>,
}

/// 可燃性のエンティティを表します
/// 燃え尽きて消滅するためには Life も付与する必要があります
#[derive(Default, Component, Reflect)]
pub struct Burnable {
    /// 燃焼しているあいだlifeが減少し、0になったらそのエンティティは消滅する処理をしなければなりません
    pub life: u32,
}

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
        audio.set_volume(1.0 - ((250.0 + distance) / 500.0).min(1.0).max(0.0));
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
        Falling,
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
            // Fireはセンサーとして機能するわけではありませんが、
            // 炎ダメージ判定をするときにはアクター側から交差判定を行うので、
            // それで検出できるようにコリジョンとグループを設定しています
            Sensor,
            Collider::ball(8.0),
            *SENSOR_GROUPS,
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

fn despawn(mut commands: Commands, query: Query<(Entity, &Counter), With<Fire>>) {
    for (entity, counter) in query.iter() {
        if counter.count <= 0 {
            commands.entity(entity).despawn_recursive();
            // info!("despawn {} {}", file!(), line!());
        }
    }
}

fn burn(fire_query: Query<&Fire>, mut burnable_query: Query<&mut Burnable>) {
    for fire in fire_query.iter() {
        if let Some(burnable) = fire.burnable {
            if let Ok(mut burnable) = burnable_query.get_mut(burnable) {
                if 0 < burnable.life {
                    burnable.life -= 1;
                }
            }
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
                    // 延焼センサーは ENTITY_GROUP (本棚やチェストなど) と SENSOR_GROUP (草など) にのみ反応する
                    groups: Some(*ENTITY_GROUPS),
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

fn melt_ice(fire_query: Query<&Transform, With<Fire>>, mut level: ResMut<LevelSetup>) {
    if let Some(ref mut level) = level.chunk {
        for fire_transform in fire_query.iter() {
            let position = fire_transform.translation.truncate();
            let tile = level.get_tile_by_coords(position);
            if tile == Tile::Ice {
                level.set_tile_by_position(position, Tile::Water);
            }
        }
    }
}

fn despawn_on_water(
    mut commands: Commands,
    fire_query: Query<(Entity, &Transform), With<Fire>>,
    level: Res<LevelSetup>,
) {
    if let Some(ref level) = level.chunk {
        for (entity, fire_transform) in fire_query.iter() {
            let position = fire_transform.translation.truncate();
            let tile = level.get_tile_by_coords(position);
            if tile == Tile::Water {
                commands.entity(entity).despawn_recursive();
                // info!("despawn {} {}", file!(), line!());
            }
        }
    }
}

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup);
        app.add_systems(
            FixedUpdate,
            (
                despawn,
                translate,
                ignite,
                update_se_volume,
                burn,
                (melt_ice, despawn_on_water).chain(),
            )
                .in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Fire>();
    }
}
