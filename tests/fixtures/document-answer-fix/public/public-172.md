# `MD013` - La lunghezza della riga supera il limite

Tag: `line_length`

Alias: `line-length`

Parametri:

- `code_block_line_length`: valore di configurazione (`integer`, predefinito `80`)
- `code_blocks`: valore di configurazione (`boolean`, predefinito `true`)
- `heading_line_length`: valore di configurazione (`integer`, predefinito `80`)
- `headings`: valore di configurazione (`boolean`, predefinito `true`)
- `line_length`: valore di configurazione (`integer`, predefinito `80`)
- `stern`: valore di configurazione (`boolean`, predefinito `false`)
- `strict`: valore di configurazione (`boolean`, predefinito `false`)
- `tables`: valore di configurazione (`boolean`, predefinito `true`)

## Panoramica

La lunghezza della riga supera il limite. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
IF THIS LINE IS THE MAXIMUM LENGTH
This line is okay because there are-no-spaces-beyond-that-length
This line is a violation because there are spaces beyond that length
This-line-is-okay-because-there-are-no-spaces-anywhere-within
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
