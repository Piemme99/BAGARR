# Maîtrise World / resources custom / messages / détection de changement

Session de review (juillet 2026). L'utilisateur a écrit lui-même, avec guidage concept+API :
une resource de handles (`EnemyAssets`) initialisée via `impl FromWorld` (emprunts séquentiels
du `world` correctement ordonnés après un E0499 compris et corrigé), le partage de `Handle`
par clonage, un découplage producteur/consommateurs par message (`EnemyKilled`,
`MessageWriter`/`MessageReader`), et une run condition `resource_changed::<Kills>`.

A verbalisé correctement : la différence `Res`/`ResMut` du point de vue de l'ordonnanceur,
la sémantique non-destructive du `MessageReader` (curseur `Local` privé vs tampon partagé en
lecture), la rétention ~2 frames du double buffer (pipeline « en retard mais jamais lossy »),
et la distinction entre `mut` de binding Rust et accès en écriture déclaré par le type.

**Implications pour la suite** : Default/FromWorld, Assets/Handle, messages et change detection
sont acquis — ne plus ré-expliquer, s'y référer (fiches 05, 07, 08, 09). La « fiche à faire »
Assets/Handle mentionnée dans le record 0001 est couverte (fiche 09 + pratique). Zones fraîches
à consolider quand elles reviennent : observers (`On<E>`, jamais pratiqué), ordonnancement
explicite (`.after()`/`.chain()`, théorie seulement), extraction de logique pure hors ECS
(point 4 de la review, en cours — voir HANDOFF.md à la racine).
