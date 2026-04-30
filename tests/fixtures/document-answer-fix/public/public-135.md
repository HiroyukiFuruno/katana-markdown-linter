# `MD032` - Les listes doivent être entourées de lignes vides

Étiquettes: `blank_lines`, `bullet`, `ol`, `ul`

Alias: `blanks-around-lists`

Paramètres:

Aucun.

## Vue d'ensemble

Les listes doivent être entourées de lignes vides. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
Some text
* List item
* List item

1. List item
2. List item
***
```

```markdown
Some text

* List item
* List item

1. List item
2. List item

***
```

```markdown
1. List item
   More item 1
2. List item
More item 2
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
