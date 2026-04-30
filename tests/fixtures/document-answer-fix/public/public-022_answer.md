# `MD025` - Ein Dokument darf nur eine oberste Überschrift enthalten

Tags: `headings`

Aliasse: `single-h1`, `single-title`

Parameter:

- `front_matter_title`: Konfigurationswert (`string`, Standard `^\s*title\s*[:=]`)
- `level`: Konfigurationswert (`integer`, Standard `1`)

## Überblick

Ein Dokument darf nur eine oberste Überschrift enthalten. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# Top level heading

# Another top-level heading
```

```markdown
# Title

## Heading

## Another heading
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
