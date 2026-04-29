# `MD022` - Les titres doivent être entourés de lignes vides

Étiquettes: `blank_lines`, `headings`

Alias: `blanks-around-headings`

Paramètres:

- `lines_above`: valeur de configuration (`integer|integer[]`, défaut `1`)
- `lines_below`: valeur de configuration (`integer|integer[]`, défaut `1`)

## Vue d'ensemble

Les titres doivent être entourés de lignes vides. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# Heading 1
Some text

Some more text
## Heading 2
```

```markdown
# Heading 1

Some text

Some more text

## Heading 2
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
