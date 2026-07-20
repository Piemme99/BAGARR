use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};

use crate::{enemy::Enemy, player::Player};

const ATTACK_RADIUS: f32 = 40.0;
const ATTACK_REACH: f32 = 70.0;

pub struct CombatPlugin;

#[derive(Resource, Default)]
pub struct Kills(pub u32);

#[derive(Message)]
pub struct EnemyKilled;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EnemyKilled>();
        app.init_resource::<Kills>();
        app.add_systems(Update, attack.run_if(input_just_pressed(MouseButton::Left)));
        app.add_systems(Update, increment_kills);
    }
}

fn attack(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    commands: Commands,
    enemy_writer: MessageWriter<EnemyKilled>,
) {
    let Some(hit_center) = attack_hit_center(&window_query, &camera_query, &player_query) else {
        return;
    };
    kill_enemies_in_circle(hit_center, enemy_query, commands, enemy_writer);
}

fn attack_hit_center(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform)>,
    player_query: &Query<&Transform, With<Player>>,
) -> Option<Vec2> {
    let world_position = cursor_world_position(window_query, camera_query)?;
    let player_pos = player_query.single().ok()?.translation.truncate();
    let look_direction = (world_position - player_pos).normalize_or_zero();
    Some(player_pos + look_direction * ATTACK_REACH)
}

fn kill_enemies_in_circle(
    hit_center: Vec2,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    mut enemy_writer: MessageWriter<EnemyKilled>,
) {
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation.truncate();
        if enemy_pos.distance_squared(hit_center) <= ATTACK_RADIUS * ATTACK_RADIUS {
            commands.entity(enemy_entity).despawn();
            enemy_writer.write(EnemyKilled);
        }
    }
}

fn increment_kills(mut enemy_reader: MessageReader<EnemyKilled>, mut kills: ResMut<Kills>) {
    for _ in enemy_reader.read() {
        kills.0 += 1;
    }
}

/// Position du curseur convertie en coordonnées monde.
fn cursor_world_position(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    // window : la fenêtre principale (unique). `.ok()` : Result -> Option, `?` : sort si absente.
    let window = window_query.single().ok()?;
    // camera + camera_transform : la caméra et son transform absolu (nécessaire pour projeter).
    let (camera, camera_transform) = camera_query.single().ok()?;
    // cursor : position du curseur en PIXELS écran. `?` : None si le curseur est hors fenêtre.
    let cursor = window.cursor_position()?;
    // valeur de retour : pixels écran -> coordonnées MONDE. `.ok()` : Result -> Option renvoyé tel quel.
    camera.viewport_to_world_2d(camera_transform, cursor).ok()
}
