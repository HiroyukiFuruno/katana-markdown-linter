# `MD022` - Los encabezados deben estar rodeados de líneas en blanco

Etiquetas: `blank_lines`, `headings`

Alias: `blanks-around-headings`

Parámetros:

- `lines_above`: valor de configuración (`integer|integer[]`, predeterminado `1`)
- `lines_below`: valor de configuración (`integer|integer[]`, predeterminado `1`)

## Resumen

Los encabezados deben estar rodeados de líneas en blanco. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# Heading 1
Some text

Some more text
## Heading 2
```

```markdown
# Heading 1

Some text

Some more text

## Heading 2
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
