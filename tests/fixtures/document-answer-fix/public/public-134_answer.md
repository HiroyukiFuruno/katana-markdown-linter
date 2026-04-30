# `MD031` - Les blocs de code doivent être entourés de lignes vides

Étiquettes: `blank_lines`, `code`

Alias: `blanks-around-fences`

Paramètres:

- `list_items`: valeur de configuration (`boolean`, défaut `true`)

## Vue d'ensemble

Les blocs de code doivent être entourés de lignes vides. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

````markdown
Some text
```

```

```

```
Some more text
```

````markdown

Some text

```text

```

```text

```

Some more text

```text

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
