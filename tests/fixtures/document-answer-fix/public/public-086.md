# `MD036` - No uses una línea solo enfatizada como encabezado

Etiquetas: `emphasis`, `headings`

Alias: `no-emphasis-as-heading`

Parámetros:

- `punctuation`: valor de configuración (`string`, predeterminado `.,;:!?。，；：！？`)

## Resumen

No uses una línea solo enfatizada como encabezado. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
**My document**

Lorem ipsum dolor sit amet...

_Another section_

Consectetur adipiscing elit, sed do eiusmod.
```

```markdown
# My document

Lorem ipsum dolor sit amet...

## Another section

Consectetur adipiscing elit, sed do eiusmod.
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
