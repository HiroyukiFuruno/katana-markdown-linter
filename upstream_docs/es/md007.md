# `MD007` - La sangría de listas no ordenadas debe seguir la configuración

Etiquetas: `bullet`, `indentation`, `ul`

Alias: `ul-indent`

Parámetros:

- `indent`: valor de configuración (`integer`, predeterminado `2`)
- `start_indent`: valor de configuración (`integer`, predeterminado `2`)
- `start_indented`: valor de configuración (`boolean`, predeterminado `false`)

## Resumen

La sangría de listas no ordenadas debe seguir la configuración. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
* List item
   * Nested list item indented by 3 spaces
```

```markdown
* List item
  * Nested list item indented by 2 spaces
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
