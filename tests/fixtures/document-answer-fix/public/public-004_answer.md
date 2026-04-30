# `MD001` - Überschriftenebenen dürfen nur um eine Ebene auf einmal steigen

Tags: `headings`

Aliasse: `heading-increment`

Parameter:

- `front_matter_title`: Konfigurationswert (`string`, Standard `^\s*title\s*[:=]`)

## Überblick

Überschriftenebenen dürfen nur um eine Ebene auf einmal steigen. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# Heading 1

### Heading 3

We skipped out a 2nd level heading in this document
```

```markdown
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

## Another Heading 2

### Another Heading 3
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
