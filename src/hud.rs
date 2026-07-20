use bevy::prelude::*;

use crate::combat::Kills;

#[derive(Component)]
pub struct KillsText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, update_hud.run_if(resource_changed::<Kills>));
    }
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        KillsText,
        Text::new("Kills: 0"),
        TextFont::from_font_size(30.0),
        Node {
            position_type: PositionType::Absolute, // positionnement libre (pas dans un flux)
            top: Val::Px(12.0),                    // à 12px du haut
            left: Val::Px(12.0),                   // à 12px de la gauche
            ..default()
        },
    ));
}

fn update_hud(kills: Res<Kills>, mut query: Query<&mut Text, With<KillsText>>) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };
    text.0 = format!("Kills: {}", kills.0);
}
