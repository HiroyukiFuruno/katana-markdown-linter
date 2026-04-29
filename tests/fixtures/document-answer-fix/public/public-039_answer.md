# `MD042` - Verwende keine leeren Links

Tags: `links`

Aliasse: `no-empty-links`

Parameter:

Keine.

## Überblick

Verwende keine leeren Links. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
[an empty link]()
```

```markdown
[a valid link](https://example.com/)
```

```markdown
[an empty fragment](#)
```

```markdown
[a valid fragment](#fragment)
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
