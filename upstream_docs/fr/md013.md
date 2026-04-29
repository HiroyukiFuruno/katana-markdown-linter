# `MD013` - La longueur de ligne dépasse la limite

Étiquettes: `line_length`

Alias: `line-length`

Paramètres:

- `code_block_line_length`: valeur de configuration (`integer`, défaut `80`)
- `code_blocks`: valeur de configuration (`boolean`, défaut `true`)
- `heading_line_length`: valeur de configuration (`integer`, défaut `80`)
- `headings`: valeur de configuration (`boolean`, défaut `true`)
- `line_length`: valeur de configuration (`integer`, défaut `80`)
- `stern`: valeur de configuration (`boolean`, défaut `false`)
- `strict`: valeur de configuration (`boolean`, défaut `false`)
- `tables`: valeur de configuration (`boolean`, défaut `true`)

## Vue d'ensemble

La longueur de ligne dépasse la limite. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
IF THIS LINE IS THE MAXIMUM LENGTH
This line is okay because there are-no-spaces-beyond-that-length
This line is a violation because there are spaces beyond that length
This-line-is-okay-because-there-are-no-spaces-anywhere-within
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
