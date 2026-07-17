# rust-fuse

## Description

Création d'un filesystem FUSE en Rust pour les systèmes Linux.

## Contexte

Ce projet est une implémentation d'un filesystem FUSE en Rust pour les systèmes Linux. Il utilise la bibliothèque `fuse-rs` pour interagir avec le noyau Linux et la bibliothèque `libc` pour accéder aux fonctions système.

## Fonctionnalités

- `syscall tracing`: permet de tracer les appels système pour déboguer les problèmes.
- `eBPF support`: permet d'utiliser le programme de filtre eBPF pour améliorer les performances.

## Dépendances

- `fuse-rs`: bibliothèque pour interagir avec le noyau Linux.
- `libc`: bibliothèque pour accéder aux fonctions système.
- `log`: bibliothèque de logging.
- `pretty_env_logger`: bibliothèque pour logger les informations d'environnement.

## Structure du projet

* `src/main.rs`: fichier principal du projet.
* `src/fuse.rs`: implémentation de la bibliothèque FUSE.
* `src/module.rs`: module noyau pour la liaison FUSE.
* `tests/main.rs`: tests unitaires pour le filesystem.
* `Cargo.toml`: fichier de configuration de Cargo.

## Comment utiliser le projet

1. Cloner le projet avec `git clone`.
2. Récupérer les dépendances avec `cargo build`.
3. Compilez le projet avec `cargo run`.

## Aide

Pour obtenir de l'aide, consultez le manuel de la bibliothèque `fuse-rs`.

## Licence

Ce projet est distribué sous la licence MIT.