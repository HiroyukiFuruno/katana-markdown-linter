# `MD025` - Un document ne doit contenir qu'un seul titre de niveau supérieur

Étiquettes: `headings`

Alias: `single-h1`, `single-title`

Paramètres:

- `front_matter_title`: valeur de configuration (`string`, défaut `^\s*title\s*[:=]`)
- `level`: valeur de configuration (`integer`, défaut `1`)

## Vue d'ensemble

Un document ne doit contenir qu'un seul titre de niveau supérieur. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# Top level heading

# Another top-level heading
```

```markdown
# Title

## Heading

## Another heading
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
