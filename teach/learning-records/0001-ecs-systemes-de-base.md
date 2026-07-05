# Sait écrire des systèmes ECS de base en autonomie

L'utilisateur a écrit lui-même (avec guidage concept+API seulement) les systèmes `move_player`
et `update_enemies` : déclaration de composants, `Query<&mut T, With/Without>`, `Res<Time>` +
`delta_secs()`, itération d'une query, maths de vecteurs (direction normalisée, déplacement,
rotation via `Quat`). Il lit et corrige les erreurs du compilateur (borrow E0502, exhaustivité de
`match`, move E0508) en comprenant la cause.

**Implications pour la suite** : ne plus ré-expliquer les bases ECS (système = fn à params injectés,
Query/Res/Commands, schedules Startup/Update). Le socle est acquis. Zones encore fraîches, à
consolider quand elles reviennent : ownership/borrow fin (move/Copy/Clone → fiche 07), Assets/Handle
(fiche à faire), pattern matching avancé. Terrain suivant : collisions maison (Phase 3), UI (Phase 4).
