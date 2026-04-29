# `MD001` - Los niveles de encabezado solo deben aumentar de uno en uno

Etiquetas: `headings`

Alias: `heading-increment`

Parámetros:

- `front_matter_title`: valor de configuración (`string`, predeterminado `^\s*title\s*[:=]`)

## Resumen

Los niveles de encabezado solo deben aumentar de uno en uno. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# Heading 1

### Heading 3

We skipped out a 2nd level heading in this document
```

```markdown
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

## Another Heading 2

### Another Heading 3
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
