# `MD041` - La primera línea del archivo debe ser un encabezado de nivel superior

Etiquetas: `headings`

Alias: `first-line-h1`, `first-line-heading`

Parámetros:

- `allow_preamble`: valor de configuración (`boolean`, predeterminado `false`)
- `front_matter_title`: valor de configuración (`string`, predeterminado `^\s*title\s*[:=]`)
- `level`: valor de configuración (`integer`, predeterminado `1`)

## Resumen

La primera línea del archivo debe ser un encabezado de nivel superior. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
This is a document without a heading
```

```markdown
# Document Heading

This is a document with a top-level heading
```

```markdown
<h1 align="center"><img src="https://placekitten.com/300/150"/></h1>

This is a document with a top-level HTML heading
```

```markdown
This is a document with preamble text

# Document Heading
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
