# `MD055` - Mantén un estilo coherente para la línea separadora de la tabla

Etiquetas: `table`

Alias: `table-pipe-style`

Parámetros:

- `style`: valor de configuración (`string`, predeterminado `consistent`)

## Resumen

Mantén un estilo coherente para la línea separadora de la tabla. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
| Header | Header |
| ------ | ------
  Cell   | Cell   |
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
```

```markdown
| Header | Header |
| ------ | ------ |
| Cell   | Cell   |
This text is part of the table
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
