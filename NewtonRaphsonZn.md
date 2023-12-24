# [Newton fractals](https://en.wikipedia.org/wiki/Newton_fractal)

Dans ce cas, la fonction $f(z_n) = z_n - \frac{p(z_n)}{p'(z_n)}$ où $p(z)$ est une fonction polynomiale, $z_0$
correspond à un pixel exprimé sous la forme d'un nombre complexe dans l'espace physique défini par le paramètre
global `range`.

On décide ici d'arrêter les itérations dès que $|z_{n+1}-z_n|^2 < 1e-6$.

On utilise ici une construction différente de `PixelIntensity` où, comme précédemment le champ `zn` vaut $|z_n|$ (module
de la dernière valeur de $z_n$ mais où `count` vaut l'argument[^1] de $z_n$ normalisé entre 0 et 1)

| Nom du type       | Description du type |
|-------------------|---------------------|
| `NewtonRaphsonZ3` | (aucun champ)       |
| `NewtonRaphsonZ4` | (aucun champ)       |

| `NewtonRaphsonZ3`: $p(z)=z^3-1$     | `NewtonRaphsonZ4`: $p(z)=z^4-1$     |
|-------------------------------------|-------------------------------------|
| ![](images/NewtonRaphsonZ3.png) | ![](images/NewtonRaphsonZ4.png) |

[^1]: argument au sens des nombres complexité où $ z = |z| \cdot e^{\i \arg(z)}$