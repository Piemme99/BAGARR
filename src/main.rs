// Les types de query Bevy (filtres With/Without, tuples) sont jugés "complexes" par clippy.
// C'est un faux positif récurrent et bien connu en Bevy → on l'autorise pour tout le crate.
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod combat; // charge src/combat.rs
mod enemy; // charge src/enemy.rs
mod hud; // charge src/hud.rs
mod player; // charge src/player.rs
use crate::combat::CombatPlugin;
use crate::enemy::EnemyPlugin;
use crate::hud::HudPlugin;
use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(HudPlugin)
        .add_systems(Startup, setup) // ne reste que la caméra (globale)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
