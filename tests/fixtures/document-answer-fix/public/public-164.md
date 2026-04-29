# `MD003` - Mantieni uno stile coerente per i titoli

Tag: `headings`

Alias: `heading-style`

Parametri:

- `style`: valore di configurazione (`string`, predefinito `consistent`)

## Panoramica

Mantieni uno stile coerente per i titoli. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
# ATX style H1

## Closed ATX style H2 ##

Setext style H1
===============
```

```markdown
# ATX style H1

## ATX style H2
```

```markdown
Setext style H1
===============

Setext style H2
---------------

### ATX style H3
```

```markdown
A line of text followed by a horizontal rule becomes a heading
---
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
