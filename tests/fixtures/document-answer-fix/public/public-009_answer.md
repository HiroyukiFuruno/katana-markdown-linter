# `MD009` - Entferne überflüssige Leerzeichen am Zeilenende

Tags: `whitespace`

Aliasse: `no-trailing-spaces`

Parameter:

- `br_spaces`: Konfigurationswert (`integer`, Standard `2`)
- `code_blocks`: Konfigurationswert (`boolean`, Standard `false`)
- `list_item_empty_lines`: Konfigurationswert (`boolean`, Standard `false`)
- `strict`: Konfigurationswert (`boolean`, Standard `false`)

## Überblick

Entferne überflüssige Leerzeichen am Zeilenende. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
Text text text
text[2 spaces]
```

```markdown
- list item text
  [2 spaces]
  list item text
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
