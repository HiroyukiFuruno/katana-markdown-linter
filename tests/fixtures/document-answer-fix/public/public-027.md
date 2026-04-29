# `MD030` - Halte die Leerzeichen nach Listenmarkern einheitlich

Tags: `ol`, `ul`, `whitespace`

Aliasse: `list-marker-space`

Parameter:

- `ol_multi`: Konfigurationswert (`integer`, Standard `1`)
- `ol_single`: Konfigurationswert (`integer`, Standard `1`)
- `ul_multi`: Konfigurationswert (`integer`, Standard `1`)
- `ul_single`: Konfigurationswert (`integer`, Standard `1`)

## Überblick

Halte die Leerzeichen nach Listenmarkern einheitlich. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
* Foo
* Bar
* Baz

1. Foo
1. Bar
1. Baz

1. Foo
   * Bar
1. Baz
```

```markdown
* Foo
* Bar
* Baz
```

```markdown
*   Foo

    Second paragraph

*   Bar
```

```markdown
1.  Foo

    Second paragraph

1.  Bar
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
