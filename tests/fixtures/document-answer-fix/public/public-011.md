# `MD011` - Korrigiere umgedrehte Link-Syntax

Tags: `links`

Aliasse: `no-reversed-links`

Parameter:

Keine.

## Überblick

Korrigiere umgedrehte Link-Syntax. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
(Incorrect link syntax)[https://www.example.com/]
```

```markdown
[Correct link syntax](https://www.example.com/)
```

```markdown
For (example)[^1]
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
