# `MD011` - Corrigez la syntaxe de lien inversée

Étiquettes: `links`

Alias: `no-reversed-links`

Paramètres:

Aucun.

## Vue d'ensemble

Corrigez la syntaxe de lien inversée. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
(Incorrect link syntax)[https://www.example.com/]
```

```markdown
[Correct link syntax](https://www.example.com/)
```

```markdown
For (example)[^1]
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
