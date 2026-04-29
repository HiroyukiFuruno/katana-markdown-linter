# `MD038` - Ne placez pas d'espaces dans les marqueurs de code

Étiquettes: `code`, `whitespace`

Alias: `no-space-in-code`

Paramètres:

Aucun.

## Vue d'ensemble

Ne placez pas d'espaces dans les marqueurs de code. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
`some text `

` some text`

`   some text   `
```

```markdown
`some text`
```

```markdown
`` `backticks` ``

`` backtick` ``
```

```markdown
` code `
```

```markdown
` `

`   `
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
