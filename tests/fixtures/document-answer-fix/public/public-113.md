# `MD005` - Alignez l'indentation des éléments de même niveau

Étiquettes: `bullet`, `indentation`, `ul`

Alias: `list-indent`

Paramètres:

Aucun.

## Vue d'ensemble

Alignez l'indentation des éléments de même niveau. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
   * A misaligned item
```

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
  * Nested Item 3
```

```markdown
...
8. Item
9. Item
10. Item
11. Item
...
```

```markdown
...
 8. Item
 9. Item
10. Item
11. Item
...
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
