# `MD029` - Numera correctamente las listas ordenadas

Etiquetas: `ol`

Alias: `ol-prefix`

Parámetros:

- `style`: valor de configuración (`string`, predeterminado `one_or_ordered`)

## Resumen

Numera correctamente las listas ordenadas. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
1. Do this.
1. Do that.
1. Done.
```

```markdown
1. Do this.
2. Do that.
3. Done.
```

```markdown
0. Do this.
1. Do that.
2. Done.
```

```markdown
0. Do this.
0. Do that.
0. Done.
```

```markdown
1. Do this.
3. Done.
```

```markdown
...
08. Item
09. Item
10. Item
11. Item
...
```

```text
Code block
```

```text
   Code block
   ```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
