# `MD051` - El fragmento de encabezado enlazado no existe

Etiquetas: `links`

Alias: `link-fragments`

Parámetros:

- `ignore_case`: valor de configuración (`boolean`, predeterminado `false`)
- `ignored_pattern`: valor de configuración (`string`, predeterminado ``)

## Resumen

El fragmento de encabezado enlazado no existe. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
# Heading Name

[Link](#fragment)
```

```markdown
# Heading Name

[Link](#heading-name)
```

```markdown
# Heading Name

[Link](#Heading-Name)
```

```markdown
# Heading Name {#custom-name}

[Link](#custom-name)
```

```markdown
<a id="bookmark"></a>

[Link](#bookmark)
```

```markdown
[Link](#top)
```

```markdown
[Link](#L20)
```

```markdown
[Link](#L19C5-L21C11)
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
