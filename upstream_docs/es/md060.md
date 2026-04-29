# `MD060` - Alinea los espacios en las celdas de la tabla

Etiquetas: `table`

Alias: `table-column-style`

Parámetros:

- `aligned_delimiter`: valor de configuración (`boolean`, predeterminado `false`)
- `style`: valor de configuración (`string`, predeterminado `any`)

## Resumen

Alinea los espacios en las celdas de la tabla. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

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

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
