use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};

use crate::{
    enemy::Enemy,
    player::{PLAYER_INVULNERABILITY_SECS, Player},
};

const ATTACK_RADIUS: f32 = 40.0;
const ATTACK_REACH: f32 = 70.0;

// ---------- Plugin ----------

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EnemyKilled>();
        app.init_resource::<Kills>();
        app.add_systems(Update, attack.run_if(input_just_pressed(MouseButton::Left)));
        app.add_systems(Update, increment_kills);
        app.add_systems(Update, detect_enemy_contact);
        app.add_systems(Update, tick_invulnerability);
    }
}

// ---------- Ressources & messages ----------

#[derive(Resource, Default)]
pub struct Kills(pub u32);

#[derive(Message)]
pub struct EnemyKilled;

// ---------- Composants ----------

#[derive(Component)]
pub struct Hitbox(pub f32);

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Invulnerable(Timer);

// ---------- Systèmes ----------

fn attack(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    mut enemy_writer: MessageWriter<EnemyKilled>,
) {
    let Some(cursor_world) = cursor_world_position(&window_query, &camera_query) else {
        return;
    };
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let hit_center = attack_hit_center(player_pos, cursor_world);

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

fn detect_enemy_contact(
    mut player_query: Query<
        (Entity, &Transform, &Hitbox, &mut Health),
        (With<Player>, Without<Invulnerable>),
    >,
    enemy_query: Query<(&Transform, &Hitbox), With<Enemy>>,
    mut commands: Commands,
) {
    let Ok((player_entity, player_transform, &Hitbox(player_radius), mut player_health)) =
        player_query.single_mut()
    else {
        return;
    };
    let player_pos = player_transform.translation.truncate();
    for (enemy_transform, &Hitbox(enemy_radius)) in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation.truncate();
        if are_colliding(player_pos, player_radius, enemy_pos, enemy_radius) {
            player_health.0 -= 1.0;
            commands
                .entity(player_entity)
                .insert(Invulnerable(Timer::from_seconds(
                    PLAYER_INVULNERABILITY_SECS,
                    TimerMode::Once,
                )));
            info!("COLLISION");
            break;
        }
    }
}

fn tick_invulnerability(
    mut query: Query<(Entity, &mut Invulnerable)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut invulnerable) in query.iter_mut() {
        invulnerable.0.tick(time.delta());
        if invulnerable.0.is_finished() {
            commands.entity(entity).remove::<Invulnerable>();
        }
    }
}

// ---------- Calculs purs ----------

fn attack_hit_center(player_pos: Vec2, cursor_world: Vec2) -> Vec2 {
    let look_direction = (cursor_world - player_pos).normalize_or_zero();
    player_pos + look_direction * ATTACK_REACH
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

fn are_colliding(center1: Vec2, radius1: f32, center2: Vec2, radius2: f32) -> bool {
    let radius_total = radius1 + radius2;
    center1.distance_squared(center2) <= radius_total * radius_total
}

// ---------- Tests ----------

// `#[cfg(test)]` : ce module n'est compilé que par `cargo test`. Il n'existe pas dans le binaire
// du jeu. C'est la convention Rust : les tests unitaires vivent dans le fichier qu'ils testent.
#[cfg(test)]
mod tests {
    // `super` = le module parent (combat.rs). Le `*` importe tout, y compris les fonctions
    // privées : un module enfant voit les items privés de son parent.
    use super::*;

    // Chaque `#[test]` est une fonction sans paramètre ni retour, lancée isolément.
    // Elle réussit si elle se termine, échoue si un `assert!` panique.
    #[test]
    fn far_apart_circles_do_not_collide() {
        // distance 100, rayons 10 + 10 = 20 → pas de contact
        assert!(!are_colliding(
            Vec2::ZERO,
            10.0,
            Vec2::new(100.0, 0.0),
            10.0
        ));
    }

    #[test]
    fn overlapping_circles_collide() {
        // distance 15, rayons 10 + 10 = 20 → les disques se chevauchent
        assert!(are_colliding(Vec2::ZERO, 10.0, Vec2::new(15.0, 0.0), 10.0));
    }

    #[test]
    fn tangent_circles_collide() {
        // distance 20 = 10 + 10 exactement : le `<=` de la fonction dit que « toucher » compte.
        // Ce test fige cette décision ; si un jour tu passes en `<`, il te le rappellera.
        assert!(are_colliding(Vec2::ZERO, 10.0, Vec2::new(20.0, 0.0), 10.0));
    }

    #[test]
    fn nested_circles_collide() {
        // Un petit cercle entièrement à l'intérieur d'un grand : les bords ne se croisent pas,
        // mais il y a bien contact (distance 5 < 50 + 2).
        assert!(are_colliding(Vec2::ZERO, 50.0, Vec2::new(5.0, 0.0), 2.0));
    }

    #[test]
    fn collision_is_symmetric() {
        // L'ordre des deux cercles ne doit rien changer.
        let (a, ra, b, rb) = (Vec2::new(3.0, 4.0), 7.0, Vec2::new(-2.0, 1.0), 1.5);
        assert_eq!(are_colliding(a, ra, b, rb), are_colliding(b, rb, a, ra));
    }
}
