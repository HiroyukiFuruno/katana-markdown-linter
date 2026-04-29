# `MD001` - Les niveaux de titre ne doivent augmenter que d'un niveau à la fois

Étiquettes: `headings`

Alias: `heading-increment`

Paramètres:

- `front_matter_title`: valeur de configuration (`string`, défaut `^\s*title\s*[:=]`)

## Vue d'ensemble

Les niveaux de titre ne doivent augmenter que d'un niveau à la fois. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# Heading 1

### Heading 3

We skipped out a 2nd level heading in this document
```

```markdown
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

## Another Heading 2

### Another Heading 3
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
