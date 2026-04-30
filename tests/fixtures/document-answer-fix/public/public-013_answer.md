# `MD013` - Die Zeilenlänge überschreitet das Limit

Tags: `line_length`

Aliasse: `line-length`

Parameter:

- `code_block_line_length`: Konfigurationswert (`integer`, Standard `80`)
- `code_blocks`: Konfigurationswert (`boolean`, Standard `true`)
- `heading_line_length`: Konfigurationswert (`integer`, Standard `80`)
- `headings`: Konfigurationswert (`boolean`, Standard `true`)
- `line_length`: Konfigurationswert (`integer`, Standard `80`)
- `stern`: Konfigurationswert (`boolean`, Standard `false`)
- `strict`: Konfigurationswert (`boolean`, Standard `false`)
- `tables`: Konfigurationswert (`boolean`, Standard `true`)

## Überblick

Die Zeilenlänge überschreitet das Limit. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
IF THIS LINE IS THE MAXIMUM LENGTH
This line is okay because there are-no-spaces-beyond-that-length
This line is a violation because there are spaces beyond that length
This-line-is-okay-because-there-are-no-spaces-anywhere-within
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
