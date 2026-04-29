# `MD010` - N'utilisez pas de tabulations dures

Étiquettes: `hard_tab`, `whitespace`

Alias: `no-hard-tabs`

Paramètres:

- `code_blocks`: valeur de configuration (`boolean`, défaut `true`)
- `ignore_code_languages`: valeur de configuration (`string[]`, défaut `[]`)
- `spaces_per_tab`: valeur de configuration (`integer`, défaut `1`)

## Vue d'ensemble

N'utilisez pas de tabulations dures. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
Some text

	* hard tab character used to indent the list item
```

```markdown
Some text

    * Spaces used to indent the list item instead
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
