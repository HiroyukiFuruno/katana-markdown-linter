# `MD022` - Überschriften müssen von Leerzeilen umgeben sein

Tags: `blank_lines`, `headings`

Aliasse: `blanks-around-headings`

Parameter:

- `lines_above`: Konfigurationswert (`integer|integer[]`, Standard `1`)
- `lines_below`: Konfigurationswert (`integer|integer[]`, Standard `1`)

## Überblick

Überschriften müssen von Leerzeilen umgeben sein. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# Heading 1
Some text

Some more text
## Heading 2
```

```markdown
# Heading 1

Some text

Some more text

## Heading 2
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
