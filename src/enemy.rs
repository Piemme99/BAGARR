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

#[derive(Resource)]
struct EnemyAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mesh = meshes.add(ENEMY_SHAPE);

        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        let material = materials.add(ENEMY_COLOR);

        EnemyAssets { mesh, material }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyAssets>();
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
    enemy_assets: Res<EnemyAssets>,
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
        commands.spawn((
            Enemy,
            Mesh2d(enemy_assets.mesh.clone()),
            MeshMaterial2d(enemy_assets.material.clone()),
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
    position_on_edge(half_window, edge, rng)
}

fn position_on_edge(half_window: Vec2, edge: Edge, rng: &mut impl RngExt) -> Vec2 {
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
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec2;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn position_tombe_sur_un_bord() {
        let half = Vec2::new(400.0, 300.0);
        let mut rng = StdRng::seed_from_u64(42);

        for edge in [Edge::Top, Edge::Bottom, Edge::Right, Edge::Left] {
            let pos = position_on_edge(half, edge, &mut rng);

            assert!(
                (pos.x.abs() - half.x).abs() < f32::EPSILON
                    || (pos.y.abs() - half.y).abs() < f32::EPSILON
            )
        }
    }
}
