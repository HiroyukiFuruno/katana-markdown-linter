# `MD042` - No uses enlaces vacíos

Etiquetas: `links`

Alias: `no-empty-links`

Parámetros:

Ninguno.

## Resumen

No uses enlaces vacíos. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
[an empty link]()
```

```markdown
[a valid link](https://example.com/)
```

```markdown
[an empty fragment](#)
```

```markdown
[a valid fragment](#fragment)
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
