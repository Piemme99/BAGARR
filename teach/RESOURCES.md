# Compilation Rust — Resources

## Knowledge

- [The Cargo Book — Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
  Référence officielle des profils (`dev`, `release`) et de tous les réglages : `opt-level`,
  `codegen-units`, `lto`, `incremental`, overrides par package (`[profile.dev.package."*"]`).
  Use for: la sémantique exacte de chaque réglage de build.
- [The Rust Book — Ch. 14.1: Customizing Builds with Release Profiles](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
  Intro pédagogique aux profils et à `opt-level` (défauts : dev=0, release=3).
  Use for: la première intuition du compromis compile-time / run-time.
- [rustc dev guide — Overview of the Compiler](https://rustc-dev-guide.rust-lang.org/overview.html)
  Le pipeline complet de rustc : lexing/parsing → HIR → MIR → codegen LLVM.
  Use for: comprendre les étapes entre le `.rs` et le code machine.
- [rustc dev guide — Monomorphization](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)
  Comment les génériques sont "dépliés" par type concret au moment du codegen.
  Use for: LA raison de fond des temps de compil (surtout avec Bevy).
- [matklad — Fast Rust Builds](https://matklad.github.io/2021/09/04/fast-rust-builds.html)
  Article de référence (auteur de rust-analyzer) sur ce qui rend un build Rust lent et comment
  l'accélérer ; explique la duplication de monomorphisation entre crates.
  Use for: modèle mental des builds lents et leviers concrets.
- [Bevy — Setup / Enable Fast Compiles](https://bevy.org/learn/quick-start/getting-started/setup/)
  Doc officielle Bevy : linker rapide (mold/lld), `dynamic_linking`, avertissement de shipping.
  Use for: la config exacte recommandée par Bevy et ses caveats.
- [Unofficial Bevy Cheat Book — Customizing Bevy](https://bevy-cheatbook.github.io/setup/bevy-config.html)
  Détails communautaires sur features Bevy et config de dev.
  Use for: compléments pratiques sur les features et le setup.
- [mold — dépôt officiel (rui314/mold)](https://github.com/rui314/mold)
  Le linker rapide qu'on utilise. Explique ce qu'est un linker et pourquoi mold est rapide.
  Use for: comprendre l'étape de linking et le rôle de mold.
- [Bevy Discussion #8991 — mold linker and dynamic linking](https://github.com/bevyengine/bevy/discussions/8991)
  Discussion officielle sur combiner mold et `dynamic_linking`.
  Use for: nuances sur l'interaction des deux optimisations.

### Écosystème d'outils (rustup / cargo / rustc / crates.io)
- [The rustup book](https://rust-lang.github.io/rustup/)
  Doc officielle du gestionnaire de toolchains : canaux (stable/beta/nightly), toolchains,
  composants, profils d'install. Use for: tout ce qui touche aux versions de Rust.
- [rustup book — Toolchains](https://rust-lang.github.io/rustup/concepts/toolchains.html)
  Définition précise d'une toolchain et des canaux. Use for: comprendre `rustup update/default`.
- [The Cargo Book — Guide](https://doc.rust-lang.org/cargo/)
  Doc officielle de cargo dans son ensemble. Use for: workflow projet, commandes cargo.
- [Cargo Book — Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
  Contraintes de version (caret `^`), plages semver, sources. Use for: comprendre `bevy = "0.19"`.
- [Cargo Book — Dependency Resolution & Cargo.lock](https://doc.rust-lang.org/cargo/reference/resolver.html)
  Comment cargo choisit et fige les versions dans `Cargo.lock`. Use for: le rôle du lockfile.
- [The Rust Book — Ch.1 Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
  Point d'entrée officiel, installe rustup+cargo+rustc d'un coup. Use for: vue d'ensemble débutant.

### ECS Bevy (Entity / Component / System, App, Commands, Bundle…)
- [Bevy — Quick Start : ECS](https://bevy.org/learn/quick-start/getting-started/ecs/)
  Intro officielle au paradigme ECS de Bevy (entités, composants, systèmes, App). Use for: le
  modèle mental de base.
- [Unofficial Bevy Cheat Book — Entities & Components](https://bevy-cheatbook.github.io/programming/ec.html)
  Explication claire entité=id / composant=donnée. Use for: bien distinguer les deux.
- [Bevy Cheat Book — Bundles](https://bevy-cheatbook.github.io/programming/bundle.html)
  Ce qu'est un bundle (collection de composants), et le lien avec les tuples. Use for: `spawn((A,B))`.
- [Bevy 0.15 — Required Components (release notes)](https://bevy.org/news/bevy-0-15/)
  Annonce du mécanisme qui a déprécié les bundle-structs (SpriteBundle…) au profit de `#[require]`.
  Use for: comprendre pourquoi `spawn(Camera2d)` suffit en 0.19.
- [Bevy Cheat Book — Commands](https://bevy-cheatbook.github.io/programming/commands.html)
  Rôle de `Commands` et pourquoi les changements structurels sont différés. Use for: spawn/despawn.

### Plugins Bevy
- [Bevy — Quick Start : Plugins](https://bevy.org/learn/quick-start/getting-started/plugins/)
  Intro officielle : plugin = feature branchée dans l'App, `DefaultPlugins`. Use for: le concept.
- [Bevy Cheat Book — Plugins](https://bevy-cheatbook.github.io/programming/plugins.html)
  Écrire un plugin, PluginGroup, `.set()`/`.disable()`. Use for: organiser le code par feature.
- [docs.rs — DefaultPlugins](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html)
  Liste des plugins inclus. Use for: savoir ce que fournit `DefaultPlugins`.
- [docs.rs — PluginGroupBuilder](https://docs.rs/bevy/latest/bevy/app/struct.PluginGroupBuilder.html)
  `.set` / `.disable` / `.add` / ordre de build. Use for: configurer/retirer un plugin d'un groupe.
- Source vérifiée : `bevy_internal-0.19.0/src/default_plugins.rs` (composition exacte en 0.19).

### Paramètres de système (Res, ResMut, Query, Commands…)
- [Bevy Cheat Book — Systems](https://bevy-cheatbook.github.io/programming/systems.html)
  Ce qu'est un système, quels paramètres il peut prendre, regroupement en tuples (max 16).
  Use for: le vocabulaire des paramètres.
- [Bevy Cheat Book — Resources](https://bevy-cheatbook.github.io/programming/res.html)
  `Res` / `ResMut`, données globales. Use for: accès aux ressources.
- [Bevy Cheat Book — Local Resources](https://bevy-cheatbook.github.io/programming/local.html)
  `Local<T>` — état privé à un système. Use for: mémoire propre à un système.
- [docs.rs — ResMut](https://docs.rs/bevy/latest/bevy/ecs/system/struct.ResMut.html)
  Le type exact et sa durée de vie. Use for: comprendre le wrapper.

### World & niveaux d'accès (vues, Commands, &mut World, FromWorld)
- [Bevy Cheat Book — Intro: Your Data](https://bevy-cheatbook.github.io/programming/intro-data.html)
  Le World comme conteneur de tout l'état (entités/composants/resources). Use for: le modèle mental
  « base de données ».
- [Bevy Cheat Book — Direct World Access](https://bevy-cheatbook.github.io/programming/world.html)
  Accéder au World en direct via `&mut World`. Use for: ce qu'on gagne/perd avec l'accès exclusif.
- [Bevy Cheat Book — Exclusive Systems](https://bevy-cheatbook.github.io/programming/exclusive.html)
  Systèmes `&mut World` : quand et pourquoi (rarement). Use for: les cas d'usage légitimes.
- [docs.rs — World](https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html)
  L'API exacte (`resource_mut`, `spawn`, …). Use for: signatures précises en 0.19.
- [docs.rs — FromWorld](https://docs.rs/bevy/latest/bevy/ecs/world/trait.FromWorld.html)
  Le trait derrière `init_resource` ; blanket impl pour tout `T: Default`. Use for: resources qui ne
  se construisent pas à partir de rien (ex. handles d'assets).
- Source vérifiée 0.19 : `bevy_ecs-0.19.0/src/world/mod.rs:3995` (trait `FromWorld` + blanket impl
  Default), `bevy_asset-0.19.0/src/handle.rs:134` (`Handle::Strong(Arc<StrongHandle>)`).

### Timer & temps
- [Bevy Cheat Book — Time and Timers](https://bevy-cheatbook.github.io/fundamentals/time.html)
  `Timer`, `tick`, `finished`/`just_finished`, `TimerMode`. Use for: chronos périodiques.
- [docs.rs — Timer](https://docs.rs/bevy/latest/bevy/time/struct.Timer.html)
  API exacte du Timer. Use for: signatures précises.
- [Bevy examples — time/timers.rs](https://github.com/bevyengine/bevy/blob/main/examples/time/timers.rs)
  Exemple canonique d'usage. Use for: modèle de code.

### Rust : enums, ownership, Copy/Clone (Rust Book officiel)
- [The Rust Book — Ch.6 : Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
  Enums, variantes (avec/sans données), `match` exhaustif. Use for: modéliser un choix fini.
- [The Rust Book — Ch.4.1 : What Is Ownership? (move, Copy)](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
  Ownership, sémantique de *move*, trait `Copy`. Use for: comprendre pourquoi indexer *move*.
- [The Rust Book — Appendix C : Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
  Ce que fait `#[derive(...)]`, `Clone` vs `Copy`, etc. Use for: pourquoi/quoi dériver.
- [The Rust Book — Ch.19.3 : Pattern Syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
  Toute la richesse du pattern matching. Use for: aller plus loin que `match` simple.

### Communication & découplage entre systèmes
- [Bevy Cheat Book — Events](https://bevy-cheatbook.github.io/programming/events.html)
  Le pattern producteur/consommateurs découplé. ⚠️ terminologie « Event » du cheatbook = **`Message`**
  en Bevy 0.19 (renommé : `MessageWriter`/`MessageReader`/`.write()`/`.read()`/`add_message`). Use for:
  le concept, en traduisant les noms.
- [Bevy Cheat Book — System Order of Execution](https://bevy-cheatbook.github.io/programming/system-order.html)
  `.before`/`.after`/`.chain`, dépendances déclaratives, parallélisme. Use for: ordonner sans coupler.
- [Bevy Cheat Book — Schedules](https://bevy-cheatbook.github.io/programming/schedules.html)
  Les schedules (First/PreUpdate/Update/PostUpdate/Last…). Use for: où placer un système.
- Source vérifiée 0.19 : `bevy_ecs/src/message/*` (Messages), `bevy_ecs/src/observer/*` (`On<E>`,
  `add_observer`) — l'API réelle diffère des tutos « Event/EventWriter/send ».

## Wisdom (Communities)

- [Bevy Discord (officiel)](https://discord.gg/bevy)
  Communauté très active, canaux #help et #showcase. Use for: questions de setup/build en direct,
  retours sur du code Bevy réel.
- [r/rust](https://reddit.com/r/rust)
  Subreddit à fort signal sur le langage et l'écosystème. Use for: questions générales Rust, temps
  de compil, tooling.
- [Bevy GitHub Discussions](https://github.com/bevyengine/bevy/discussions)
  Discussions officielles archivées et cherchables. Use for: problèmes de build documentés par
  d'autres.

_Préférence communauté de l'utilisateur : non encore exprimée._
