# `MD025` - Un documento solo debe tener un encabezado de nivel superior

Etiquetas: `headings`

Alias: `single-h1`, `single-title`

Parámetros:

- `front_matter_title`: valor de configuración (`string`, predeterminado `^\s*title\s*[:=]`)
- `level`: valor de configuración (`integer`, predeterminado `1`)

## Resumen

Un documento solo debe tener un encabezado de nivel superior. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# Top level heading

# Another top-level heading
```

```markdown
# Title

## Heading

## Another heading
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
