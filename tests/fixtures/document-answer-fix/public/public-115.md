# `MD009` - Supprimez les espaces superflus en fin de ligne

Étiquettes: `whitespace`

Alias: `no-trailing-spaces`

Paramètres:

- `br_spaces`: valeur de configuration (`integer`, défaut `2`)
- `code_blocks`: valeur de configuration (`boolean`, défaut `false`)
- `list_item_empty_lines`: valeur de configuration (`boolean`, défaut `false`)
- `strict`: valeur de configuration (`boolean`, défaut `false`)

## Vue d'ensemble

Supprimez les espaces superflus en fin de ligne. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
Text text text
text[2 spaces]
```

```markdown
- list item text
  [2 spaces]
  list item text
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
