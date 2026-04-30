# `MD001` - I livelli dei titoli devono aumentare solo di un livello alla volta

Tag: `headings`

Alias: `heading-increment`

Parametri:

- `front_matter_title`: valore di configurazione (`string`, predefinito `^\s*title\s*[:=]`)

## Panoramica

I livelli dei titoli devono aumentare solo di un livello alla volta. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
# Heading 1

### Heading 3

We skipped out a 2nd level heading in this document
```

```markdown
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

## Another Heading 2

### Another Heading 3
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
