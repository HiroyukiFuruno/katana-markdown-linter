# `MD031` - I blocchi di codice devono essere circondati da righe vuote

Tag: `blank_lines`, `code`

Alias: `blanks-around-fences`

Parametri:

- `list_items`: valore di configurazione (`boolean`, predefinito `true`)

## Panoramica

I blocchi di codice devono essere circondati da righe vuote. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

````markdown
Some text
```

```

```

```
Some more text
```

````markdown
Some text

```

```

```

```

Some more text
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
