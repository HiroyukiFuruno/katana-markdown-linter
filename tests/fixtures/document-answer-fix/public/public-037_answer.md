# `MD040` - Eingezäunte Codeblöcke müssen eine Sprache angeben

Tags: `code`, `language`

Aliasse: `fenced-code-language`

Parameter:

- `allowed_languages`: Konfigurationswert (`string[]`, Standard `[]`)
- `language_only`: Konfigurationswert (`boolean`, Standard `false`)

## Überblick

Eingezäunte Codeblöcke müssen eine Sprache angeben. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

````markdown
```

```
```

````markdown

```text

```

```text

````markdown

```text

```

```text

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
