# `MD058` - Les tableaux doivent être entourés de lignes vides

Étiquettes: `table`

Alias: `blanks-around-tables`

Paramètres:

Aucun.

## Vue d'ensemble

Les tableaux doivent être entourés de lignes vides. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
Some text
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
> Blockquote
```

```markdown
Some text

| Header | Header |
| ------ | ------ |
| Cell   | Cell   |

> Blockquote
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
This text is part of the table and the next line is blank

Some text
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
