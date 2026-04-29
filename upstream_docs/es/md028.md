# `MD028` - Hay una línea en blanco innecesaria dentro de la cita

Etiquetas: `blockquote`, `whitespace`

Alias: `no-blanks-blockquote`

Parámetros:

Ninguno.

## Resumen

Hay una línea en blanco innecesaria dentro de la cita. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
> This is a blockquote
> which is immediately followed by

> this blockquote. Unfortunately
> In some parsers, these are treated as the same blockquote.
```

```markdown
> This is a blockquote.

And Jimmy also said:

> This too is a blockquote.
```

```markdown
> This is a blockquote.
>
> This is the same blockquote.
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
