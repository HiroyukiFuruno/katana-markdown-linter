# `MD007` - Die Einrückung ungeordneter Listen muss zur Konfiguration passen

Tags: `bullet`, `indentation`, `ul`

Aliasse: `ul-indent`

Parameter:

- `indent`: Konfigurationswert (`integer`, Standard `2`)
- `start_indent`: Konfigurationswert (`integer`, Standard `2`)
- `start_indented`: Konfigurationswert (`boolean`, Standard `false`)

## Überblick

Die Einrückung ungeordneter Listen muss zur Konfiguration passen. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
* List item
   * Nested list item indented by 3 spaces
```

```markdown
* List item
  * Nested list item indented by 2 spaces
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
