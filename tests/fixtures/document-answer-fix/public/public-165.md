# `MD004` - Mantieni uno stile coerente per gli elenchi non ordinati

Tag: `bullet`, `ul`

Alias: `ul-style`

Parametri:

- `style`: valore di configurazione (`string`, predefinito `consistent`)

## Panoramica

Mantieni uno stile coerente per gli elenchi non ordinati. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
* Item 1
+ Item 2
- Item 3
```

```markdown
* Item 1
* Item 2
* Item 3
```

```markdown
* Item 1
  + Item 2
    - Item 3
  + Item 4
* Item 4
  + Item 5
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
