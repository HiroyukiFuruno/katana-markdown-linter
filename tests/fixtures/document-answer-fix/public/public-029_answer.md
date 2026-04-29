# `MD032` - Listen müssen von Leerzeilen umgeben sein

Tags: `blank_lines`, `bullet`, `ol`, `ul`

Aliasse: `blanks-around-lists`

Parameter:

Keine.

## Überblick

Listen müssen von Leerzeilen umgeben sein. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
Some text
* List item
* List item

1. List item
2. List item
***
```

```markdown
Some text

* List item
* List item

1. List item
2. List item

***
```

```markdown
1. List item
   More item 1
2. List item
More item 2
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
