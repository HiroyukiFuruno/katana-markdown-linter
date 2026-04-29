# `MD003` - Verwende einen einheitlichen Überschriftenstil

Tags: `headings`

Aliasse: `heading-style`

Parameter:

- `style`: Konfigurationswert (`string`, Standard `consistent`)

## Überblick

Verwende einen einheitlichen Überschriftenstil. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# ATX style H1

## Closed ATX style H2 ##

Setext style H1
===============
```

```markdown
# ATX style H1

## ATX style H2
```

```markdown
Setext style H1
===============

Setext style H2
---------------

### ATX style H3
```

```markdown
A line of text followed by a horizontal rule becomes a heading
---
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
