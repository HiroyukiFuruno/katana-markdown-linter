# `MD029` - Nummeriere geordnete Listen korrekt

Tags: `ol`

Aliasse: `ol-prefix`

Parameter:

- `style`: Konfigurationswert (`string`, Standard `one_or_ordered`)

## Überblick

Nummeriere geordnete Listen korrekt. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
1. Do this.
1. Do that.
1. Done.
```

```markdown
1. Do this.
2. Do that.
3. Done.
```

```markdown
0. Do this.
1. Do that.
2. Done.
```

```markdown
0. Do this.
0. Do that.
0. Done.
```

```markdown
1. Do this.
3. Done.
```

```markdown
...
08. Item
09. Item
10. Item
11. Item
...
```

```text
Code block
```

```text
   Code block
   ```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
