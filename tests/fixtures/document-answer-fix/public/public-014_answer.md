# `MD014` - Vor dem Befehlsbeispiel steht ein unnötiges Dollarzeichen

Tags: `code`

Aliasse: `commands-show-output`

Parameter:

Keine.

## Überblick

Vor dem Befehlsbeispiel steht ein unnötiges Dollarzeichen. Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.

## Konfiguration

Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.

## Beispiele

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

## Begründung

Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.
