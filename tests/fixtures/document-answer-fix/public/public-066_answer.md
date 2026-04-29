# `MD013` - La longitud de línea supera el límite

Etiquetas: `line_length`

Alias: `line-length`

Parámetros:

- `code_block_line_length`: valor de configuración (`integer`, predeterminado `80`)
- `code_blocks`: valor de configuración (`boolean`, predeterminado `true`)
- `heading_line_length`: valor de configuración (`integer`, predeterminado `80`)
- `headings`: valor de configuración (`boolean`, predeterminado `true`)
- `line_length`: valor de configuración (`integer`, predeterminado `80`)
- `stern`: valor de configuración (`boolean`, predeterminado `false`)
- `strict`: valor de configuración (`boolean`, predeterminado `false`)
- `tables`: valor de configuración (`boolean`, predeterminado `true`)

## Resumen

La longitud de línea supera el límite. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
IF THIS LINE IS THE MAXIMUM LENGTH
This line is okay because there are-no-spaces-beyond-that-length
This line is a violation because there are spaces beyond that length
This-line-is-okay-because-there-are-no-spaces-anywhere-within
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
