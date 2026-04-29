# `MD004` - Verwende einen einheitlichen Stil für ungeordnete Listen

Tags: `bullet`, `ul`

Aliasse: `ul-style`

Parameter:

- `style`: Konfigurationswert (`string`, Standard `consistent`)

## Überblick

Verwende einen einheitlichen Stil für ungeordnete Listen. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
* Item 1
+ Item 2
- Item 3
```

```markdown
* Item 1
* Item 2
* Item 3
```

```markdown
* Item 1
  + Item 2
    - Item 3
  + Item 4
* Item 4
  + Item 5
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
