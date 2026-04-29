# `MD032` - Gli elenchi devono essere circondati da righe vuote

Tag: `blank_lines`, `bullet`, `ol`, `ul`

Alias: `blanks-around-lists`

Parametri:

Nessuno.

## Panoramica

Gli elenchi devono essere circondati da righe vuote. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

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

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
