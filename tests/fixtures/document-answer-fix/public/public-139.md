# `MD036` - N'utilisez pas une ligne seulement mise en emphase comme titre

Étiquettes: `emphasis`, `headings`

Alias: `no-emphasis-as-heading`

Paramètres:

- `punctuation`: valeur de configuration (`string`, défaut `.,;:!?。，；：！？`)

## Vue d'ensemble

N'utilisez pas une ligne seulement mise en emphase comme titre. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
**My document**

Lorem ipsum dolor sit amet...

_Another section_

Consectetur adipiscing elit, sed do eiusmod.
```

```markdown
# My document

Lorem ipsum dolor sit amet...

## Another section

Consectetur adipiscing elit, sed do eiusmod.
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
