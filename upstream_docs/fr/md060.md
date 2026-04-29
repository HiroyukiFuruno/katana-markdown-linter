# `MD060` - Alignez les espaces dans les cellules du tableau

Étiquettes: `table`

Alias: `table-column-style`

Paramètres:

- `aligned_delimiter`: valeur de configuration (`boolean`, défaut `false`)
- `style`: valeur de configuration (`string`, défaut `any`)

## Vue d'ensemble

Alignez les espaces dans les cellules du tableau. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

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

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
