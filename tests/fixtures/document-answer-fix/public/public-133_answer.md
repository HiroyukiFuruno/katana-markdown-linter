# `MD030` - Gardez le même nombre d'espaces après les marqueurs de liste

Étiquettes: `ol`, `ul`, `whitespace`

Alias: `list-marker-space`

Paramètres:

- `ol_multi`: valeur de configuration (`integer`, défaut `1`)
- `ol_single`: valeur de configuration (`integer`, défaut `1`)
- `ul_multi`: valeur de configuration (`integer`, défaut `1`)
- `ul_single`: valeur de configuration (`integer`, défaut `1`)

## Vue d'ensemble

Gardez le même nombre d'espaces après les marqueurs de liste. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
* Foo
* Bar
* Baz

1. Foo
1. Bar
1. Baz

1. Foo
   * Bar
1. Baz
```

```markdown
* Foo
* Bar
* Baz
```

```markdown
*   Foo

    Second paragraph

*   Bar
```

```markdown
1.  Foo

    Second paragraph

1.  Bar
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
