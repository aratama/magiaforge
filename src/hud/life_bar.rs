use crate::actor::{Actor, ActorGroup};
use crate::component::metamorphosis::Metamorphosed;
use crate::controller::player::Player;
use crate::enemy::huge_slime::Boss;
use crate::set::FixedUpdateInGameSet;
use bevy::prelude::*;

const LIFE_BAR_WIDTH: f32 = 16.0;

const LIFE_BAR_HEIGHT: f32 = 2.0;

const LIFE_BAR_Y: f32 = -8.0;

const LIFE_BAR_Z: f32 = 100.0;

#[derive(Component)]
pub struct LifeBar;

#[derive(Component)]
pub struct LifeBarForeground;

#[derive(Component)]
pub struct LifeBarBackground;

#[derive(Resource, Reflect, Clone)]
pub struct LifeBarResource {
    material_life: Handle<ColorMaterial>,
    material_background: Handle<ColorMaterial>,
    shape: Handle<Mesh>,
}

fn setup_life_bar(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material_life = materials.add(Color::hsl(0.0, 1.0, 0.5));
    let material_background = materials.add(Color::hsla(0.0, 0.0, 0.0, 0.5));
    let shape = meshes.add(Rectangle::new(LIFE_BAR_WIDTH, LIFE_BAR_HEIGHT));
    commands.insert_resource(LifeBarResource {
        material_life,
        material_background,
        shape,
    });
}

pub fn spawn_life_bar(child_builder: &mut ChildBuilder, res: &Res<LifeBarResource>) {
    child_builder
        .spawn((
            LifeBar,
            Transform::from_xyz(0.0, LIFE_BAR_Y, LIFE_BAR_Z),
            Visibility::Hidden,
        ))
        .with_child((
            LifeBarForeground,
            Mesh2d::from(res.shape.clone()),
            MeshMaterial2d::from(res.material_life.clone()),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ))
        .with_child((
            LifeBarBackground,
            Mesh2d::from(res.shape.clone()),
            MeshMaterial2d::from(res.material_background.clone()),
        ));
}

fn update_life_bar(
    actor_query: Query<(
        &Actor,
        Option<&Player>,
        Option<&Metamorphosed>,
        Option<&Boss>,
    )>,
    mut lifebar_query: Query<(&Parent, &mut Visibility), With<LifeBar>>,
    mut foreground_query: Query<
        (&Parent, &mut Transform),
        (With<LifeBarForeground>, Without<LifeBar>),
    >,
) {
    for (foreground_parent, mut transform) in foreground_query.iter_mut() {
        let (lifebar_parent, mut visibility) =
            lifebar_query.get_mut(foreground_parent.get()).unwrap();
        let (actor, player, morphed, boss) = actor_query.get(lifebar_parent.get()).unwrap();
        let ratio = actor.life as f32 / actor.max_life as f32;
        *visibility = if actor.actor_group != ActorGroup::Entity
            && player.is_none()
            && morphed.is_none()
            && boss.is_none()
            && ratio < 1.0
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        transform.scale.x = ratio;
        transform.translation.x = (ratio * LIFE_BAR_WIDTH - LIFE_BAR_WIDTH) * 0.5;
    }
}

pub struct LifeBarPlugin;

impl Plugin for LifeBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LifeBarResource>();
        app.add_systems(Startup, setup_life_bar);
        app.add_systems(FixedUpdate, update_life_bar.in_set(FixedUpdateInGameSet));
    }
}
