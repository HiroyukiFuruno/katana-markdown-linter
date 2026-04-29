# `MD010` - Non usare tabulazioni rigide

Tag: `hard_tab`, `whitespace`

Alias: `no-hard-tabs`

Parametri:

- `code_blocks`: valore di configurazione (`boolean`, predefinito `true`)
- `ignore_code_languages`: valore di configurazione (`string[]`, predefinito `[]`)
- `spaces_per_tab`: valore di configurazione (`integer`, predefinito `1`)

## Panoramica

Non usare tabulazioni rigide. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
Some text

 * hard tab character used to indent the list item
```

```markdown
Some text

    * Spaces used to indent the list item instead
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
