# `MD003` - Conservez un style de titre cohérent

Étiquettes: `headings`

Alias: `heading-style`

Paramètres:

- `style`: valeur de configuration (`string`, défaut `consistent`)

## Vue d'ensemble

Conservez un style de titre cohérent. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# ATX style H1

## Closed ATX style H2 ##

Setext style H1
===============
```

```markdown
# ATX style H1

## ATX style H2
```

```markdown
Setext style H1
===============

Setext style H2
---------------

### ATX style H3
```

```markdown
A line of text followed by a horizontal rule becomes a heading
---
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
