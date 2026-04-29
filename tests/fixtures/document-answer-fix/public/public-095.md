# `MD045` - Las imágenes deben tener texto alternativo

Etiquetas: `accessibility`, `images`

Alias: `no-alt-text`

Parámetros:

Ninguno.

## Resumen

Las imágenes deben tener texto alternativo. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
![Alternate text](image.jpg)
```

```markdown
![Alternate text][ref]

...

[ref]: image.jpg "Optional title"
```

```html
<img src="image.jpg" alt="Alternate text" />
```

```html
<img src="image.jpg" aria-hidden="true" />
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
