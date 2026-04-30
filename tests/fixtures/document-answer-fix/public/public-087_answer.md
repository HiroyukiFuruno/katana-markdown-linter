# `MD037` - No pongas espacios dentro de los marcadores de énfasis

Etiquetas: `emphasis`, `whitespace`

Alias: `no-space-in-emphasis`

Parámetros:

Ninguno.

## Resumen

No pongas espacios dentro de los marcadores de énfasis. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
Here is some ** bold ** text.

Here is some * italic * text.

Here is some more __ bold __ text.

Here is some more _ italic _ text.
```

```markdown
Here is some **bold** text.

Here is some *italic* text.

Here is some more __bold__ text.

Here is some more _italic_ text.
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
