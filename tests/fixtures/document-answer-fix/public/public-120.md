# `MD014` - Un signe dollar inutile précède l'exemple de commande

Étiquettes: `code`

Alias: `commands-show-output`

Paramètres:

Aucun.

## Vue d'ensemble

Un signe dollar inutile précède l'exemple de commande. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
$ ls
$ cat foo
$ less bar
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

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
