# `MD024` - Des titres sont dupliqués

Étiquettes: `headings`

Alias: `no-duplicate-heading`

Paramètres:

- `siblings_only`: valeur de configuration (`boolean`, défaut `false`)

## Vue d'ensemble

Des titres sont dupliqués. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# Some text

## Some text
```

```markdown
# Some text

## Some more text
```

```markdown
# Change log

## 1.0.0

### Features

## 2.0.0

### Features
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
