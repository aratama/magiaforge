use crate::set::FixedUpdateGameActiveSet;
use crate::set::FixedUpdateInGameSet;
use crate::states::GameState;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;

/// 他のエンティティに付加する形で、点光源を表します
/// PointLight2Dは一般には他のコンポーネントの子として追加できないため、
/// WithPointLightを追加することで独立した点光源を生成し、位置を同期させます
#[derive(Component, Reflect)]
pub struct WithPointLight {
    pub radius: f32,
    pub intensity: f32,
    pub falloff: f32,
    pub color: Color,

    /// 明るさを変化させるときのオフセット
    pub animation_offset: u32,

    /// 明るさのちらつきは変化する速さ
    pub speed: f32,

    /// 明るさの変化の最大幅
    pub amplitude: f32,
}

#[derive(Component, Reflect)]
struct EntityPointLight {
    parent: Entity,
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
fn spawn(
    mut commands: Commands,
    parent_query: Query<(Entity, &WithPointLight, &Transform), Added<WithPointLight>>,
) {
    for (parent, light, transform) in parent_query.iter() {
        commands.spawn((
            Name::new("entity point light"),
            StateScoped(GameState::InGame),
            EntityPointLight { parent },
            transform.clone(),
            PointLight2d {
                radius: light.radius,
                intensity: light.intensity,
                falloff: light.falloff,
                color: light.color,
                ..default()
            },
        ));
    }
}

fn update_transform(
    parent_query: Query<
        (&WithPointLight, &Transform),
        Or<(Changed<WithPointLight>, Changed<Transform>)>,
    >,
    mut child_query: Query<
        (&EntityPointLight, &mut Transform),
        (With<PointLight2d>, Without<WithPointLight>),
    >,
) {
    for (child, mut transform) in child_query.iter_mut() {
        if let Ok((_, lantern_transform)) = parent_query.get(child.parent) {
            transform.translation = lantern_transform.translation;
        }
    }
}

fn update_intensity(
    parent_query: Query<&WithPointLight>,
    mut child_query: Query<(&EntityPointLight, &mut PointLight2d)>,
    frame_count: Res<FrameCount>,
) {
    for (child, mut light) in child_query.iter_mut() {
        if let Ok(lantern) = parent_query.get(child.parent) {
            light.intensity = lantern.intensity
                + (((lantern.animation_offset + frame_count.0) as f32 * lantern.speed).cos())
                    * lantern.amplitude;
        }
    }
}

fn despawn(
    mut commands: Commands,
    parent_query: Query<&WithPointLight>,
    query: Query<(Entity, &EntityPointLight)>,
) {
    for (entity, light) in query.iter() {
        if !parent_query.contains(light.parent) {
            commands.entity(entity).despawn_recursive();
            // info!("despawn {} {}", file!(), line!());
        }
    }
}

pub struct EntityPointLightPlugin;

impl Plugin for EntityPointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_transform, update_intensity).in_set(FixedUpdateGameActiveSet),
        );
        app.add_systems(FixedUpdate, (spawn, despawn).in_set(FixedUpdateInGameSet));
        app.register_type::<EntityPointLight>();
    }
}
