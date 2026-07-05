use bevy::{prelude::*, window::PrimaryWindow};
use rand::RngExt;

use crate::player::Player;

const ENEMY_COLOR: Color = Color::srgb(0.9, 0.1, 0.2);
const ENEMY_SHAPE: Triangle2d = Triangle2d::new(
    Vec2::new(0.0, 20.0),
    Vec2::new(-16.0, -20.0),
    Vec2::new(16.0, -20.0),
);
const ENEMY_SPEED: f32 = 150.0;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemies);
        app.add_systems(Update, update_enemies);
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let half_window = window.size() / 2.0;

    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.just_finished() {
        let mesh = meshes.add(ENEMY_SHAPE);
        let material = materials.add(ENEMY_COLOR);
        commands.spawn((
            Enemy,
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(
                generate_random_enemy_position(half_window, &mut rand::rng()).extend(0.0),
            ),
        ));
    }
}

fn update_enemies(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for mut enemy_transform in &mut enemy_query {
        let direction =
            (player_transform.translation - enemy_transform.translation).normalize_or_zero();
        enemy_transform.translation += direction * ENEMY_SPEED * time.delta_secs();
        enemy_transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, direction.truncate())
    }
}

#[derive(Copy, Clone)]
enum Edge {
    Top,
    Bottom,
    Right,
    Left,
}
fn generate_random_enemy_position(half_window: Vec2, rng: &mut impl RngExt) -> Vec2 {
    let edges = [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left];
    let edge = edges[rng.random_range(0..edges.len())];
    match edge {
        Edge::Top => Vec2::new(
            rng.random_range(-half_window.x..half_window.x),
            half_window.y,
        ),
        Edge::Bottom => Vec2::new(
            rng.random_range(-half_window.x..half_window.x),
            -half_window.y,
        ),
        Edge::Right => Vec2::new(
            half_window.x,
            rng.random_range(-half_window.y..half_window.y),
        ),
        Edge::Left => Vec2::new(
            -half_window.x,
            rng.random_range(-half_window.y..half_window.y),
        ),
    }
}
