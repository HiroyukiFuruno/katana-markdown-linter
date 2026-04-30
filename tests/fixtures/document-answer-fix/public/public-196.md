# `MD040` - I blocchi di codice recintati devono indicare il linguaggio

Tag: `code`, `language`

Alias: `fenced-code-language`

Parametri:

- `allowed_languages`: valore di configurazione (`string[]`, predefinito `[]`)
- `language_only`: valore di configurazione (`boolean`, predefinito `false`)

## Panoramica

I blocchi di codice recintati devono indicare il linguaggio. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

````markdown
```

```
```

````markdown
```

```
```

````markdown
```

```
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
