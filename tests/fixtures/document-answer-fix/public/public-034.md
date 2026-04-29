# `MD037` - Setze keine Leerzeichen innerhalb von Hervorhebungsmarkern

Tags: `emphasis`, `whitespace`

Aliasse: `no-space-in-emphasis`

Parameter:

Keine.

## Überblick

Setze keine Leerzeichen innerhalb von Hervorhebungsmarkern. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
Here is some ** bold ** text.

Here is some * italic * text.

Here is some more __ bold __ text.

Here is some more _ italic _ text.
```

```markdown
Here is some **bold** text.

Here is some *italic* text.

Here is some more __bold__ text.

Here is some more _italic_ text.
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
