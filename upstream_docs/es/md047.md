# `MD047` - Los archivos deben terminar con un salto de línea

Etiquetas: `blank_lines`

Alias: `single-trailing-newline`

Parámetros:

Ninguno.

## Resumen

Los archivos deben terminar con un salto de línea. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# Heading

This file ends without a newline.[EOF]
```

```markdown
# Heading

This file ends with a newline.
[EOF]
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
