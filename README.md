# Elevage pokemons

J'ai utilisé Tauri pour réaliser une application bureau qui utilise Rust en backend et du html / js côté frontend. 

La fonctionnalité de reproduction est disponible côté backend mais je n'ai pas eu le temps (ni l'énergie) de finir 
l'implémentation côté frontend. 

La gestion d'erreur n'est pas complète et il n'y a pas toutes les validations, par exemple il est possible de créer un pokemon sans nom !

## Démarrer l'application

Se placer dans le dossier `src-tauri` et lancer la commande : 
```shell
cargo run dev
```