# `MD009` - Elimina espacios sobrantes al final de la línea

Etiquetas: `whitespace`

Alias: `no-trailing-spaces`

Parámetros:

- `br_spaces`: valor de configuración (`integer`, predeterminado `2`)
- `code_blocks`: valor de configuración (`boolean`, predeterminado `false`)
- `list_item_empty_lines`: valor de configuración (`boolean`, predeterminado `false`)
- `strict`: valor de configuración (`boolean`, predeterminado `false`)

## Resumen

Elimina espacios sobrantes al final de la línea. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
Text text text
text[2 spaces]
```

```markdown
- list item text
  [2 spaces]
  list item text
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
