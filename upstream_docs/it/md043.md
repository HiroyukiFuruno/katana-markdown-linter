# `MD043` - Segui la struttura di titoli richiesta

Tag: `headings`

Alias: `required-headings`

Parametri:

- `headings`: valore di configurazione (`string[]`, predefinito `[]`)
- `match_case`: valore di configurazione (`boolean`, predefinito `false`)

## Panoramica

Segui la struttura di titoli richiesta. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
# Heading
## Item
### Detail
```

```json
[
    "# Heading",
    "## Item",
    "### Detail"
]
```

```markdown
# Heading
## Item
### Detail (optional)
## Foot
### Notes (optional)
```

```json
[
    "# Heading",
    "## Item",
    "*",
    "## Foot",
    "*"
]
```

```markdown
# Project Name
## Description
## Examples
```

```json
[
    "?",
    "## Description",
    "## Examples"
]
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
