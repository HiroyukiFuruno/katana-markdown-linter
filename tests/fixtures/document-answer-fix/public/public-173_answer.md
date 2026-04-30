# `MD014` - C'è un simbolo dollaro non necessario prima dell'esempio di comando

Tag: `code`

Alias: `commands-show-output`

Parametri:

Nessuno.

## Panoramica

C'è un simbolo dollaro non necessario prima dell'esempio di comando. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

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

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
