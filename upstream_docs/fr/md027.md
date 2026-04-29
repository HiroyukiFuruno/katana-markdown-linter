# `MD027` - Supprimez les espaces superflus après le marqueur de citation

Étiquettes: `blockquote`, `indentation`, `whitespace`

Alias: `no-multiple-space-blockquote`

Paramètres:

- `list_items`: valeur de configuration (`boolean`, défaut `true`)

## Vue d'ensemble

Supprimez les espaces superflus après le marqueur de citation. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
>  This is a blockquote with bad indentation
>  there should only be one.
```

```markdown
> This is a blockquote with correct
> indentation.
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
