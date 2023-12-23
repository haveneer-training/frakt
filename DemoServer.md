# Utilisation du serveur de référence

Une fois téléchargé et dézippé, vous pouvez le lancer depuis la ligne de commandes (sur certains OS, il faudra peut-être
ajuster les permissions d'exécution).

# Les options en ligne de commandes

Une aide en ligne de commande est disponible:

```bash
$ ./server --help
```

```
Usage: server [OPTIONS]

Options:
      --port <PORT>
          Port to listen to

          [default: 8787]

      --host-address <HOST_ADDRESS>
          Address allowed to connect to.

          Should be different from localhost to allow external connections

          [default: localhost]

  -v, --verbose
          Display detail

          log will show code path

      --debug
          Log additional debug info

      --server-rendering
          The server also render fractals

      --ntiles <NTILES>
          The server also render fractals

          [default: 4]

      --random-tiles
          Random tiles

  -V, --version
          Print version and exit

  -h, --help
          Print help (see a summary with '-h')
```

Lors de la mise au point, pour voir quelques détails sur la communication worker-server, l'option `--debug` est
recommandée.

Par défaut, l'interface réseau d'écoute est celle associée à `localhost`. Ainsi, si vous souhaitez l'utiliser sur un
réseau (local par exemple), il faudra lui préciser l'interface d'écoute sous la forme:

```asm
$ ./server --host-address=192.168.1.99
```

(l'adresse n'est qu'un exemple)

## Les commandes 

- <kbd>+</kbd>, <kbd>P</kbd>: Zoom in.

- <kbd>-</kbd>, <kbd>M</kbd>: Zoom out.

- <kbd>↑</kbd>, <kbd>→</kbd>, <kbd>↓</kbd>, <kbd>←</kbd> : move window frame

- <kbd>SHIFT</kbd> + <kbd>↑</kbd>, <kbd>→</kbd>, <kbd>↓</kbd>, <kbd>←</kbd> : move window frame slowly

- <kbd>C</kbd>: change curve

- <kbd>A</kbd>: increase colormap size

- <kbd>Z</kbd>: decrease colormap size

- <kbd>H</kbd>: change colormap

- <kbd>C</kbd>: change curve

- <kbd>S</kbd>: Save current view

- <kbd>T</kbd>: Toggle server rendering (disable by default)

- <kbd>escape</kbd>: Quit

## Environnements testés:

* Windows: pas besoin d'installation complémentaire; doit fonctionner *out of the box*.

* macOS: pas besoin d'installation complémentaire; doit fonctionner *out of the box*.

* Linux: si ce n'est pas déjà le cas, vous aurez besoin de driver OpenGL.
 
  D'expérience, sur Ubuntu, j'ai eu à installer les packages suivants:

  ```bash
  apt install libegl1 libegl1-mesa
  ```   

## Exemple de sortie

```bash
RUST_LOG=trace ./server --debug --ntiles=1200
```

```
2023-12-23T21:01:24.122554Z DEBUG Write JSON message with 16 data bytes: FragmentTask(FragmentTask { id: U8Data { offset: 0, count: 16 }, fractal: Julia(JuliaDescriptor { c: Complex { re: 0.285, im: 0.013 }, divergence_threshold_square: 4.0 }), max_iteration: 64, resolution: Resolution { nx: 1, ny: 1 }, range: Range { min: Point { x: -1.2, y: -1.2 }, max: Point { x: -1.198, y: -1.198 } } })
2023-12-23T21:01:24.122598Z DEBUG Write total message size: 261
2023-12-23T21:01:24.122610Z DEBUG Write JSON message size: 245
2023-12-23T21:01:24.122614Z DEBUG Write string message: {"FragmentTask":{"id":{"offset":0,"count":16},"fractal":{"Julia":{"c":{"re":0.285,"im":0.013},"divergence_threshold_square":4.0}},"max_iteration":64,"resolution":{"nx":1,"ny":1},"range":{"min":{"x":-1.2,"y":-1.2},"max":{"x":-1.198,"y":-1.198}}}}
2023-12-23T21:01:24.122617Z TRACE Write data: [00, 00, 00, 00, 00, 00, 00, 00, F0, 15, 99, 71, 45, D6, C8, 8E]
2023-12-23T21:01:24.123746Z DEBUG Worker 2 got a job; executing.
2023-12-23T21:01:24.123754Z DEBUG New connection from [::1]:54745
2023-12-23T21:01:24.123838Z DEBUG Read message with total size: 201
2023-12-23T21:01:24.123868Z DEBUG Read JSON string message: {"FragmentResult":{"id":{"offset":0,"count":16},"resolution":{"nx":1,"ny":1},"range":{"min":{"x":-1.2,"y":-1.2},"max":{"x":-1.198,"y":-1.198}},"pixels":{"offset":16,"count":1}}}
2023-12-23T21:01:24.123875Z DEBUG Read JSON struct message: FragmentResult(FragmentResult { id: U8Data { offset: 0, count: 16 }, resolution: Resolution { nx: 1, ny: 1 }, range: Range { min: Point { x: -1.2, y: -1.2 }, max: Point { x: -1.198, y: -1.198 } }, pixels: PixelData { offset: 16, count: 1 } })
2023-12-23T21:01:24.123880Z TRACE 24 data bytes read: [00, 00, 00, 00, 00, 00, 00, 00, F0, 15, 99, 71, 45, D6, C8, 8E, 40, 07, 35, F6, 00, 00, 00, 00]
```