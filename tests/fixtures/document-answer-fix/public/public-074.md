# `MD024` - Hay encabezados duplicados

Etiquetas: `headings`

Alias: `no-duplicate-heading`

Parámetros:

- `siblings_only`: valor de configuración (`boolean`, predeterminado `false`)

## Resumen

Hay encabezados duplicados. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# Some text

## Some text
```

```markdown
# Some text

## Some more text
```

```markdown
# Change log

## 1.0.0

### Features

## 2.0.0

### Features
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
