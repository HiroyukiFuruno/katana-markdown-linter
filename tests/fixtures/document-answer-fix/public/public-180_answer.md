# `MD024` - Sono presenti titoli duplicati

Tag: `headings`

Alias: `no-duplicate-heading`

Parametri:

- `siblings_only`: valore di configurazione (`boolean`, predefinito `false`)

## Panoramica

Sono presenti titoli duplicati. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
# Some text

## Some text
```

```markdown
# Some text

## Some more text
```

```markdown
# Change log

## 1.0.0

### Features

## 2.0.0

### Features
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
