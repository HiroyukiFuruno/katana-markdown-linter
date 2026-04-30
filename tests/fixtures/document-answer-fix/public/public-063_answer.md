# `MD010` - No uses tabulaciones duras

Etiquetas: `hard_tab`, `whitespace`

Alias: `no-hard-tabs`

Parámetros:

- `code_blocks`: valor de configuración (`boolean`, predeterminado `true`)
- `ignore_code_languages`: valor de configuración (`string[]`, predeterminado `[]`)
- `spaces_per_tab`: valor de configuración (`integer`, predeterminado `1`)

## Resumen

No uses tabulaciones duras. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
Some text

 * hard tab character used to indent the list item
```

```markdown
Some text

    * Spaces used to indent the list item instead
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
