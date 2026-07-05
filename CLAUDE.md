# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Ce projet est avant tout un support d'apprentissage

BAGARR est un mini vampire-survivor (uniquement la phase de "bagarre") dont le **but réel est
d'apprendre Bevy et Rust**. L'utilisateur est un SWE expérimenté mais **débutant en Rust et en Bevy**,
francophone, clavier AZERTY. Le jeu lui-même est secondaire : c'est le prétexte pour toucher un
maximum de concepts Bevy. **Priorité : le MVP jouable avant tout polish.**

Réponds en **français**.

## ⚠️ Mode de collaboration (le plus important) — comment écrire du code ici

Il ne s'agit PAS de livrer du code fini. On sépare deux catégories :

- **Boilerplate / plomberie / config** (câblage de l'`App`, setup, structures imposées par le
  framework) → **Claude l'écrit**, mais *petit à petit* (pas de gros pavé d'un coup), avec des
  **commentaires explicatifs directement dans le code**. L'utilisateur supprime ces commentaires au
  fur et à mesure qu'il comprend (ils servent de curseur de compréhension).
- **Logique de gameplay** (déplacement, collisions, spawn, attaque… le vrai apprentissage) → **Claude
  explique le concept Bevy + la surface d'API pertinente** (types, signatures, mini-exemples isolés),
  puis **c'est l'utilisateur qui écrit la logique lui-même**. Ne pas écrire cette logique à sa place.

L'utilisateur a explicitement rejeté le mode "Claude donne le code, l'utilisateur recopie". Recopier
sans produire n'apprend rien.

**Précision d'API impérative** : on est sur Bevy 0.19, dont l'API diffère beaucoup des versions
antérieures (tutos en ligne souvent périmés). **Vérifie l'API réelle dans le source** avant d'affirmer
quoi que ce soit — le code des crates est dans
`~/.cargo/registry/src/index.crates.io-*/bevy*-0.19.0/`. Ne te fie pas à ta mémoire pour les
signatures Bevy.

## Commandes

```bash
cargo r                    # LANCER en dev : alias (défini dans .cargo/config.toml) =
                           #   run --features bevy/dynamic_linking. Itération rapide.
cargo run --release        # build optimisé (lent à compiler, fluide à l'exécution)
cargo build                # compiler sans lancer
cargo check                # vérifier la compilation sans produire de binaire (rapide)
cargo tree                 # visualiser l'arbre de dépendances Bevy
rustup update stable       # MAJ toolchain (Bevy 0.19 exige rustc >= 1.95 ; MSRV)
xdg-open teach/explainers/NN-*.html   # ouvrir une fiche de cours
```

Setup compilation rapide déjà en place : linker **mold** + feature **dynamic_linking** (via l'alias
`cargo r`) + split des profils (notre code `opt-level=1`, dépendances `opt-level=3`). Le *pourquoi*
est détaillé dans `teach/explainers/01-compilation-rust.html`. `dynamic_linking` est réservé au dev
(via `cargo r`) — **ne jamais l'utiliser pour une build distribuée** (`cargo build --release` reste
propre et statique).

## Structure du dépôt

- `src/main.rs` — le jeu. Actuellement minimal : `App` + `DefaultPlugins` + une `Camera2d` (Phase 0).
- `ROADMAP.md` — **source de vérité du plan**. Phases cochées au fur et à mesure, décisions
  verrouillées (table en tête de fichier), concepts Bevy abordés par phase. **Maintenu par Claude :
  cocher les étapes finies et garder à jour.**
- `teach/` — carnet de cours géré par le skill `/teach` (workspace d'apprentissage stateful) :
  - `MISSION.md`, `GLOSSARY.md`, `RESOURCES.md` (sources de confiance), `learning-records/`
  - `explainers/*.html` — fiches interactives écrites par Claude, **sourcées** (citations vers doc
    officielle), avec quiz et encarts "Essaie ça". C'est du contenu pédagogique (≠ code du jeu).
  - Quand un concept mérite un approfondissement, créer une nouvelle fiche numérotée + l'indexer.

## Décisions de conception verrouillées (détail dans ROADMAP.md)

2D top-down · **arène fixe bornée, caméra statique** · déplacement joueur 8 directions ZQSD (vecteur
normalisé) · **attaque manuelle au clic gauche** vers le curseur · ennemis qui spawnent aux bords et
foncent en ligne droite · **collisions faites à la main** (distance cercle-cercle, pas de lib
physique) · **formes géométriques colorées d'abord**, sprites/animations en phase polish · pas de
PV/mort du joueur dans le MVP.
