# `MD053` - Es gibt unbenutzte Linkdefinitionen

Tags: `images`, `links`

Aliasse: `link-image-reference-definitions`

Parameter:

- `ignored_definitions`: Konfigurationswert (`string[]`, Standard `["//"]`)

## Überblick

Es gibt unbenutzte Linkdefinitionen. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
[//]: # (This behaves like a comment)
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
