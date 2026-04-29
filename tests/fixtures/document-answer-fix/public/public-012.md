# `MD012` - Reduziere mehrere aufeinanderfolgende Leerzeilen

Tags: `blank_lines`, `whitespace`

Aliasse: `no-multiple-blanks`

Parameter:

- `maximum`: Konfigurationswert (`integer`, Standard `1`)

## Überblick

Reduziere mehrere aufeinanderfolgende Leerzeilen. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
Some text here


Some more text here
```

```markdown
Some text here

Some more text here
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
