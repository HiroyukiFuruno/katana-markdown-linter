# `MD005` - Richte Listeneinträge gleicher Ebene gleich ein

Tags: `bullet`, `indentation`, `ul`

Aliasse: `list-indent`

Parameter:

Keine.

## Überblick

Richte Listeneinträge gleicher Ebene gleich ein. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
   * A misaligned item
```

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
  * Nested Item 3
```

```markdown
...
8. Item
9. Item
10. Item
11. Item
...
```

```markdown
...
 8. Item
 9. Item
10. Item
11. Item
...
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
