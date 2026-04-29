# `MD010` - Verwende keine harten Tabulatoren

Tags: `hard_tab`, `whitespace`

Aliasse: `no-hard-tabs`

Parameter:

- `code_blocks`: Konfigurationswert (`boolean`, Standard `true`)
- `ignore_code_languages`: Konfigurationswert (`string[]`, Standard `[]`)
- `spaces_per_tab`: Konfigurationswert (`integer`, Standard `1`)

## Überblick

Verwende keine harten Tabulatoren. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
Some text

	* hard tab character used to indent the list item
```

```markdown
Some text

    * Spaces used to indent the list item instead
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
