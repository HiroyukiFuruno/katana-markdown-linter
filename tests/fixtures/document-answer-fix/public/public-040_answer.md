# `MD043` - Halte die erforderliche Überschriftenstruktur ein

Tags: `headings`

Aliasse: `required-headings`

Parameter:

- `headings`: Konfigurationswert (`string[]`, Standard `[]`)
- `match_case`: Konfigurationswert (`boolean`, Standard `false`)

## Überblick

Halte die erforderliche Überschriftenstruktur ein. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# Heading
## Item
### Detail
```

```json
[
    "# Heading",
    "## Item",
    "### Detail"
]
```

```markdown
# Heading
## Item
### Detail (optional)
## Foot
### Notes (optional)
```

```json
[
    "# Heading",
    "## Item",
    "*",
    "## Foot",
    "*"
]
```

```markdown
# Project Name
## Description
## Examples
```

```json
[
    "?",
    "## Description",
    "## Examples"
]
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
