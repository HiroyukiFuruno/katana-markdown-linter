# `MD051` - Le fragment de titre ciblé par le lien n'existe pas

Étiquettes: `links`

Alias: `link-fragments`

Paramètres:

- `ignore_case`: valeur de configuration (`boolean`, défaut `false`)
- `ignored_pattern`: valeur de configuration (`string`, défaut ``)

## Vue d'ensemble

Le fragment de titre ciblé par le lien n'existe pas. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
# Heading Name

[Link](#fragment)
```

```markdown
# Heading Name

[Link](#heading-name)
```

```markdown
# Heading Name

[Link](#Heading-Name)
```

```markdown
# Heading Name {#custom-name}

[Link](#custom-name)
```

```markdown
<a id="bookmark"></a>

[Link](#bookmark)
```

```markdown
[Link](#top)
```

```markdown
[Link](#L20)
```

```markdown
[Link](#L19C5-L21C11)
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
