use super::super::{set::GameSet, states::GameState};
use crate::game::{actor::enemy::Enemy, entity::actor::Actor};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

static LIFE_BAR_WIDTH: f32 = 16.0;

static LIFE_BAR_HEIGHT: f32 = 2.0;

#[derive(Component)]
pub struct LifeBar;

#[derive(Component)]
pub struct LifeBarBackground;

#[derive(Resource, Reflect)]
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
    child_builder.spawn((
        LifeBar,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(res.shape.clone()),
            material: res.material_life.clone(),
            transform: Transform::from_xyz(0.0, 10.0, 1.0),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
    child_builder.spawn((
        LifeBarBackground,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(res.shape.clone()),
            material: res.material_background.clone(),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
    ));
}

pub fn update_life_bar(
    mut query: Query<(&Parent, &mut Transform, &mut Visibility), With<LifeBar>>,
    mut background_query: Query<
        (&Parent, &mut Visibility),
        // TODO: ここでなぜ Withoutが必要なのかよく理解できていない
        (With<LifeBarBackground>, Without<LifeBar>),
    >,
    enemy_query: Query<&Actor, With<Enemy>>,
) {
    for (life_bar_parent, mut transform, mut visibility) in query.iter_mut() {
        let enemy = enemy_query.get(life_bar_parent.get()).unwrap();
        let ratio = enemy.life as f32 / enemy.max_life as f32;
        *visibility = if ratio < 1.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        transform.scale.x = ratio;
        transform.translation.x = (ratio * LIFE_BAR_WIDTH - LIFE_BAR_WIDTH) * 0.5;
    }
    for (life_bar_parent, mut visibility) in background_query.iter_mut() {
        let enemy = enemy_query.get(life_bar_parent.get()).unwrap();
        let ratio = enemy.life as f32 / enemy.max_life as f32;
        *visibility = if ratio < 1.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub struct LifeBarPlugin;

impl Plugin for LifeBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LifeBarResource>();
        app.add_systems(Startup, setup_life_bar);
        app.add_systems(
            Update,
            update_life_bar
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );
    }
}
