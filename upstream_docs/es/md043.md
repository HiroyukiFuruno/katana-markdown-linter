# `MD043` - Sigue la estructura de encabezados requerida

Etiquetas: `headings`

Alias: `required-headings`

Parámetros:

- `headings`: valor de configuración (`string[]`, predeterminado `[]`)
- `match_case`: valor de configuración (`boolean`, predeterminado `false`)

## Resumen

Sigue la estructura de encabezados requerida. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# Heading
## Item
### Detail
```

```json
[
    "# Heading",
    "## Item",
    "### Detail"
]
```

```markdown
# Heading
## Item
### Detail (optional)
## Foot
### Notes (optional)
```

```json
[
    "# Heading",
    "## Item",
    "*",
    "## Foot",
    "*"
]
```

```markdown
# Project Name
## Description
## Examples
```

```json
[
    "?",
    "## Description",
    "## Examples"
]
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
