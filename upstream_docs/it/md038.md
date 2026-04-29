# `MD038` - Non inserire spazi dentro i marcatori di codice

Tag: `code`, `whitespace`

Alias: `no-space-in-code`

Parametri:

Nessuno.

## Panoramica

Non inserire spazi dentro i marcatori di codice. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
`some text `

` some text`

`   some text   `
```

```markdown
`some text`
```

```markdown
`` `backticks` ``

`` backtick` ``
```

```markdown
` code `
```

```markdown
` `

`   `
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
