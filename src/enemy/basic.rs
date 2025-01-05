use crate::asset::GameAssets;
use crate::component::counter::CounterAnimated;
use crate::component::entity_depth::EntityDepth;
use crate::component::falling::Falling;
use crate::component::flip::Flip;
use crate::component::life::Life;
use crate::component::life::LifeBeingSprite;
use crate::constant::*;
use crate::controller::despawn_with_gold::DespawnWithGold;
use crate::controller::servant::Servant;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorGroup;
use crate::entity::actor::ActorProps;
use crate::entity::bullet::HomingTarget;
use crate::hud::life_bar::spawn_life_bar;
use crate::hud::life_bar::LifeBarResource;
use crate::inventory::Inventory;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::Wand;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

pub fn spawn_basic_enemy<T: Component>(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
    marker: T,
    name: &str,
    spell: Option<SpellType>,
    move_force: f32,
    golds: u32,
    actor_group: ActorGroup,
    master: Option<Entity>,
    max_life: i32,
    radius: f32,
) -> Entity {
    let mut builder = commands.spawn((
        Name::new(name.to_string()),
        StateScoped(GameState::InGame),
        DespawnWithGold { golds },
        marker,
        Actor::new(ActorProps {
            uuid: Uuid::new_v4(),
            angle: 0.0,
            point_light_radius: 0.0,
            radius,
            move_force: move_force,
            current_wand: 0,
            actor_group,
            golds,
            inventory: Inventory::new(),
            equipments: [None; MAX_ITEMS_IN_EQUIPMENT],
            wands: Wand::single(spell),
            fire_resistance: false,
        }),
        EntityDepth::new(),
        Life::new(max_life),
        HomingTarget,
        Transform::from_translation(position.extend(SHADOW_LAYER_Z)),
        GlobalTransform::default(),
        Visibility::default(),
        (
            RigidBody::Dynamic,
            Collider::ball(radius),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            ExternalImpulse::default(),
            ActiveEvents::COLLISION_EVENTS,
            actor_group.to_groups(),
        ),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "chicken_shadow".into(),
        },
    ));

    builder.with_children(|mut parent| {
        parent.spawn((
            Falling::new(0.0, -0.1),
            Flip,
            LifeBeingSprite,
            CounterAnimated,
            AseSpriteAnimation {
                aseprite,
                animation: Animation::default().with_tag("idle"),
            },
        ));

        spawn_life_bar(&mut parent, &life_bar_locals);
    });

    if let Some(owner) = master {
        builder.insert(Servant { master: owner });
    }

    builder.id()
}
