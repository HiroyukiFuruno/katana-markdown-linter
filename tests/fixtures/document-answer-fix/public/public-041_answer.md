# `MD044` - Schreibe Eigennamen gemäß Konfiguration

Tags: `spelling`

Aliasse: `proper-names`

Parameter:

- `code_blocks`: Konfigurationswert (`boolean`, Standard `true`)
- `html_elements`: Konfigurationswert (`boolean`, Standard `true`)
- `names`: Konfigurationswert (`string[]`, Standard `[]`)

## Überblick

Schreibe Eigennamen gemäß Konfiguration. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```json
[
    "JavaScript"
]
```

```json
[
    "GitHub",
    "github.com"
]
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
