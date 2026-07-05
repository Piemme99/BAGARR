# Mission: Comprendre la compilation Rust (dans le contexte Bevy/BAGARR)

## Why
Apprendre Bevy en construisant BAGARR, un mini vampire-survivor. Bevy est lent à compiler : sans
comprendre *pourquoi* et *ce que font* les réglages de build, on subit les temps de compil ou on
copie-colle une config magique sans la maîtriser. Le but est de rendre l'itération rapide **en
comprenant** chaque choix, pour que l'apprentissage de Bevy ne soit jamais bloqué par le build.

## Success looks like
- Savoir décrire, à gros grain, ce qui se passe entre `cargo run` et l'exécutable qui se lance.
- Expliquer avec ses mots pourquoi Rust (et Bevy en particulier) est long à compiler.
- Choisir en connaissance de cause un `opt-level` et un profil selon qu'on veut compiler vite ou
  tourner vite — et justifier le split "code léger / deps optimisées".
- Savoir ce que mold et `bevy/dynamic_linking` accélèrent (l'étape de linking) et pourquoi on ne
  ship pas avec `dynamic_linking`.

## Constraints
- SWE expérimenté mais débutant Rust/Bevy — les analogies avec d'autres langages compilés aident.
- Français, clavier AZERTY, Arch Linux, Rust 1.92 stable, Bevy 0.19.
- Apprend en écrivant lui-même le code du jeu ; le contenu pédagogique est fourni par l'assistant.
- Préfère comprendre le "pourquoi" en profondeur, pas juste la recette.

## Out of scope
- Écriture d'un backend de compilateur, internes de LLVM au-delà du concept d'optimisation.
- Cross-compilation, WASM, packaging/distribution (hors "pourquoi on ne ship pas dynamic_linking").
- Optimisation fine des perfs du jeu au runtime (profilage, ECS scheduling) — plus tard.
