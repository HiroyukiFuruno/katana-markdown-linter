# `MD054` - Link- und Bildstile müssen zur Konfiguration passen

Tags: `images`, `links`

Aliasse: `link-image-style`

Parameter:

- `autolink`: Konfigurationswert (`boolean`, Standard `true`)
- `collapsed`: Konfigurationswert (`boolean`, Standard `true`)
- `full`: Konfigurationswert (`boolean`, Standard `true`)
- `inline`: Konfigurationswert (`boolean`, Standard `true`)
- `shortcut`: Konfigurationswert (`boolean`, Standard `true`)
- `url_inline`: Konfigurationswert (`boolean`, Standard `true`)

## Überblick

Link- und Bildstile müssen zur Konfiguration passen. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

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

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
