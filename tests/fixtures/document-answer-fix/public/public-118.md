# `MD012` - Réduisez les lignes vides consécutives

Étiquettes: `blank_lines`, `whitespace`

Alias: `no-multiple-blanks`

Paramètres:

- `maximum`: valeur de configuration (`integer`, défaut `1`)

## Vue d'ensemble

Réduisez les lignes vides consécutives. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
Some text here


Some more text here
```

```markdown
Some text here

Some more text here
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
