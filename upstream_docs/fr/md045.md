# `MD045` - Les images doivent avoir un texte alternatif

Étiquettes: `accessibility`, `images`

Alias: `no-alt-text`

Paramètres:

Aucun.

## Vue d'ensemble

Les images doivent avoir un texte alternatif. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
![Alternate text](image.jpg)
```

```markdown
![Alternate text][ref]

...

[ref]: image.jpg "Optional title"
```

```html
<img src="image.jpg" alt="Alternate text" />
```

```html
<img src="image.jpg" aria-hidden="true" />
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
