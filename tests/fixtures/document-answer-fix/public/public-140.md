# `MD037` - Ne placez pas d'espaces dans les marqueurs d'emphase

Étiquettes: `emphasis`, `whitespace`

Alias: `no-space-in-emphasis`

Paramètres:

Aucun.

## Vue d'ensemble

Ne placez pas d'espaces dans les marqueurs d'emphase. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
Here is some ** bold ** text.

Here is some * italic * text.

Here is some more __ bold __ text.

Here is some more _ italic _ text.
```

```markdown
Here is some **bold** text.

Here is some *italic* text.

Here is some more __bold__ text.

Here is some more _italic_ text.
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
