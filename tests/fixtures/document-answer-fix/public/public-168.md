# `MD009` - Rimuovi gli spazi superflui a fine riga

Tag: `whitespace`

Alias: `no-trailing-spaces`

Parametri:

- `br_spaces`: valore di configurazione (`integer`, predefinito `2`)
- `code_blocks`: valore di configurazione (`boolean`, predefinito `false`)
- `list_item_empty_lines`: valore di configurazione (`boolean`, predefinito `false`)
- `strict`: valore di configurazione (`boolean`, predefinito `false`)

## Panoramica

Rimuovi gli spazi superflui a fine riga. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
Text text text
text[2 spaces]
```

```markdown
- list item text
  [2 spaces]
  list item text
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
