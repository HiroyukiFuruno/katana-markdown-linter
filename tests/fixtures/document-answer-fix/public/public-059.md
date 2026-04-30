# `MD004` - Mantén un estilo coherente para las listas no ordenadas

Etiquetas: `bullet`, `ul`

Alias: `ul-style`

Parámetros:

- `style`: valor de configuración (`string`, predeterminado `consistent`)

## Resumen

Mantén un estilo coherente para las listas no ordenadas. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
* Item 1
+ Item 2
- Item 3
```

```markdown
* Item 1
* Item 2
* Item 3
```

```markdown
* Item 1
  + Item 2
    - Item 3
  + Item 4
* Item 4
  + Item 5
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
