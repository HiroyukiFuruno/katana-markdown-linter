# `MD027` - Rimuovi gli spazi superflui dopo il marcatore di citazione

Tag: `blockquote`, `indentation`, `whitespace`

Alias: `no-multiple-space-blockquote`

Parametri:

- `list_items`: valore di configurazione (`boolean`, predefinito `true`)

## Panoramica

Rimuovi gli spazi superflui dopo il marcatore di citazione. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
>  This is a blockquote with bad indentation
>  there should only be one.
```

```markdown
> This is a blockquote with correct
> indentation.
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
