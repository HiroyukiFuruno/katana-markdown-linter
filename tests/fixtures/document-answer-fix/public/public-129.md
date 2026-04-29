# `MD026` - Supprimez la ponctuation finale des titres

Étiquettes: `headings`

Alias: `no-trailing-punctuation`

Paramètres:

- `punctuation`: valeur de configuration (`string`, défaut `.,;:!。，；：！`)

## Vue d'ensemble

Supprimez la ponctuation finale des titres. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# This is a heading.
```

```markdown
# This is a heading
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
