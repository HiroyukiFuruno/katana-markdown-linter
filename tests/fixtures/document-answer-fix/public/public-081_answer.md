# `MD031` - Los bloques de código deben estar rodeados de líneas en blanco

Etiquetas: `blank_lines`, `code`

Alias: `blanks-around-fences`

Parámetros:

- `list_items`: valor de configuración (`boolean`, predeterminado `true`)

## Resumen

Los bloques de código deben estar rodeados de líneas en blanco. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

````markdown
Some text
```

```

```

```
Some more text
```

````markdown

Some text

```text

```

```text

```

Some more text

```text

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
