# Projet Rust: *Fractal Explorer* <br> Architecture des Logiciels - 4<sup>ème</sup> année - ESGI

L'objet du projet est de composer un réseau de machines permettant d'explorer la géométrie
de [fractales](https://fr.wikipedia.org/wiki/Fractale).

Pour se faire, le réseau est composé d'un serveur réalisant la visualisation ou l'export des images et de travailleurs
qui se connectent au serveur pour récupérer des tâches de calcul pour une fraction de l'image.

La nature même des fractales que nous traiterons ici permet un découpage du rendu d'une image globale en fragments
indépendants et donc parfaitement calculables en parallèles par un réseau de machines.

## Errata

* Il y avait un doublon de définition du paramètre `max_iteration` entre `FragmentTask` et `JuliaDescriptor`; il a été
  retiré de ce dernier (`max_iteration` est donc mutualisé entre toutes les définitions de fractales).

## Scénario d'une exécution optimale

1. Le serveur démarre.

2. Les paramètres de simulation sont chargés:
    * définition de la fractale à générer avec ses paramètres spécifiques
    * coordonnées de la fenêtre de visualisation (exprimées dans l'espace *physique*)
    * résolution du rendu (dimensions de la fenêtre exprimées en pixels)

3. Boucle jusqu'à son interruption

   Le serveur est à l'écoute sur le port 8787 (par défaut) dans l'attente de la connexion d'un travailleur.

   Il existe plusieurs types d'interaction entre un travailleur et le serveur

    1. Le travailleur demande une charge de travail pour participer à l'œuvre collective.

       Ce message (du travailleur vers le serveur) est de type `FragmentRequest` et comprend
        * son identité (un nom de participant à des fins de statistiques)
        * une quantité maximale de pixels dont il peut supporter la charge de calcul (dans la version
          avancée, il y a un temps limite pour répondre; la charge peut changer en fonction de la zone de calcul et de
          la formule de la fractale en question).

       auquel le serveur répond par un message de type `FragmentTask` qui contient
        * un identifiant pour cette tâche
        * les paramètres de la fractale à traiter
        * les coordonnées de la fenêtre du rendu demandé (exprimées dans l'espace *physique* de la fractale)
        * la résolution du rendu demandé (dimensions de la fenêtre exprimées en pixels)

    2. Le travailleur retourne le résultat d'une charge de travail sous la forme d'un message `FragmentResult` qui
       contient:
        * l'identifiant pour cette tâche (comme envoyé dans le `FragmentTask`)
        * coordonnées de la fenêtre du rendu demandé (comme envoyé dans le `FragmentTask`)
        * résolution du rendu demandé (comme envoyé dans le `FragmentTask`)
        * la liste des pixels calculés (détail de cette
          structure [ci-dessous](#description-la-structure-des-pixels-calculés))

       à cela le serveur lui répondra soit un message de type `FragmentScore` qui correspond à une évaluation de la
       qualité du résultat retourné.

       Après avoir reçu un `FragmentResult` valide, le serveur met à jour la représentation de la fractale.

   À côté de cela, le serveur peut recevoir des directives de modification de la génération de la fractale soit
   interactivement, soit par d'autres commandes issues de messages réseau (dès lors, si un `FragmentResult` arrive alors
   que la configuration de la fractale a changé, celui-ci sera déclaré invalide et ignoré).

## Les fractales

### Généralité de calcul

Les calculs de fractales que nous considérons ici consistent en l'évaluation d'une suite $z_{n+1}=f(z_n)$ (à valeurs
dans le corps des nombres complexes) où l'évaluation s'arrête soit quand on observe que $z_n$ a convergé ou divergé (
voir le détail pour chaque fractale) ou si $n$ dépasse un maximum d'itérations défini par la fractale. Les dernières
valeurs de $z_n$ et de $n$ forment les composantes de `PixelIntensity`.

Les coordonnées du pixel pour lequel on évalue cette suite pourra être utilisé soit en tant que valeur de $z_0$ soit
comme un paramètre interne à la définition de $f$.
Dans tous les cas, le point physique (représenté comme un nombre complexe, dans l'espace défini par le
paramètre `range`) devra être pris au centre du pixel associé à la résolution.

### Les modèles de fractales

* [Julia](Julia.md)
* [Mandelbrot](Mandelbrot.md)
* [Iterated SinZ](IteratedSinZ.md)
* [Newton Raphson Z^n](NewtonRaphsonZn.md)

## Votre objectif

* Réaliser un travailleur écrit en Rust sans bibliothèque extérieure autres que celles autorisées.

  **C'est la partie principale du projet.**

  Le travailleur *doit* pouvoir être lancé de la manière suivante: `worker [server_address]`

  où
    * `server_address` représente l'adresse du serveur (nom ou IP).
    * le port de connexion est par défaut `8787`
    * le nom de connexion au serveur doit être celui de votre groupe

      (tel que défini dans myges, vous avez le droit d'y mettre un suffixe personnalisé et *inspiré*)

      (vous pouvez ajouter aussi des options complémentaires)

* Réaliser un serveur minimal qui permette de tester un travailleur.

  Un serveur de référence vous est fourni pour tester votre client. Vous pouvez le télécharger en tant que documents
  fournis pour les cours (sur https://myGES.fr). Les présentes instructions contiennent
  sa [documentation](DemoServer.md).

* Le travailleur doit savoir gérer plusieurs définitions de fractales (en commençant par les ensemble de Julia)

* Le travailleur doit pouvoir effectuer un rendu en local et sauvegarder le résultat dans une image

* Il ne doit pas y avoir de duplication de code entre le travailleur et le serveur.

  Vous définirez un "crate" pour:
    * Le travailleur
    * Le serveur
    * Les éléments communs au travailleur et au serveur
    * Les opérations mathématiques sur nombres complexes

## Les modalités de réalisation

* Le projet doit être traité par groupe de 3 ou 4 personnes

* Le code doit remis sous Git (github ou gitlab) **avec** une archive déposée dans MyGES (c'est cette archive qui fait
  foi en cas de litige).

  Le projet Git devra être créé à partir d'un *fork* du projet portant le sujet (et n'oubliez pas de m'en donner l'accès
  en lecture).

* Le code doit être fonctionnel sous Linux, macOS et Windows

* Le code devra être raisonnablement testé (par des tests unitaires et des tests d'intégration)

* Le code devra suivre les règles de codage défini par `rustfmt`

* Le code devra être documenté avec `rustdoc`

* La documentation devra être intégrée au dépôt du code et écrite au format Markdown.

* Les seuls modules (*aka* crates) autorisés ici sont:
    * [`serde`](https://crates.io/crates/serde) et [`serde_json`](https://crates.io/crates/serde_json) pour la
      sérilalisation/désérialisation
    * [`image`](https://crates.io/crates/image) pour l'export d'images

  et éventuellement si besoin (en rien indispensable):
    * `rand`
    * `clap`
    * `rayon`
    * `toml`
    * `anyhow`
    * `tracing`
    * [`pixels`](https://crates.io/crates/pixels), [`egui`](https://github.com/emilk/egui), [`druid`](https://github.com/linebender/druid)
      ou [`piston`](https://github.com/pistondevelopers/piston)[[`exemples`](https://github.com/pistondevelopers/piston-examples)]
      si vous envisagez de faire un mode
      graphique.

  Pour tout autre package, vous devrez demander un accord préalable.

Le jour de la soutenance orale, vous serez évalués sur:

* Le respect des consignes
* La fiabilité et le respect du protocole entre travailleur et serveur
* Le respect des idiomes Rust (dont la gestion des erreurs)
* L'organisation et la lisibilité du code
* Je veux tous les commits (depuis le premier qui est le clone de ce dépôt) avec l’identité de chacun des contributeurs;
  si vous n’apparaissez pas dans les commits de code, vous serez considérés avec un Malus
* Il y aura une note collective et une note individuelle.
* La doc Markdown doit mettre en évidence
    * Votre organisation du travail en tant qu'équipe
    * Votre démarche d'élaboration des différents composants du projet
    * Les spécificités de votre projet (i.e. ce qui n'est pas déjà dans le sujet)
    * Vos éventuels bonus (parmi la liste présentée ou bien d'autres si validés au préalable par l'enseignant)

      Les bonus ne sont pris en compte uniquement si le travailleur est fonctionnel (fonctionnement raisonnablement
      sans planter dans des situations "normales" de jeu). Le niveau minimal fonctionnel du travailleur et du serveur (
      en
      mode test de votre travailleur) défini la note de 10/20.
* Vous aurez aussi une modification, un petit développement à faire en live sur votre code pendant la soutenance.

## Bonus possibles:

* Réaliser une interface pour le travailleur et/ou le serveur.

  Le serveur peut être autonome et disposé de ses moyens de production de l'image, soit exposer une interface web pour
  servir l'image courante ou piloter les paramètres (avec des messages spécifiques)

* Ajouter une intégration continue qui permette de tester votre code travailleur et serveur (sous GitHub ou GitLab)

* Utilisation d'un fichier externe pour recharger des configurations intéressantes ou pour sauvegarder celle courante.

* Réduire au maximum (voire à zéro) les éléments suivants

  (ce qui est un élément très qualitatif pour vos codes en Rust en plus d'être un bonus dans le cadre de ce projet)
    * les `unwrap()`, les `expect()`, les `panic!()`
    * les `mut` (variables mutables)
    * les *warnings* de compilation

* Réussir à faire *crasher* le serveur de référence

* Optimisation spécifique tels que du parallélisme ou de la vectorisation ou même l'exploitation de méthodes avancées de
  calcul telles que la méthode des perturbations.

NB: Pour les *Bonus*, vous avez le droit d'employer des modules (*aka* crates) additionnels après une approbation
explicite de celui-ci (il pourra vous être demandé de justifier ce besoin).

## Le protocole d'échange

Les messages sont échangés à travers sur un flux TCP sous la forme d'une suite d'octets, résultat d'une sérialisation
d'une structure hybride mêlant une description JSON et des données binaires.

En effet, pour optimiser le transfert des informations de pixels (qui peuvent être volumineuses et pour lesquels, on ne
veut pas commettre d'arrondi)

Tous les messages sont de la forme:

| Total message size         | JSON message size          | JSON message     | Data...       |
|----------------------------|----------------------------|------------------|---------------|
| (u32 encodé en Big Endian) | (u32 encodé en Big Endian) | (encodé en utf8) | (binary data) |

La section *Data* est donc composée de (Total message size) - (JSON message size) octets et contient l'ensemble des
données correspondantes aux sections de type `*Data` (le décodage de cette section dépend des
paramètres `offset`, `count`).

### Description des messages

Tous ces messages sont transmis sous la forme d'une
sérialisation [JSON](https://fr.wikipedia.org/wiki/JavaScript_Object_Notation).

| Nom du message    | Champs du message                                                                                                        | Exemple                                                                                                                                                                                                                                       |
|-------------------|--------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `FragmentRequest` | `worker_name: String`<br/>`maximal_work_load: u32`                                                                       | `{"FragmentRequest":{"worker_name":"fractal painter","maximal_work_load":1000}}`                                                                                                                                                              |
| `FragmentTask`    | `id: U8Data`<br/>`fractal: *FractalDescriptor*`<br/>`max_iteration: u16`<br/>`resolution: Resolution`<br/>`range: Range` | `{"FragmentTask":{"id":{"offset":0,"count":8},"fractal":{"Julia":{"c":{"re":0.0,"im":0.1},"divergence_threshold_square":0.0}},"max_iteration":0,"resolution":{"nx":160,"ny":120},"range":{"min":{"x":0.0,"y":0.0},"max":{"x":1.0,"y":1.0}}}}` |
| `FragmentResult`  | `id: U8Data`<br/>`resolution: Resolution`<br/>`range: Range`<br/>`pixels: PixelData`                                     | `{"FragmentResult":{"id":{"offset":0,"count":8},"resolution":{"x":160,"y":120},"range":{"min":{"x":0.0,"y":0.0},"max":{"x":1.0,"y":1.0}},"pixels":{"offset":8,"count":19200}}}`                                                               |

Vous trouverez le détail de *FractalDescriptor* dans la description de chaque fractale.

### Séquencement des messages

![Séquencement des messages](images/Sequence.drawio.svg "Séquencement des messages")

### Les types complémentaires

| Nom du type      | Description du type            |
|------------------|--------------------------------|
| `Point`          | `x: f64`<br/>`y: f64`          |
| `Complex`        | `re: f64`<br/>`im: f64`        |
| `Range`          | `min: Point`<br/>`max: Point`  |
| `Resolution`     | `nx: u16`<br/>`ny: u16`        |
| `U8Data`         | `offset: u32`<br/>`count: u32` |
| `PixelData`      | `offset: u32`<br/>`count: u32` |
| `PixelIntensity` | `zn: f32`<br/>`count: f32`     |

Un objet de `PixelIntensity` correspond à la valeur donnée par la fonction *fractale* choisie pour un pixel.
L'attribut `zn` correspond au module en fin d'itération et `count` au nombre d'itérations effectués divisé par le nombre
maximum d'itérations. Les champs de `PixelIntensity` sont sérialisés en un flux octets (avec encodage Big Endian) dans
l'ordre `zn` puis `count` et les pixels d'un `FragmentResult` sont sérialisés successivement ligne par ligne, de gauche
à droite pour chaque ligne.

## Couleur et rendu par le serveur

Le serveur a pour mission de distribuer les tâches et de collecter les résultats.
Au moment de la mise en image des données de pixels (`PixelIntensity`), il faut appliquer une coloration, c'est ce qu'on
appelera la `ColorMap`.

Le serveur pourra produire le résultat soit sous la forme d'un fichier d'image, soit dans une fenêtre graphique (afin
d'avoir un code portable Linux/macOS/Windows, il vous est recommandé d'utiliser l'une des bibliothèques cités dans la
liste des *crates* autorisés.

## Notions abordées

* Réseau / mémoire partagée
* Performance de calculs
* Respect d'une API réseau
* Segmentation d'un projet en composants faiblement couplés
* Décomposition et implémentation en structures et traits
* `serde` pour le transfert des données
* Mise en place de tests unitaires et d'intégration

<!-- for PDF export using pandoc
---
title: "Project Rust"
subtitle: "Architecture des logiciels - 4ème année - ESGI"
author: Pascal HAVÉ \<training+esgi@haveneer.com\>
date: 21 octobre 2023
geometry: "left=1cm,right=1cm,top=1cm,bottom=2cm"
output: pdf_document
---
-->
