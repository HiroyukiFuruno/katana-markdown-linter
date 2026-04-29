# `MD026` - Rimuovi la punteggiatura finale dai titoli

Tag: `headings`

Alias: `no-trailing-punctuation`

Parametri:

- `punctuation`: valore di configurazione (`string`, predefinito `.,;:!。，；：！`)

## Panoramica

Rimuovi la punteggiatura finale dai titoli. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
# This is a heading.
```

```markdown
# This is a heading
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
