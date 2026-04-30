# `MD033` - No uses HTML en línea

Etiquetas: `html`

Alias: `no-inline-html`

Parámetros:

- `allowed_elements`: valor de configuración (`string[]`, predeterminado `[]`)
- `table_allowed_elements`: valor de configuración (`string[]`, predeterminado `[]`)

## Resumen

No uses HTML en línea. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
<h1>Inline HTML heading</h1>
```

```markdown
# Markdown heading
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
