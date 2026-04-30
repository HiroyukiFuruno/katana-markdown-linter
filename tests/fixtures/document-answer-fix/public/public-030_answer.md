# `MD033` - Verwende kein Inline-HTML

Tags: `html`

Aliasse: `no-inline-html`

Parameter:

- `allowed_elements`: Konfigurationswert (`string[]`, Standard `[]`)
- `table_allowed_elements`: Konfigurationswert (`string[]`, Standard `[]`)

## Überblick

Verwende kein Inline-HTML. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
<h1>Inline HTML heading</h1>
```

```markdown
# Markdown heading
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
