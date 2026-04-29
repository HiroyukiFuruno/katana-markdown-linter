# `MD012` - Reduce las líneas en blanco consecutivas

Etiquetas: `blank_lines`, `whitespace`

Alias: `no-multiple-blanks`

Parámetros:

- `maximum`: valor de configuración (`integer`, predeterminado `1`)

## Resumen

Reduce las líneas en blanco consecutivas. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
Some text here


Some more text here
```

```markdown
Some text here

Some more text here
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
