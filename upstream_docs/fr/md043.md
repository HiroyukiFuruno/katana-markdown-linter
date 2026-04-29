# `MD043` - Respectez la structure de titres requise

Étiquettes: `headings`

Alias: `required-headings`

Paramètres:

- `headings`: valeur de configuration (`string[]`, défaut `[]`)
- `match_case`: valeur de configuration (`boolean`, défaut `false`)

## Vue d'ensemble

Respectez la structure de titres requise. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# Heading
## Item
### Detail
```

```json
[
    "# Heading",
    "## Item",
    "### Detail"
]
```

```markdown
# Heading
## Item
### Detail (optional)
## Foot
### Notes (optional)
```

```json
[
    "# Heading",
    "## Item",
    "*",
    "## Foot",
    "*"
]
```

```markdown
# Project Name
## Description
## Examples
```

```json
[
    "?",
    "## Description",
    "## Examples"
]
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
