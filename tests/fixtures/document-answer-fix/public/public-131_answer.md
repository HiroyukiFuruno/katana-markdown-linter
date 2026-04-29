# `MD028` - Une citation contient une ligne vide inutile

Étiquettes: `blockquote`, `whitespace`

Alias: `no-blanks-blockquote`

Paramètres:

Aucun.

## Vue d'ensemble

Une citation contient une ligne vide inutile. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
> This is a blockquote
> which is immediately followed by

> this blockquote. Unfortunately
> In some parsers, these are treated as the same blockquote.
```

```markdown
> This is a blockquote.

And Jimmy also said:

> This too is a blockquote.
```

```markdown
> This is a blockquote.
>
> This is the same blockquote.
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
