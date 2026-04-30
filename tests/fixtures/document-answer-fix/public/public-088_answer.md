# `MD038` - No pongas espacios dentro de los marcadores de código

Etiquetas: `code`, `whitespace`

Alias: `no-space-in-code`

Parámetros:

Ninguno.

## Resumen

No pongas espacios dentro de los marcadores de código. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
`some text `

` some text`

`   some text   `
```

```markdown
`some text`
```

```markdown
`` `backticks` ``

`` backtick` ``
```

```markdown
` code `
```

```markdown
` `

`   `
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
