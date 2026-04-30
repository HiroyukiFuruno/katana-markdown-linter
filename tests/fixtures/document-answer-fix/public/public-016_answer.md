# `MD019` - ATX-Überschriften dürfen nur ein Leerzeichen nach dem # haben

Tags: `atx`, `headings`, `spaces`

Aliasse: `no-multiple-space-atx`

Parameter:

Keine.

## Überblick

ATX-Überschriften dürfen nur ein Leerzeichen nach dem # haben. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
#  Heading 1

##  Heading 2
```

```markdown
# Heading 1

## Heading 2
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
