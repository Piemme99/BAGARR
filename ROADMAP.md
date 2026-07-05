# BAGARR — Roadmap

Mini vampire-survivor pour apprendre Bevy. Objectif : toucher un max de concepts Bevy,
mais **le MVP jouable passe avant le polish**.

- **Moteur** : Bevy 0.19 · Rust stable
- **Genre** : vampire-survivor, uniquement la "bagarre"
- **Méthode** : Claude explique concept + API, l'utilisateur écrit tout le code du jeu.

## Décisions verrouillées

| Sujet | Décision |
|---|---|
| Vue | 2D top-down, caméra 2D orthographique statique |
| Terrain | Arène fixe bornée (un seul écran) |
| Joueur | Déplacement 8 directions clavier (ZQSD), vitesse normalisée |
| Attaque | Manuelle au clic gauche, zone circulaire projetée vers le curseur |
| Ennemis | Spawn périodique depuis les bords, foncent en ligne droite sur le joueur |
| Collisions | Faites à la main (distance entre cercles), aucune lib physique |
| Rendu | Formes géométriques colorées d'abord ; sprites/animations en phase polish |
| Mort du joueur | Pas dans le MVP (ajoutée en phase 2) |
| Compile | mold + `bevy/dynamic_linking` (dev) + `opt-level` deps |

---

## Phase 0 — Mise en place ⚙️

- [x] Installer le linker `mold`
- [x] `cargo init` du projet
- [x] `Cargo.toml` : dépendance Bevy 0.19 + profils de compilation
- [x] `.cargo/config.toml` : linker mold + alias `cargo r` (dynamic_linking)
- [x] Fenêtre qui s'ouvre + caméra 2D → écran vide qui tourne (`cargo r`)
- **Concepts** : structure d'un projet Cargo, `App`, `DefaultPlugins`, `Startup`, `Commands`, `Camera2d`
- **Fiches teach/** : 01 compilation & profils · 02 écosystème d'outils · 03 modèle ECS

## Phase 1 — Le joueur bouge 🕹️

- [x] Composant `Player`, spawn d'une forme au centre
- [x] Lire le clavier et déplacer le joueur (8 directions, normalisé, delta time)
- [x] Borner le joueur dans l'arène (via `Query<&Window, With<PrimaryWindow>>`, `clamp`)
- **Concepts** : `Component`, `Query`, filtre `With`, `Res<ButtonInput>`, `Res<Time>`, systèmes `Update`, `Transform`, `single()`/`Result`, `let-else`

## Phase 2 — Les ennemis 👾

- [x] Composant `Enemy`, spawn manuel d'un ennemi (triangle via `Mesh2d` + `MeshMaterial2d`)
- [x] Système de spawn périodique depuis les bords (`Timer` / `Resource`, positions aléatoires via `rand` + `enum Edge`)
- [x] Ennemis qui se dirigent vers le joueur chaque frame (+ rotation vers la cible, itération sur la query)
- **Concepts** : `Resource`, `Timer`, requêtes croisées (joueur + ennemis), maths de vecteurs, `Mesh2d`/`Assets`/`Handle`, `Quat`, itération de `Query`
- **Fiches teach/** : 04 plugins · 05 paramètres de système · 06 Timer

## Phase 3 — La bagarre 👊

- [x] Détecter le clic gauche + convertir la position curseur en coordonnées monde (`viewport_to_world_2d`, `run_if(input_just_pressed)`)
- [x] Zone de coup circulaire projetée devant le joueur vers le curseur (cooldown reporté — clic manuel = cadence naturelle)
- [x] Tuer les ennemis touchés (collision cercle-cercle maison via `distance_squared`, `despawn`)
- [ ] Contact ennemi↔joueur (pour l'instant : sans effet, juste détecté)
- **Concepts** : `Window`/`Camera` → `viewport_to_world_2d`, `MouseButton`, `run_if`/run conditions, `despawn`, `Gizmos` (debug), collisions maison, fonctions partagées

## Phase 4 — Boucle jouable 🎯

- [x] Compteur de kills (ressource `Kills`)
- [x] Affichage du compteur à l'écran (UI texte : `Text`/`TextFont`/`Node`, MAJ depuis `Res<Kills>`)
- **Concepts** : Bevy UI (`Text`, `Node`, positionnement `Val::Px`/`PositionType`), ressource partagée entre plugins
- ✅ **MVP ATTEINT** 🎉 (jeu jouable : déplacement + ennemis + attaque + score)

---

## Phase 5+ — Polish (après le MVP)

Piochées selon l'envie, pour explorer d'autres pans de Bevy :

- [ ] Sprites à la place des formes (chargement d'assets, `AssetServer`)
- [ ] Animations de sprites (spritesheets, `TextureAtlas`, timer d'animation)
- [ ] PV du joueur + game-over + écran de fin (`States`)
- [ ] Feedback : flash de dégât, petites particules, camera shake
- [ ] Sons (coups, morts d'ennemis)
- [ ] Vagues de difficulté croissante
- [ ] Menu de démarrage
- [ ] Plusieurs types d'ennemis

_Maintenu par Claude. Dernière mise à jour : 2026-07-04._
