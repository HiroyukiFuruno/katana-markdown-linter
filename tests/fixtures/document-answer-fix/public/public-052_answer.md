# `MD055` - Verwende einen einheitlichen Stil für die Tabellen-Trennzeile

Tags: `table`

Aliasse: `table-pipe-style`

Parameter:

- `style`: Konfigurationswert (`string`, Standard `consistent`)

## Überblick

Verwende einen einheitlichen Stil für die Tabellen-Trennzeile. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
| Header | Header |
| ------ | ------
  Cell   | Cell   |
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
This text is part of the table
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
