# `MD041` - Die erste Zeile der Datei muss eine oberste Überschrift sein

Tags: `headings`

Aliasse: `first-line-h1`, `first-line-heading`

Parameter:

- `allow_preamble`: Konfigurationswert (`boolean`, Standard `false`)
- `front_matter_title`: Konfigurationswert (`string`, Standard `^\s*title\s*[:=]`)
- `level`: Konfigurationswert (`integer`, Standard `1`)

## Überblick

Die erste Zeile der Datei muss eine oberste Überschrift sein. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
This is a document without a heading
```

```markdown
# Document Heading

This is a document with a top-level heading
```

```markdown
<h1 align="center"><img src="https://placekitten.com/300/150"/></h1>

This is a document with a top-level HTML heading
```

```markdown
This is a document with preamble text

# Document Heading
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
