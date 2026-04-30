# `MD053` - Des définitions de lien ne sont pas utilisées

Étiquettes: `images`, `links`

Alias: `link-image-reference-definitions`

Paramètres:

- `ignored_definitions`: valeur de configuration (`string[]`, défaut `["//"]`)

## Vue d'ensemble

Des définitions de lien ne sont pas utilisées. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
[//]: # (This behaves like a comment)
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
