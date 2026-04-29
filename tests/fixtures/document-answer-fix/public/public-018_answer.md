# `MD021` - Geschlossene ATX-Überschriften dürfen nur ein inneres Leerzeichen haben

Tags: `atx_closed`, `headings`, `spaces`

Aliasse: `no-multiple-space-closed-atx`

Parameter:

Keine.

## Überblick

Geschlossene ATX-Überschriften dürfen nur ein inneres Leerzeichen haben. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
#  Heading 1  #

##  Heading 2  ##
```

```markdown
# Heading 1 #

## Heading 2 ##
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
