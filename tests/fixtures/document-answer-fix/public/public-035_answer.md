# `MD038` - Setze keine Leerzeichen innerhalb von Code-Markern

Tags: `code`, `whitespace`

Aliasse: `no-space-in-code`

Parameter:

Keine.

## Überblick

Setze keine Leerzeichen innerhalb von Code-Markern. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
`some text `

` some text`

`   some text   `
```

```markdown
`some text`
```

```markdown
`` `backticks` ``

`` backtick` ``
```

```markdown
` code `
```

```markdown
` `

`   `
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
