# `MD005` - Allinea il rientro degli elementi allo stesso livello

Tag: `bullet`, `indentation`, `ul`

Alias: `list-indent`

Parametri:

Nessuno.

## Panoramica

Allinea il rientro degli elementi allo stesso livello. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
   * A misaligned item
```

```markdown
* Item 1
  * Nested Item 1
  * Nested Item 2
  * Nested Item 3
```

```markdown
...
8. Item
9. Item
10. Item
11. Item
...
```

```markdown
...
 8. Item
 9. Item
10. Item
11. Item
...
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
