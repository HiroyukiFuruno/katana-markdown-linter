# `MD041` - La première ligne du fichier doit être un titre de niveau supérieur

Étiquettes: `headings`

Alias: `first-line-h1`, `first-line-heading`

Paramètres:

- `allow_preamble`: valeur de configuration (`boolean`, défaut `false`)
- `front_matter_title`: valeur de configuration (`string`, défaut `^\s*title\s*[:=]`)
- `level`: valeur de configuration (`integer`, défaut `1`)

## Vue d'ensemble

La première ligne du fichier doit être un titre de niveau supérieur. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
This is a document without a heading
```

```markdown
# Document Heading

This is a document with a top-level heading
```

```markdown
<h1 align="center"><img src="https://placekitten.com/300/150"/></h1>

This is a document with a top-level HTML heading
```

```markdown
This is a document with preamble text

# Document Heading
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
