# `MD011` - Corrige la sintaxis de enlace invertida

Etiquetas: `links`

Alias: `no-reversed-links`

Parámetros:

Ninguno.

## Resumen

Corrige la sintaxis de enlace invertida. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
(Incorrect link syntax)[https://www.example.com/]
```

```markdown
[Correct link syntax](https://www.example.com/)
```

```markdown
For (example)[^1]
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
