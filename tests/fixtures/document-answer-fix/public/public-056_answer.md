# `MD060` - Richte die Leerzeichen in Tabellenzellen aus

Tags: `table`

Aliasse: `table-column-style`

Parameter:

- `aligned_delimiter`: Konfigurationswert (`boolean`, Standard `false`)
- `style`: Konfigurationswert (`string`, Standard `any`)

## Überblick

Richte die Leerzeichen in Tabellenzellen aus. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

```markdown
| Character | Meaning |
| --------- | ------- |
| Y         | Yes     |
| N         | No      |
```

```markdown
| Character | Meaning |
|-----------|---------|
|     Y     |     Yes |
|     N     |      No |
```

```markdown
| Character | Meaning |
| --- | --- |
| Y | Yes |
| N | No |
```

```markdown
|Character|Meaning|
|---|---|
|Y|Yes|
|N|No|
```

```markdown
| Character | Meaning |
| --------- | ------- |
| Y | Yes |
| N | No |
```

```markdown
|Character|Meaning|
|---------|-------|
|Y|Yes|
|N|No|
```

```markdown
Character | Meaning
--- | ---
Y | Yes
N | No
```

```markdown
| Response | Emoji |
| -------- | ----- |
| Yes      | ✅    |
| No       | ❎    |
```

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
