# `MD026` - Entferne Satzzeichen am Ende von Überschriften

Tags: `headings`

Aliasse: `no-trailing-punctuation`

Parameter:

- `punctuation`: Konfigurationswert (`string`, Standard `.,;:!。，；：！`)

## Überblick

Entferne Satzzeichen am Ende von Überschriften. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# This is a heading.
```

```markdown
# This is a heading
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
