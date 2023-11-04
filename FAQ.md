# Généralités sur le projet

## Structuration

* Il faut bien (au minimum) 4 crates : `worker`, `shared`, `server`, `complex` (nommage indicatif).
* Il faut bien un fichier `Cargo.toml` à la racine qui les unifie tous (le mot clef magique sera `[workspace]`
  et `members`).

Un exemple possible se trouve dans ma base d'exemples:
https://github.com/haveneer/rust-quicklook-training/tree/master/rs/full-tree

## Rejet par le serveur fourni des messages de votre client

Si vous rencontrez des « `message : Too large message size` » de la part du serveur fourni, il est bien possible que
vous n'ayez pas bien respecté le format de transfert.

En effet, tel qu'il est décrit
dans [le sujet](https://github.com/haveneer-training/Frakt#le-protocole-déchange), il y a
bien une part qui est la sérialisation en JSON du message souhaité (comme nous avons vu en cours) mais aussi le JSON
message size préalable qui doit contenir la taille du message total que vous lui envoyez.
En effet, un message est composé d'une partie JSON et d'une partie *Data*. Pour qu'il
soit plus facile de les séparer, le message contient des préfixes de taille pour l'ensemble du message `S_total` et la
partie
JSON `S_json`, ce qui permet à chaque fois de lire la taille totale du message `S_total` (entier de taille fixe) puis la
taille de la partie `S_json`, lire les octets destinés à encoder le message JSON et ensuite les octets restant pour le
bloc de données.
Cela demandera quelques primitives d'écriture/lecture sur TcpStream différentes de celles que nous avons vues (
lire un nombre, lire un nombre d'octets définis ; idem pour l'écriture).

Un *timeout* renvoyé par le serveur peut aussi être un comportement différent du même problème.
Pour préciser, si vous n'écrivez pas en ce moment les préfixes, ce sont les premiers caractères de votre message qui
sont lus par le serveur comme un entier 32 bits (4 octets). Par exemple un message "Hi" qui ne contient que 2 caractères
(ici 2 octets) sera insuffisant pour reconstruire l'entier 32 bits attendu par le serveur. Il attendra encore 2 octets
jusqu'au timeout.

## Sérialisation par `serde` des chaînes de caractères

Quand `serde` sérialise une chaîne de caractères, il protège tous les caractères d'échappement et les
délimiteurs `"..."`.

Ainsi les deux assertions suivantes sont vraies:

```rust
assert_eq!(
    serde_json::to_string("a\nb\"c"),
    Ok("\"a\\nb\\\"c\"") // string habituelles, il ne faut pas oublier de protéger spéciaux; moins facile à lire 
);
```

```rust
assert_eq!(
    serde_json::to_string("a\nb\"c"),
    Ok(r#""a\nb\"c""#) // raw string pas besoin de protéger les caractères; plus simple à lire
);
```