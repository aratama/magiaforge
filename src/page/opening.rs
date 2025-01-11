use core::f32;

use crate::audio::NextBGM;
use crate::hud::overlay::OverlayEvent;
use crate::se::SE;
use crate::states::GameState;
use crate::{asset::GameAssets, se::SEEvent};
use bevy::animation::{animated_field, AnimationTarget, AnimationTargetId};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationState, AseSpriteAnimation};

const FADE_IN: f32 = 5.0;
const CRY: f32 = FADE_IN + 5.0;
const APPEAR: f32 = CRY + 2.0;
const TURN: f32 = APPEAR + 2.0;
const ATTACK: f32 = TURN + 2.0;
const HIT: f32 = ATTACK + 0.2;
const FADE_OUT: f32 = HIT + 3.0;
const TAORERU: f32 = FADE_OUT + 3.0;
const CLOSE: f32 = TAORERU + 3.0;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
struct OpeningCount {
    count: u32,
}

#[derive(Component, Debug)]
struct Raven;

#[derive(Event, Clone, Debug, PartialEq, Eq)]
enum OpeningEvent {
    SE(SE),
    BGM(Option<Handle<AudioSource>>),
    Animate {
        name: String,
        tag: String,
        frame: u16,
    },
    FadeIn,
    FadeOut,
    Close,
}

fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,

    mut count: ResMut<OpeningCount>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    count.count = 0;

    commands.spawn((
        StateScoped(GameState::Opening),
        Camera2d { ..default() },
        OrthographicProjection {
            near: -1000.0,
            scale: 0.25,
            ..OrthographicProjection::default_3d()
        },
    ));

    commands.spawn((
        Name::new("background"),
        StateScoped(GameState::Opening),
        AseSpriteAnimation {
            aseprite: assets.opening.clone(),
            animation: "default".into(),
        },
        Transform::from_xyz(0.0, 0.0, -100.0),
    ));

    setup_witch(&mut commands, &assets, &mut animations, &mut graphs);

    setup_raven(&mut commands, &assets, &mut animations, &mut graphs);
}

fn setup_witch(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    animations: &mut ResMut<Assets<AnimationClip>>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
) {
    let name = Name::new("witch");

    let animation_target_id = AnimationTargetId::from_name(&name);

    let mut animation = AnimationClip::default();

    animation.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new([
                (0.0, Vec3::new(0.0, 0.0, 0.0)),
                (HIT, Vec3::new(0.0, 0.0, 0.0)),
                (HIT + 2.0, Vec3::new(0.0, -120.0, 0.0)),
            ])
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );
    let rot = 0.6;
    animation.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new([
                (0.0, Quat::from_rotation_z(f32::consts::PI * 0.0)),
                (
                    HIT + rot * 0.0,
                    Quat::from_rotation_z(f32::consts::PI * 0.0),
                ),
                (
                    HIT + rot * 1.0,
                    Quat::from_rotation_z(f32::consts::PI * 0.5),
                ),
                (
                    HIT + rot * 2.0,
                    Quat::from_rotation_z(f32::consts::PI * 1.0),
                ),
                (
                    HIT + rot * 3.0,
                    Quat::from_rotation_z(f32::consts::PI * 1.5),
                ),
                (
                    HIT + rot * 4.0,
                    Quat::from_rotation_z(f32::consts::PI * 2.0),
                ),
                (
                    HIT + rot * 5.0,
                    Quat::from_rotation_z(f32::consts::PI * 2.5),
                ),
                (
                    HIT + rot * 6.0,
                    Quat::from_rotation_z(f32::consts::PI * 3.0),
                ),
                (
                    HIT + rot * 7.0,
                    Quat::from_rotation_z(f32::consts::PI * 3.5),
                ),
                (
                    HIT + rot * 8.0,
                    Quat::from_rotation_z(f32::consts::PI * 4.0),
                ),
                (
                    HIT + rot * 9.0,
                    Quat::from_rotation_z(f32::consts::PI * 4.5),
                ),
                (
                    HIT + rot * 10.0,
                    Quat::from_rotation_z(f32::consts::PI * 5.0),
                ),
                (
                    HIT + rot * 11.0,
                    Quat::from_rotation_z(f32::consts::PI * 5.5),
                ),
            ])
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );

    animation.add_event(0.0, OpeningEvent::BGM(Some(assets.kaze.clone())));
    animation.add_event(0.0, OpeningEvent::FadeOut);
    animation.add_event(FADE_IN, OpeningEvent::FadeIn);

    animation.add_event(
        TURN,
        OpeningEvent::Animate {
            name: "witch".to_string(),
            tag: "turn".to_string(),
            frame: 32,
        },
    );
    animation.add_event(
        HIT,
        OpeningEvent::Animate {
            name: "witch".to_string(),
            tag: "hang".to_string(),
            frame: 33,
        },
    );
    animation.add_event(HIT, OpeningEvent::SE(SE::Damage));
    animation.add_event(FADE_OUT, OpeningEvent::FadeOut);
    animation.add_event(FADE_OUT, OpeningEvent::BGM(None));
    animation.add_event(TAORERU, OpeningEvent::SE(SE::Taoreru));
    animation.add_event(CLOSE, OpeningEvent::Close);

    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    let mut player = AnimationPlayer::default();
    player.play(animation_index);

    let entity = commands
        .spawn((
            name,
            StateScoped(GameState::Opening),
            AseSpriteAnimation {
                aseprite: assets.witch.clone(),
                animation: "fly".into(),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            AnimationGraphHandle(graphs.add(graph)),
            player,
        ))
        .id();

    commands.entity(entity).insert(AnimationTarget {
        id: animation_target_id,
        player: entity,
    });
}

fn setup_raven(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    animations: &mut ResMut<Assets<AnimationClip>>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
) {
    let name = Name::new("raven");

    // https://bevyengine.org/examples/animation/animated-transform/
    let animation_target_id = AnimationTargetId::from_name(&name);

    let mut animation = AnimationClip::default();

    animation.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new([
                (0.0, Vec3::new(0.0, -200.0, -1.0)),
                (APPEAR, Vec3::new(0.0, -200.0, -1.0)),
                (APPEAR + 1.0, Vec3::new(0.0, 20.0, -1.0)),
                (ATTACK + 0.0, Vec3::new(0.0, 20.0, 1.0)),
                (ATTACK + 1.0, Vec3::new(0.0, -200.0, 1.0)),
            ])
            .expect("should be able to build translation curve because we pass in valid samples"),
        ),
    );

    animation.add_event(CRY, OpeningEvent::SE(SE::Dragon));
    animation.add_event(APPEAR + 0.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 1.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 2.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 3.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 4.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 5.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 6.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 7.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(APPEAR + 8.0, OpeningEvent::SE(SE::DragonFlutter));
    animation.add_event(
        ATTACK,
        OpeningEvent::Animate {
            name: "raven".to_string(),
            tag: "attack".to_string(),
            frame: 4,
        },
    );

    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    let mut player = AnimationPlayer::default();
    player.play(animation_index);

    let entity = commands
        .spawn((
            name,
            Raven,
            StateScoped(GameState::Opening),
            AseSpriteAnimation {
                aseprite: assets.raven.clone(),
                animation: "idle".into(),
            },
            Transform::default(),
            AnimationGraphHandle(graphs.add(graph)),
            player,
        ))
        .id();

    commands.entity(entity).insert(AnimationTarget {
        id: animation_target_id,
        player: entity,
    });
}

fn timeline(mut count: ResMut<OpeningCount>) {
    count.count += 1;
}

fn click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut writer: EventWriter<SEEvent>,
    mut overlay_event_writer: EventWriter<OverlayEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        writer.send(SEEvent::new(SE::Click));
        overlay_event_writer.send(OverlayEvent::Close(GameState::InGame));
    }
}

pub struct OpeningPlugin;

impl Plugin for OpeningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OpeningCount { count: 0 });
        app.add_systems(OnEnter(GameState::Opening), setup);
        app.add_systems(
            Update,
            (timeline, click).run_if(in_state(GameState::Opening)),
        );
        app.add_event::<OpeningEvent>();
        app.add_observer(
            |trigger: Trigger<OpeningEvent>,
             mut se_writer: EventWriter<SEEvent>,
             mut overlay_event_writer: EventWriter<OverlayEvent>,
             mut query_raven: Query<(&Name, &mut AseSpriteAnimation, &mut AnimationState)>,
             mut next_bgm: ResMut<NextBGM>| match trigger.event() {
                OpeningEvent::SE(se) => {
                    se_writer.send(SEEvent::new(*se));
                }
                OpeningEvent::Close => {
                    overlay_event_writer.send(OverlayEvent::Close(GameState::InGame));
                }
                OpeningEvent::Animate { name, tag, frame } => {
                    for (sprite_name, mut raven_animation, mut raven_state) in
                        query_raven.iter_mut()
                    {
                        if sprite_name.as_str() == name {
                            raven_animation.animation.tag = Some(tag.clone());
                            raven_state.current_frame = *frame;
                        }
                    }
                }
                OpeningEvent::FadeIn => {
                    overlay_event_writer.send(OverlayEvent::SetOpen(true));
                }
                OpeningEvent::FadeOut => {
                    overlay_event_writer.send(OverlayEvent::SetOpen(false));
                }
                OpeningEvent::BGM(handle) => next_bgm.0 = handle.clone(),
            },
        );
    }
}
