# `MD025` - Un documento deve avere un solo titolo di livello superiore

Tag: `headings`

Alias: `single-h1`, `single-title`

Parametri:

- `front_matter_title`: valore di configurazione (`string`, predefinito `^\s*title\s*[:=]`)
- `level`: valore di configurazione (`integer`, predefinito `1`)

## Panoramica

Un documento deve avere un solo titolo di livello superiore. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
# Top level heading

# Another top-level heading
```

```markdown
# Title

## Heading

## Another heading
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
