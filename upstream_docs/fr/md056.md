# `MD056` - Le nombre de colonnes du tableau doit rester cohérent

Étiquettes: `table`

Alias: `table-column-count`

Paramètres:

Aucun.

## Vue d'ensemble

Le nombre de colonnes du tableau doit rester cohérent. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
| Cell   |
| Cell   | Cell   | Cell   |
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
| Cell   | Cell   |
| Cell   | Cell   |
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
