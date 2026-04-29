# `MD007` - L'indentation des listes non ordonnées doit suivre la configuration

Étiquettes: `bullet`, `indentation`, `ul`

Alias: `ul-indent`

Paramètres:

- `indent`: valeur de configuration (`integer`, défaut `2`)
- `start_indent`: valeur de configuration (`integer`, défaut `2`)
- `start_indented`: valeur de configuration (`boolean`, défaut `false`)

## Vue d'ensemble

L'indentation des listes non ordonnées doit suivre la configuration. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
* List item
   * Nested list item indented by 3 spaces
```

```markdown
* List item
  * Nested list item indented by 2 spaces
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
