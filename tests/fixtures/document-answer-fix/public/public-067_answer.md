# `MD014` - Hay un signo de dólar innecesario antes del ejemplo de comando

Etiquetas: `code`

Alias: `commands-show-output`

Parámetros:

Ninguno.

## Resumen

Hay un signo de dólar innecesario antes del ejemplo de comando. Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.

## Configuración

Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.

## Ejemplos

```markdown
ls
cat foo
less bar
```

```markdown
ls
cat foo
less bar
```

```markdown
$ ls
foo bar
$ cat foo
Hello world
$ cat bar
baz
```

```markdown
$ mkdir test
mkdir: created directory 'test'
$ ls test
```

## Motivo

Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.
