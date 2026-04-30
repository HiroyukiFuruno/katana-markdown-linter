# `MD042` - N'utilisez pas de liens vides

Étiquettes: `links`

Alias: `no-empty-links`

Paramètres:

Aucun.

## Vue d'ensemble

N'utilisez pas de liens vides. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
[an empty link]()
```

```markdown
[a valid link](https://example.com/)
```

```markdown
[an empty fragment](#)
```

```markdown
[a valid fragment](#fragment)
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
