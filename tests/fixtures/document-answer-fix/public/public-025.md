# `MD028` - Im Zitatblock gibt es eine unnötige Leerzeile

Tags: `blockquote`, `whitespace`

Aliasse: `no-blanks-blockquote`

Parameter:

Keine.

## Überblick

Im Zitatblock gibt es eine unnötige Leerzeile. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
> This is a blockquote
> which is immediately followed by

> this blockquote. Unfortunately
> In some parsers, these are treated as the same blockquote.
```

```markdown
> This is a blockquote.

And Jimmy also said:

> This too is a blockquote.
```

```markdown
> This is a blockquote.
>
> This is the same blockquote.
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
