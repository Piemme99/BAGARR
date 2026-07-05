use bevy::{prelude::*, window::PrimaryWindow};

const PLAYER_COLOR: Color = Color::srgb(0.1, 0.9, 0.2);
const PLAYER_SHAPE: Vec2 = Vec2::new(50.0, 50.0);
const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Sprite::from_color(PLAYER_COLOR, PLAYER_SHAPE)));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut transform) = transform_query.single_mut() else {
        return;
    };

    // Déplacement : direction clavier, normalisée et indépendante du framerate.
    let direction = read_direction(&keyboard);
    transform.translation +=
        direction.normalize_or_zero().extend(0.0) * PLAYER_SPEED * time.delta_secs();

    // Bornage : on garde le centre du joueur dans les limites de la fenêtre.
    let Ok(window) = window_query.single() else {
        return;
    };
    let max_offset = window.size() / 2.0 - PLAYER_SHAPE / 2.0;
    transform.translation = transform
        .translation
        .clamp(-max_offset.extend(0.0), max_offset.extend(0.0));
}

/// Lit les touches ZQSD (positions physiques WASD) et renvoie un vecteur de
/// direction non normalisé (la normalisation est faite par l'appelant).
fn read_direction(keyboard: &ButtonInput<KeyCode>) -> Vec2 {
    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    direction
}
