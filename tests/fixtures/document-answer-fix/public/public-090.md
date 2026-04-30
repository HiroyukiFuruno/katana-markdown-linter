# `MD040` - Los bloques de código con valla deben indicar el lenguaje

Etiquetas: `code`, `language`

Alias: `fenced-code-language`

Parámetros:

- `allowed_languages`: valor de configuración (`string[]`, predeterminado `[]`)
- `language_only`: valor de configuración (`boolean`, predeterminado `false`)

## Resumen

Los bloques de código con valla deben indicar el lenguaje. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

````markdown
```

```
```

````markdown
```

```
```

````markdown
```

```
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
