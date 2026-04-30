# `MD052` - Die Definition für Referenzlink oder Referenzbild fehlt

Tags: `images`, `links`

Aliasse: `reference-links-images`

Parameter:

- `ignored_labels`: Konfigurationswert (`string[]`, Standard `["x"]`)
- `shortcut_syntax`: Konfigurationswert (`boolean`, Standard `false`)

## Überblick

Die Definition für Referenzlink oder Referenzbild fehlt. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
Full: [text][label]
Collapsed: [label][]
Shortcut: [label]

Full: ![text][image]
Collapsed: ![image][]
Shortcut: ![image]

[label]: https://example.com/label
[image]: https://example.com/image
```

```markdown
- [x] Checked task list item
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
