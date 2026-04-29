# `MD027` - Entferne überflüssige Leerzeichen nach dem Zitatmarker

Tags: `blockquote`, `indentation`, `whitespace`

Aliasse: `no-multiple-space-blockquote`

Parameter:

- `list_items`: Konfigurationswert (`boolean`, Standard `true`)

## Überblick

Entferne überflüssige Leerzeichen nach dem Zitatmarker. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
>  This is a blockquote with bad indentation
>  there should only be one.
```

```markdown
> This is a blockquote with correct
> indentation.
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
