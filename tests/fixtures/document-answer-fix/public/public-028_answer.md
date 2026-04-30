# `MD031` - Codeblöcke müssen von Leerzeilen umgeben sein

Tags: `blank_lines`, `code`

Aliasse: `blanks-around-fences`

Parameter:

- `list_items`: Konfigurationswert (`boolean`, Standard `true`)

## Überblick

Codeblöcke müssen von Leerzeilen umgeben sein. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

````markdown
Some text
```

```

```

```
Some more text
```

````markdown

Some text

```text

```

```text

```

Some more text

```text

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
