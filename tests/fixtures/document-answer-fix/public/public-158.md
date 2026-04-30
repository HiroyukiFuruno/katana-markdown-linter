# `MD055` - Conservez un style cohérent pour la ligne séparatrice du tableau

Étiquettes: `table`

Alias: `table-pipe-style`

Paramètres:

- `style`: valeur de configuration (`string`, défaut `consistent`)

## Vue d'ensemble

Conservez un style cohérent pour la ligne séparatrice du tableau. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
| Header | Header |
| ------ | ------
  Cell   | Cell   |
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
This text is part of the table
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
