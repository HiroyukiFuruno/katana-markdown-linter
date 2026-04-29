# `MD053` - Hay definiciones de enlace sin usar

Etiquetas: `images`, `links`

Alias: `link-image-reference-definitions`

Parámetros:

- `ignored_definitions`: valor de configuración (`string[]`, predeterminado `["//"]`)

## Resumen

Hay definiciones de enlace sin usar. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
[//]: # (This behaves like a comment)
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
