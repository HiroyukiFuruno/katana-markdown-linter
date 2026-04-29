# `MD036` - Non usare una riga solo enfatizzata come titolo

Tag: `emphasis`, `headings`

Alias: `no-emphasis-as-heading`

Parametri:

- `punctuation`: valore di configurazione (`string`, predefinito `.,;:!?。，；：！？`)

## Panoramica

Non usare una riga solo enfatizzata come titolo. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
**My document**

Lorem ipsum dolor sit amet...

_Another section_

Consectetur adipiscing elit, sed do eiusmod.
```

```markdown
# My document

Lorem ipsum dolor sit amet...

## Another section

Consectetur adipiscing elit, sed do eiusmod.
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
