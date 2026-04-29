# `MD054` - Los estilos de enlaces e imágenes deben seguir la configuración

Etiquetas: `images`, `links`

Alias: `link-image-style`

Parámetros:

- `autolink`: valor de configuración (`boolean`, predeterminado `true`)
- `collapsed`: valor de configuración (`boolean`, predeterminado `true`)
- `full`: valor de configuración (`boolean`, predeterminado `true`)
- `inline`: valor de configuración (`boolean`, predeterminado `true`)
- `shortcut`: valor de configuración (`boolean`, predeterminado `true`)
- `url_inline`: valor de configuración (`boolean`, predeterminado `true`)

## Resumen

Los estilos de enlaces e imágenes deben seguir la configuración. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
<https://example.com>
```

```markdown
[link](https://example.com)

![image](https://example.com)
```

```markdown
[link][url]

![image][url]

[url]: https://example.com
```

```markdown
[url][]

![url][]

[url]: https://example.com
```

```markdown
[url]

![url]

[url]: https://example.com
```

```markdown
[https://example.com](https://example.com)
```

```markdown
<https://example.com>
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
