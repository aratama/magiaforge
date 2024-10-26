use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct HoverColor {
    pub none: Color,
    pub hovered: Color,
}

fn update(
    mut interactions: Query<
        (&HoverColor, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (colors, interaction, mut color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => *color = colors.hovered.into(),
            Interaction::None => {
                *color = colors.none.into();
            }
        }
    }
}

pub struct HoverColorPlugin;

impl Plugin for HoverColorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}
