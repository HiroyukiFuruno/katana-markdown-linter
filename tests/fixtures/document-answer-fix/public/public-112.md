# `MD004` - Conservez un style cohérent pour les listes non ordonnées

Étiquettes: `bullet`, `ul`

Alias: `ul-style`

Paramètres:

- `style`: valeur de configuration (`string`, défaut `consistent`)

## Vue d'ensemble

Conservez un style cohérent pour les listes non ordonnées. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
* Item 1
+ Item 2
- Item 3
```

```markdown
* Item 1
* Item 2
* Item 3
```

```markdown
* Item 1
  + Item 2
    - Item 3
  + Item 4
* Item 4
  + Item 5
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
