# HANDOFF — reprise de la review de code

> Fichier temporaire : à supprimer quand tous les points ci-dessous seront traités.
> Contexte de travail : session de review pédagogique du code (voir mode de collaboration
> dans `CLAUDE.md` — Claude explique concept + API, **l'utilisateur écrit la logique**).
> Une review complète a produit ~12 points ; on les traite un par un.

## Déjà traité (committé)

- **Point 1** — assets partagés : `EnemyAssets` (resource de `Handle`s) + `impl FromWorld`,
  clones au spawn. Voir `src/enemy.rs`.
- **Point 2** — `Kills` déménagé de `hud.rs` vers `combat.rs` (le producteur possède l'état).
- **Point 2-bis** — découplage via message : `EnemyKilled` (`#[derive(Message)]`), écrit par
  `attack`, lu par `increment_kills`. Le combat ne connaît plus le compteur.
- **Point 3** — `update_hud.run_if(resource_changed::<Kills>)` : le HUD ne tourne que si ça change.
- **Point 10** — `info!` de debug supprimé au passage.

## EN COURS — Point 4 : purifier les helpers de `combat.rs`

⚠️ Vérifier d'abord l'état de `src/combat.rs` : l'explication a été donnée en fin de session,
l'utilisateur n'a probablement encore rien écrit.

Constat : `attack_hit_center`, `kill_enemies_in_circle`, `cursor_world_position` prennent des
`Query`/`Commands`/`MessageWriter` entiers → logique intestable, couplée à l'ECS.
Cible (pattern « sandwich ») déjà expliquée à l'utilisateur :
- le **système** `attack` fait toute l'extraction ECS (fenêtre, caméra, curseur, positions) et
  tous les effets (despawn + write) ;
- au milieu, des **fonctions pures** sur données simples : calcul du centre d'attaque
  (`Vec2` joueur + `Vec2` curseur → `Vec2`), sélection des victimes
  (centre + positions `(Entity, Vec2)` → entités condamnées) ;
- `cursor_world_position` : pas puriflable (a besoin de `Camera`/`GlobalTransform`), mais lui
  passer les **composants** plutôt que les queries.
- Validation : écrire un test unitaire de la sélection des victimes (modèle : le test de
  `position_on_edge` dans `enemy.rs`). Si le test s'écrit sans Bevy, c'est gagné.

## Points restants (dans l'ordre recommandé)

- **Point 9** — l'« arène fixe » n'existe pas : les bornes = taille de fenêtre, lue à 2 endroits
  (`player.rs` clamp, `enemy.rs` spawn). Une redimension de fenêtre change l'arène, logique
  dupliquée. → source de vérité unique (`ARENA_HALF_SIZE` const ou resource). La ROADMAP verrouille
  « arène fixe bornée, caméra statique ».
- **Point 5** — `Without<Player>` superflu dans la query ennemis de `attack` (`combat.rs`) : les
  deux queries y sont en lecture seule → aucun conflit d'accès, le filtre est du bruit. (Il reste
  **requis** dans `update_enemies` : `&mut` + `&`.) Pédagogie : quand le filtre est nécessaire.
- **Point 6** — `Quat::from_rotation_arc_2d(Vec2::Y, direction)` dans `update_enemies`
  (`enemy.rs`) : si l'ennemi est pile sur le joueur, `normalize_or_zero()` renvoie `ZERO` → viole
  le contrat « inputs must be unit vectors » (panic si `glam_assert` actif ; vérifié dans glam
  0.30.10 : renvoie silencieusement l'identité sinon). → garder le cas dégénéré.
- **Point 7** — `enemy.rs::spawn_enemies` : le `tick` du timer est après l'early-return de la
  fenêtre (timer gelé si query échoue) ; et `just_finished()` ne compte pas les multi-finitions
  d'une frame de lag (`times_finished_this_tick()` existe, vérifié dans le source).
- **Point 8** — tout le monde à z=0 (joueur Sprite, ennemis Mesh2d) : ordre de dessin indéterminé.
- **Point 11** — micro-style : `add_plugins((A, B, ...))` et `add_systems(Update, (a, b))` en
  tuples ; ouvre la porte à l'ordonnancement explicite.

## Aparté hors review (à vérifier un jour)

- `cargo r` (alias avec `bevy/dynamic_linking`) est probablement cassé : `bevy_dylib` n'a pas de
  0.19.0 stable sur crates.io (uniquement des `-rc`). Constaté en lançant clippy avec ce feature.

## Questions laissées ouvertes avec l'utilisateur

- Module `score` dédié ? Réponse donnée : oui le jour où plusieurs systèmes produisent du score,
  non tant que seul le combat compte.
- `GLOSSARY.md` du carnet `teach/` : y ajouter *World*, *vue/SystemParam*, *point de
  synchronisation*, *FromWorld*, *message* une fois la maîtrise confirmée (règle du glossaire :
  seulement après démonstration). La fiche 09 (`teach/explainers/09-le-world.html`) et le
  learning record 0002 datent de cette session.

## Skills suggérés

- `/teach` — pour toute nouvelle fiche explainer si un concept mérite un approfondissement
  (workspace `teach/`, conventions des fiches existantes).

## Rappels de méthode (résumé, détail dans CLAUDE.md)

- Bevy **0.19** : vérifier l'API dans `~/.cargo/registry/src/index.crates.io-*/bevy*-0.19.0/`
  avant toute affirmation. Les tutos en ligne sont périmés (ex. Events → **Messages**).
- Ne pas écrire la logique de gameplay à la place de l'utilisateur.
- L'utilisateur perd le fil entre sessions espacées : commencer par un état des lieux du code.
