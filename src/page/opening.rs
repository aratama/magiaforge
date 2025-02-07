use crate::asset::GameAssets;
use crate::audio::NextBGM;
use crate::hud::overlay::OverlayEvent;
use crate::language::M18NTtext;
use crate::message::SKIP;
use crate::registry::Registry;
use crate::se::SEEvent;
use crate::se::CLICK;
use crate::se::DAMAGE;
use crate::se::DRAGON;
use crate::se::DRAGON_FLUTTER;
use crate::se::SE_TAORERU;
use crate::states::GameState;
use bevy::animation::animated_field;
use bevy::animation::AnimationTarget;
use bevy::animation::AnimationTargetId;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AnimationState;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_aseprite_ultra::prelude::Aseprite;
use core::f32;

const FADE_IN: f32 = 3.0;
const CRY: f32 = FADE_IN + 5.0;
const APPEAR: f32 = CRY + 2.0;
const TURN: f32 = APPEAR + 1.0;
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
    SE(String),
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

#[derive(Component, Debug)]
struct SkipButton;

fn setup(
    mut commands: Commands,
    registry: Registry,
    asset_server: Res<AssetServer>,
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

    // 背景の空と月
    commands.spawn((
        Name::new("background"),
        StateScoped(GameState::Opening),
        AseSpriteAnimation {
            aseprite: registry.assets.opening.clone(),
            animation: "default".into(),
        },
        Transform::from_xyz(0.0, 0.0, -200.0),
    ));

    setup_cloud(
        &mut commands,
        &registry.assets.title_cloud,
        &mut animations,
        &mut graphs,
        // 雲が左右に流れるアニメーション
        // 幅1024ピクセルで、画面幅が320ピクセルなので、
        vec![
            (0.0, Vec3::new(320.0, 0.0, -1.0)),
            (3.0, Vec3::new(320.0, 0.0, -1.0)),
            (18.0, Vec3::new(-320.0, 0.0, -1.0)),
        ],
    );

    setup_cloud(
        &mut commands,
        &registry.assets.title_cloud2,
        &mut animations,
        &mut graphs,
        // 雲が左右に流れるアニメーション
        // 幅1024ピクセルで、画面幅が320ピクセルなので、320から-320で動かす
        vec![
            (0.0, Vec3::new(320.0, 0.0, -2.0)),
            (3.0, Vec3::new(320.0, 0.0, -2.0)),
            (32.0, Vec3::new(-320.0, 0.0, -2.0)),
        ],
    );

    setup_witch(
        &mut commands,
        &asset_server,
        &registry.assets,
        &mut animations,
        &mut graphs,
    );

    setup_raven(
        &mut commands,
        &registry.assets,
        &mut animations,
        &mut graphs,
    );

    commands
        .spawn((
            SkipButton,
            StateScoped(GameState::Opening),
            Button,
            BorderColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                bottom: Val::Px(10.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                width: Val::Px(100.0),
                height: Val::Px(25.0),
                ..default()
            },
        ))
        .with_child((
            M18NTtext::new(&SKIP.to_string()),
            TextColor::WHITE,
            TextFont {
                font: registry.assets.noto_sans_jp.clone(),
                font_size: 12.0,
                ..default()
            },
        ));
}

fn skip(
    interaction: Query<&Interaction, (With<SkipButton>, Changed<Interaction>)>,
    mut overlay: EventWriter<OverlayEvent>,
    mut se: EventWriter<SEEvent>,
) {
    for interaction in interaction.iter() {
        match interaction {
            Interaction::Pressed => {
                overlay.send(OverlayEvent::Close(GameState::InGame));
                se.send(SEEvent::new(CLICK));
            }
            _ => {}
        }
    }
}

fn setup_cloud(
    commands: &mut Commands,
    assets: &Handle<Aseprite>,
    animations: &mut ResMut<Assets<AnimationClip>>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
    translation_samples: Vec<(f32, Vec3)>,
) {
    let name = Name::new("cloud");

    let animation_target_id = AnimationTargetId::from_name(&name);

    let mut animation = AnimationClip::default();

    animation.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new(translation_samples).expect(
                "should be able to build translation curve because we pass in valid samples",
            ),
        ),
    );

    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    let mut player = AnimationPlayer::default();
    player.play(animation_index);

    let entity = commands
        .spawn((
            name,
            StateScoped(GameState::Opening),
            AseSpriteAnimation {
                aseprite: assets.clone(),
                animation: "default".into(),
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

fn setup_witch(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    assets: &Res<GameAssets>,
    animations: &mut ResMut<Assets<AnimationClip>>,
    graphs: &mut ResMut<Assets<AnimationGraph>>,
) {
    let name = Name::new("witch");

    let animation_target_id = AnimationTargetId::from_name(&name);

    let mut animation = AnimationClip::default();

    let mut translation_samples = vec![(0.0, Vec3::new(0.0, 0.0, 0.0))];

    // 浮遊アニメーション
    // 0.1秒刻みで HIT までキーフレームを打つ
    for i in 0..(HIT * 10.0).floor() as usize {
        translation_samples.push((
            0.1 * i as f32,
            Vec3::new(
                0.0 + (i as f32 * 0.1).cos() * 10.0,
                -20.0 + (i as f32 * 0.37).cos() * 5.0,
                0.0,
            ),
        ));
    }

    // 落下アニメーション
    translation_samples.push((HIT, Vec3::new(0.0, 0.0, 0.0)));
    translation_samples.push((HIT + 2.0, Vec3::new(0.0, -120.0, 0.0)));

    animation.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            UnevenSampleAutoCurve::new(translation_samples).expect(
                "should be able to build translation curve because we pass in valid samples",
            ),
        ),
    );

    let mut rotation_samples: Vec<(f32, Quat)> =
        vec![(0.0, Quat::from_rotation_z(f32::consts::PI * 0.0))];

    // 攻撃の瞬間から回転アニメーションを開始
    // 0.5 * PI づつキーフレームを打たないとうまく回転が補間されないことに注意
    for i in 0..12 {
        rotation_samples.push((
            HIT + 0.6 * i as f32,
            Quat::from_rotation_z(f32::consts::PI * 0.5 * i as f32),
        ));
    }

    animation.add_curve_to_target(
        animation_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new(rotation_samples).expect(
                "should be able to build translation curve because we pass in valid samples",
            ),
        ),
    );

    animation.add_event(
        0.0,
        OpeningEvent::BGM(Some(asset_server.load("audio/風が吹く1.ogg"))),
    );
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
    animation.add_event(HIT, OpeningEvent::SE(DAMAGE.to_string()));
    animation.add_event(FADE_OUT, OpeningEvent::FadeOut);
    animation.add_event(FADE_OUT, OpeningEvent::BGM(None));
    animation.add_event(TAORERU, OpeningEvent::SE(SE_TAORERU.to_string()));
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

    animation.add_event(CRY, OpeningEvent::SE(DRAGON.to_string()));
    animation.add_event(APPEAR + 0.0, OpeningEvent::SE(DRAGON_FLUTTER.to_string()));
    animation.add_event(APPEAR + 1.0, OpeningEvent::SE(DRAGON_FLUTTER.to_string()));
    animation.add_event(APPEAR + 2.0, OpeningEvent::SE(DRAGON_FLUTTER.to_string()));
    animation.add_event(APPEAR + 3.0, OpeningEvent::SE(DRAGON_FLUTTER.to_string()));
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

pub struct OpeningPlugin;

impl Plugin for OpeningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OpeningCount { count: 0 });
        app.add_systems(OnEnter(GameState::Opening), setup);
        app.add_systems(Update, skip);
        app.add_event::<OpeningEvent>();
        app.add_observer(
            |trigger: Trigger<OpeningEvent>,
             mut se_writer: EventWriter<SEEvent>,
             mut overlay_event_writer: EventWriter<OverlayEvent>,
             mut query_raven: Query<(&Name, &mut AseSpriteAnimation, &mut AnimationState)>,
             mut next_bgm: ResMut<NextBGM>| match trigger.event() {
                OpeningEvent::SE(se) => {
                    se_writer.send(SEEvent::new(se.clone()));
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
