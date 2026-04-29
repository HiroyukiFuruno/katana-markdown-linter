# `MD036` - Verwende keine reine Hervorhebungszeile als Überschrift

Tags: `emphasis`, `headings`

Aliasse: `no-emphasis-as-heading`

Parameter:

- `punctuation`: Konfigurationswert (`string`, Standard `.,;:!?。，；：！？`)

## Überblick

Verwende keine reine Hervorhebungszeile als Überschrift. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
**My document**

Lorem ipsum dolor sit amet...

_Another section_

Consectetur adipiscing elit, sed do eiusmod.
```

```markdown
# My document

Lorem ipsum dolor sit amet...

## Another section

Consectetur adipiscing elit, sed do eiusmod.
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
