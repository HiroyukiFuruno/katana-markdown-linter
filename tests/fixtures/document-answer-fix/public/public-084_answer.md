# `MD034` - Las URL sueltas deben usar corchetes angulares o sintaxis de enlace

Etiquetas: `links`, `url`

Alias: `no-bare-urls`

Parámetros:

Ninguno.

## Resumen

Las URL sueltas deben usar corchetes angulares o sintaxis de enlace. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
For more info, visit https://www.example.com/ or email user@example.com.
```

```markdown
For more info, visit <https://www.example.com/> or email <user@example.com>.
```

```markdown
Not a clickable link: `https://www.example.com`
```

```markdown
[https://www.example.com]
```

```markdown
[text [shortcut] text](https://example.com)
```

```markdown
[link \[text\] link](https://example.com)
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
