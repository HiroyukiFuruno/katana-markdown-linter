# `MD027` - Elimina espacios sobrantes después del marcador de cita

Etiquetas: `blockquote`, `indentation`, `whitespace`

Alias: `no-multiple-space-blockquote`

Parámetros:

- `list_items`: valor de configuración (`boolean`, predeterminado `true`)

## Resumen

Elimina espacios sobrantes después del marcador de cita. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
>  This is a blockquote with bad indentation
>  there should only be one.
```

```markdown
> This is a blockquote with correct
> indentation.
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
