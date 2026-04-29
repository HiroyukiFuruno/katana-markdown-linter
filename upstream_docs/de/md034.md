# `MD034` - Bloße URLs müssen in spitzen Klammern oder als Link geschrieben werden

Tags: `links`, `url`

Aliasse: `no-bare-urls`

Parameter:

Keine.

## Überblick

Bloße URLs müssen in spitzen Klammern oder als Link geschrieben werden. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
For more info, visit https://www.example.com/ or email user@example.com.
```

```markdown
For more info, visit <https://www.example.com/> or email <user@example.com>.
```

```markdown
Not a clickable link: `https://www.example.com`
```

```markdown
[https://www.example.com]
```

```markdown
[text [shortcut] text](https://example.com)
```

```markdown
[link \[text\] link](https://example.com)
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
