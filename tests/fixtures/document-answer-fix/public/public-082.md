# `MD032` - Las listas deben estar rodeadas de líneas en blanco

Etiquetas: `blank_lines`, `bullet`, `ol`, `ul`

Alias: `blanks-around-lists`

Parámetros:

Ninguno.

## Resumen

Las listas deben estar rodeadas de líneas en blanco. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
Some text
* List item
* List item

1. List item
2. List item
***
```

```markdown
Some text

* List item
* List item

1. List item
2. List item

***
```

```markdown
1. List item
   More item 1
2. List item
More item 2
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
