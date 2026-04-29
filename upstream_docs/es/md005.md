# `MD005` - Alinea la sangría de los elementos del mismo nivel

Etiquetas: `bullet`, `indentation`, `ul`

Alias: `list-indent`

Parámetros:

Ninguno.

## Resumen

Alinea la sangría de los elementos del mismo nivel. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
   * A misaligned item
```

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
  * Nested Item 3
```

```markdown
...
8. Item
9. Item
10. Item
11. Item
...
```

```markdown
...
 8. Item
 9. Item
10. Item
11. Item
...
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
