# `MD024` - Es gibt doppelte Überschriften

Tags: `headings`

Aliasse: `no-duplicate-heading`

Parameter:

- `siblings_only`: Konfigurationswert (`boolean`, Standard `false`)

## Überblick

Es gibt doppelte Überschriften. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# Some text

## Some text
```

```markdown
# Some text

## Some more text
```

```markdown
# Change log

## 1.0.0

### Features

## 2.0.0

### Features
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
