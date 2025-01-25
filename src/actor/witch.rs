use crate::actor::Actor;
use crate::actor::ActorSpriteGroup;
use crate::component::vertical::Vertical;
use crate::se::SEEvent;
use crate::se::CHAKUCHI;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;

pub const MAX_GETTING_UP: u32 = 240;

#[derive(Default, Component, Reflect)]
pub struct ActorAnimationSprite;

#[derive(Default, Component, Reflect)]
pub struct WitchWandSprite;

#[derive(Component)]
pub struct Witch {
    pub getting_up: u32,
}

fn update_wand(
    actor_query: Query<(&Actor, &Witch)>,
    witch_sprite_group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut query: Query<
        (&Parent, &mut Transform, &mut Visibility),
        (With<WitchWandSprite>, Without<Actor>),
    >,
) {
    for (parent, mut transform, mut visibility) in query.iter_mut() {
        let group_parent = witch_sprite_group_query.get(parent.get()).unwrap();
        let (actor, witch) = actor_query.get(group_parent.get()).unwrap();
        let direction = actor.pointer;
        let angle = direction.to_angle();
        let pi = std::f32::consts::PI;
        transform.rotation = Quat::from_rotation_z(angle);
        transform.translation.x = if pi * 0.25 < angle && angle < pi * 0.75 {
            4.0
        } else if angle < pi * -0.25 && pi * -0.75 < angle {
            -4.0
        } else {
            0.0
        };

        *visibility = if 0 < actor.drown || 0 < actor.staggered {
            Visibility::Hidden
        } else {
            if witch.getting_up < MAX_GETTING_UP {
                Visibility::Hidden
            } else {
                Visibility::Visible
            }
        };
    }
}

fn land_se(witch_query: Query<(&Vertical, &Transform), With<Witch>>, mut se: EventWriter<SEEvent>) {
    for (vertical, transform) in witch_query.iter() {
        if vertical.just_landed {
            let position = transform.translation.truncate();
            se.send(SEEvent::pos(CHAKUCHI, position));
        }
    }
}

pub struct WitchPlugin;

impl Plugin for WitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_wand, land_se).in_set(FixedUpdateGameActiveSet),
        );
    }
}
