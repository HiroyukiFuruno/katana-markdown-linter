# `MD045` - Bilder müssen Alternativtext enthalten

Tags: `accessibility`, `images`

Aliasse: `no-alt-text`

Parameter:

Keine.

## Überblick

Bilder müssen Alternativtext enthalten. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
![Alternate text](image.jpg)
```

```markdown
![Alternate text][ref]

...

[ref]: image.jpg "Optional title"
```

```html
<img src="image.jpg" alt="Alternate text" />
```

```html
<img src="image.jpg" aria-hidden="true" />
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
