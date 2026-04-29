# `MD044` - Scrivi i nomi propri secondo la configurazione

Tag: `spelling`

Alias: `proper-names`

Parametri:

- `code_blocks`: valore di configurazione (`boolean`, predefinito `true`)
- `html_elements`: valore di configurazione (`boolean`, predefinito `true`)
- `names`: valore di configurazione (`string[]`, predefinito `[]`)

## Panoramica

Scrivi i nomi propri secondo la configurazione. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```json
[
    "JavaScript"
]
```

```json
[
    "GitHub",
    "github.com"
]
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
