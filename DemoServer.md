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
./server --trace --width=1 --height=1 
```
(version `abf2823` du 24 décembre 2023)

```
2023-12-24T10:08:06.501590Z DEBUG Write JSON message with 16 data bytes: FragmentTask(FragmentTask { id: U8Data { offset: 0, count: 16 }, fractal: Julia(JuliaDescriptor { c: 0.285+0.013i, divergence_threshold_square: 4.0 }), max_iteration: 64, resolution: Resolution { nx: 1, ny: 1 }, range: Range { min: Point { x: -1.2, y: -1.2 }, max: Point { x: 1.2, y: 1.2 } } })
2023-12-24T10:08:06.501677Z DEBUG Write total message size: 255
2023-12-24T10:08:06.501695Z DEBUG Write JSON message size: 239
2023-12-24T10:08:06.501703Z DEBUG Write string message: {"FragmentTask":{"id":{"offset":0,"count":16},"fractal":{"Julia":{"c":{"re":0.285,"im":0.013},"divergence_threshold_square":4.0}},"max_iteration":64,"resolution":{"nx":1,"ny":1},"range":{"min":{"x":-1.2,"y":-1.2},"max":{"x":1.2,"y":1.2}}}}
2023-12-24T10:08:06.501711Z TRACE Write data: [00, 00, 00, 00, 00, 00, 00, 00, 6A, 87, 9C, FA, B3, 9B, 6F, D4]
2023-12-24T10:08:06.502713Z DEBUG Worker 1 got a job; executing.
2023-12-24T10:08:06.502726Z DEBUG New connection from [::1]:58654
2023-12-24T10:08:06.502800Z DEBUG Read message with total size: 195
2023-12-24T10:08:06.502870Z DEBUG Read JSON string message: {"FragmentResult":{"id":{"offset":0,"count":16},"resolution":{"nx":1,"ny":1},"range":{"min":{"x":-1.2,"y":-1.2},"max":{"x":1.2,"y":1.2}},"pixels":{"offset":16,"count":1}}}
2023-12-24T10:08:06.502906Z DEBUG Read JSON struct message: FragmentResult(FragmentResult { id: U8Data { offset: 0, count: 16 }, resolution: Resolution { nx: 1, ny: 1 }, range: Range { min: Point { x: -1.2, y: -1.2 }, max: Point { x: 1.2, y: 1.2 } }, pixels: PixelData { offset: 16, count: 1 } })
2023-12-24T10:08:06.502925Z TRACE 24 data bytes read: [00, 00, 00, 00, 00, 00, 00, 00, 6A, 87, 9C, FA, B3, 9B, 6F, D4, 3C, 9B, 7A, A4, 3F, 80, 00, 00]
2023-12-24T10:08:06.502940Z TRACE Decoded pixels: [PixelIntensity { zn: 0.018979378, count: 1.0 }]
```

Petite précision sur le `PixelIntensity`:

* Il est calculé pour z=0 (centre de l'unique pixel de cette image)
* Les itérés de Julia ne divergent pas à partir de ce point donc `count=1.0` (i.e. nombre d'itérations
  effectuées / `max_iterations` )
* Ce qui donne la réponse:

| `00, 00, 00, 00, 00, 00, 00, 00, 6A, 87, 9C, FA, B3, 9B, 6F, D4` | `3C, 9B, 7A, A4` | `3F, 80, 00, 00` |
|------------------------------------------------------------------|------------------|------------------|
| identifiant de la tâche (envoyée avec le `FragmentTask`)         | zn (f32)         | count (f32)      |

Vous pourrez vérifier que:

```rust
assert_eq!(1_f32.to_be_bytes(), [0x3F, 0x80, 0x00, 0x00]);
```