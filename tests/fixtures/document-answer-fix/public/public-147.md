# `MD044` - Respectez la casse configurée des noms propres

Étiquettes: `spelling`

Alias: `proper-names`

Paramètres:

- `code_blocks`: valeur de configuration (`boolean`, défaut `true`)
- `html_elements`: valeur de configuration (`boolean`, défaut `true`)
- `names`: valeur de configuration (`string[]`, défaut `[]`)

## Vue d'ensemble

Respectez la casse configurée des noms propres. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```json
[
    "JavaScript"
]
```

```json
[
    "GitHub",
    "github.com"
]
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
