# `MD054` - Les styles de liens et d'images doivent suivre la configuration

Étiquettes: `images`, `links`

Alias: `link-image-style`

Paramètres:

- `autolink`: valeur de configuration (`boolean`, défaut `true`)
- `collapsed`: valeur de configuration (`boolean`, défaut `true`)
- `full`: valeur de configuration (`boolean`, défaut `true`)
- `inline`: valeur de configuration (`boolean`, défaut `true`)
- `shortcut`: valeur de configuration (`boolean`, défaut `true`)
- `url_inline`: valeur de configuration (`boolean`, défaut `true`)

## Vue d'ensemble

Les styles de liens et d'images doivent suivre la configuration. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
<https://example.com>
```

```markdown
[link](https://example.com)

![image](https://example.com)
```

```markdown
[link][url]

![image][url]

[url]: https://example.com
```

```markdown
[url][]

![url][]

[url]: https://example.com
```

```markdown
[url]

![url]

[url]: https://example.com
```

```markdown
[https://example.com](https://example.com)
```

```markdown
<https://example.com>
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
