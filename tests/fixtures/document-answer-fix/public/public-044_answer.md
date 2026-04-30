# `MD047` - Dateien müssen mit einem Zeilenumbruch enden

Tags: `blank_lines`

Aliasse: `single-trailing-newline`

Parameter:

Keine.

## Überblick

Dateien müssen mit einem Zeilenumbruch enden. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# Heading

This file ends without a newline.[EOF]
```

```markdown
# Heading

This file ends with a newline.
[EOF]
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
