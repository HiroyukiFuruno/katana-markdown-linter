# `MD007` - Il rientro degli elenchi non ordinati deve seguire la configurazione

Tag: `bullet`, `indentation`, `ul`

Alias: `ul-indent`

Parametri:

- `indent`: valore di configurazione (`integer`, predefinito `2`)
- `start_indent`: valore di configurazione (`integer`, predefinito `2`)
- `start_indented`: valore di configurazione (`boolean`, predefinito `false`)

## Panoramica

Il rientro degli elenchi non ordinati deve seguire la configurazione. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
* List item
   * Nested list item indented by 3 spaces
```

```markdown
* List item
  * Nested list item indented by 2 spaces
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
