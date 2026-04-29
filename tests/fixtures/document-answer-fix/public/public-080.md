# `MD030` - Mantén el mismo número de espacios después de los marcadores de lista

Etiquetas: `ol`, `ul`, `whitespace`

Alias: `list-marker-space`

Parámetros:

- `ol_multi`: valor de configuración (`integer`, predeterminado `1`)
- `ol_single`: valor de configuración (`integer`, predeterminado `1`)
- `ul_multi`: valor de configuración (`integer`, predeterminado `1`)
- `ul_single`: valor de configuración (`integer`, predeterminado `1`)

## Resumen

Mantén el mismo número de espacios después de los marcadores de lista. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
* Foo
* Bar
* Baz

1. Foo
1. Bar
1. Baz

1. Foo
   * Bar
1. Baz
```

```markdown
* Foo
* Bar
* Baz
```

```markdown
*   Foo

    Second paragraph

*   Bar
```

```markdown
1.  Foo

    Second paragraph

1.  Bar
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
