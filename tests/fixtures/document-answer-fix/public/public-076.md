# `MD026` - Elimina la puntuación final de los encabezados

Etiquetas: `headings`

Alias: `no-trailing-punctuation`

Parámetros:

- `punctuation`: valor de configuración (`string`, predeterminado `.,;:!。，；：！`)

## Resumen

Elimina la puntuación final de los encabezados. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# This is a heading.
```

```markdown
# This is a heading
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
