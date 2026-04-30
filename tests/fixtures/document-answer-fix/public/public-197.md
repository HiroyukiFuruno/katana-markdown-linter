# `MD041` - La prima riga del file deve essere un titolo di livello superiore

Tag: `headings`

Alias: `first-line-h1`, `first-line-heading`

Parametri:

- `allow_preamble`: valore di configurazione (`boolean`, predefinito `false`)
- `front_matter_title`: valore di configurazione (`string`, predefinito `^\s*title\s*[:=]`)
- `level`: valore di configurazione (`integer`, predefinito `1`)

## Panoramica

La prima riga del file deve essere un titolo di livello superiore. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
This is a document without a heading
```

```markdown
# Document Heading

This is a document with a top-level heading
```

```markdown
<h1 align="center"><img src="https://placekitten.com/300/150"/></h1>

This is a document with a top-level HTML heading
```

```markdown
This is a document with preamble text

# Document Heading
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
