# `MD052` - Falta la definición del enlace o la imagen de referencia

Etiquetas: `images`, `links`

Alias: `reference-links-images`

Parámetros:

- `ignored_labels`: valor de configuración (`string[]`, predeterminado `["x"]`)
- `shortcut_syntax`: valor de configuración (`boolean`, predeterminado `false`)

## Resumen

Falta la definición del enlace o la imagen de referencia. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
Full: [text][label]
Collapsed: [label][]
Shortcut: [label]

Full: ![text][image]
Collapsed: ![image][]
Shortcut: ![image]

[label]: https://example.com/label
[image]: https://example.com/image
```

```markdown
- [x] Checked task list item
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
