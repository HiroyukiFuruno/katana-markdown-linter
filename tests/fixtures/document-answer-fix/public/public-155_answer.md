# `MD052` - Il manque une définition de lien ou d'image de référence

Étiquettes: `images`, `links`

Alias: `reference-links-images`

Paramètres:

- `ignored_labels`: valeur de configuration (`string[]`, défaut `["x"]`)
- `shortcut_syntax`: valeur de configuration (`boolean`, défaut `false`)

## Vue d'ensemble

Il manque une définition de lien ou d'image de référence. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
Full: [text][label]
Collapsed: [label][]
Shortcut: [label]

Full: ![text][image]
Collapsed: ![image][]
Shortcut: ![image]

[label]: https://example.com/label
[image]: https://example.com/image
```

```markdown
- [x] Checked task list item
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
