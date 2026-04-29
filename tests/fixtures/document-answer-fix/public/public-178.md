# `MD022` - I titoli devono essere circondati da righe vuote

Tag: `blank_lines`, `headings`

Alias: `blanks-around-headings`

Parametri:

- `lines_above`: valore di configurazione (`integer|integer[]`, predefinito `1`)
- `lines_below`: valore di configurazione (`integer|integer[]`, predefinito `1`)

## Panoramica

I titoli devono essere circondati da righe vuote. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
# Heading 1
Some text

Some more text
## Heading 2
```

```markdown
# Heading 1

Some text

Some more text

## Heading 2
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
