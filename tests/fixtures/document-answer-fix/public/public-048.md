# `MD051` - Das verlinkte Überschriftenfragment existiert nicht

Tags: `links`

Aliasse: `link-fragments`

Parameter:

- `ignore_case`: Konfigurationswert (`boolean`, Standard `false`)
- `ignored_pattern`: Konfigurationswert (`string`, Standard ``)

## Überblick

Das verlinkte Überschriftenfragment existiert nicht. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
# Heading Name

[Link](#fragment)
```

```markdown
# Heading Name

[Link](#heading-name)
```

```markdown
# Heading Name

[Link](#Heading-Name)
```

```markdown
# Heading Name {#custom-name}

[Link](#custom-name)
```

```markdown
<a id="bookmark"></a>

[Link](#bookmark)
```

```markdown
[Link](#top)
```

```markdown
[Link](#L20)
```

```markdown
[Link](#L19C5-L21C11)
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
