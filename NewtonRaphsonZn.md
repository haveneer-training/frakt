# [Newton fractals](https://en.wikipedia.org/wiki/Newton_fractal)

Dans ce cas, la fonction $f(z_n) = z_n - \frac{p(z_n)}{p'(z_n)}$ où $p(z)$ est une fonction polynomiale, $z_0$
correspond à un pixel exprimé sous la forme d'un nombre complexe dans l'espace physique défini par le paramètre
global `range`.

On décide ici d'arrêter les itérations dès que $|z_{n+1}-z_n|^2 < \epsilon$ où $\epsilon$ vaut $10^{-6}$.

Pour chaque `PixelIntensity`:
* `zn` vaut l'argument[^1] de $z_n$ normalisé entre 0 et 1 (i.e. $\frac{1}{2} + \frac{arg(z)}{2\pi}$),
* `count` vaut le nombre d'itérations effectuées divisé par `max_iteration`.

  NB: le serveur fourni n'utilise pas exactement ce calcul pour `count` afin de pouvoir définir des nuances plus subtiles. 
  Son calcul est en fait:
  ```rust
  fn convergence_value(pzn: f32, threshold: f64, count: u32, nmax: u32) -> f32 {
    let accuracy = f32::log10(threshold as f32);
    if count < nmax {
      0.5 - 0.5 * f32::cos(0.1 * (count as f32 - (f32::log10(pzn) / accuracy)))
    } else {
      1.
    }
  }
  ```
  où `pzn` vaut $|p(z_n)|$ et `threshold` vaut $\epsilon$.

  Ceci dit, il ne vous est pas nécessaire d'utiliser ce calcul avancé pour que votre calcul soit valide. 

| Nom du type       | Description du type |
|-------------------|---------------------|
| `NewtonRaphsonZ3` | (aucun champ)       |
| `NewtonRaphsonZ4` | (aucun champ)       |

| `NewtonRaphsonZ3`: $p(z)=z^3-1$ | `NewtonRaphsonZ4`: $p(z)=z^4-1$ |
|---------------------------------|---------------------------------|
| ![](images/NewtonRaphsonZ3.png) | ![](images/NewtonRaphsonZ4.png) |

[^1]: argument au sens des nombres complexes où $ z = |z| \cdot e^{\i \arg(z)}$