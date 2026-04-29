# `MD003` - Mantén un estilo de encabezado coherente

Etiquetas: `headings`

Alias: `heading-style`

Parámetros:

- `style`: valor de configuración (`string`, predeterminado `consistent`)

## Resumen

Mantén un estilo de encabezado coherente. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# ATX style H1

## Closed ATX style H2 ##

Setext style H1
===============
```

```markdown
# ATX style H1

## ATX style H2
```

```markdown
Setext style H1
===============

Setext style H2
---------------

### ATX style H3
```

```markdown
A line of text followed by a horizontal rule becomes a heading
---
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
