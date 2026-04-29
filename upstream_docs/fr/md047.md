# `MD047` - Les fichiers doivent se terminer par un retour à la ligne

Étiquettes: `blank_lines`

Alias: `single-trailing-newline`

Paramètres:

Aucun.

## Vue d'ensemble

Les fichiers doivent se terminer par un retour à la ligne. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# Heading

This file ends without a newline.[EOF]
```

```markdown
# Heading

This file ends with a newline.
[EOF]
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
