# `MD030` - Mantieni lo stesso numero di spazi dopo i marcatori di elenco

Tag: `ol`, `ul`, `whitespace`

Alias: `list-marker-space`

Parametri:

- `ol_multi`: valore di configurazione (`integer`, predefinito `1`)
- `ol_single`: valore di configurazione (`integer`, predefinito `1`)
- `ul_multi`: valore di configurazione (`integer`, predefinito `1`)
- `ul_single`: valore di configurazione (`integer`, predefinito `1`)

## Panoramica

Mantieni lo stesso numero di spazi dopo i marcatori di elenco. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
* Foo
* Bar
* Baz

1. Foo
1. Bar
1. Baz

1. Foo
   * Bar
1. Baz
```

```markdown
* Foo
* Bar
* Baz
```

```markdown
*   Foo

    Second paragraph

*   Bar
```

```markdown
1.  Foo

    Second paragraph

1.  Bar
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
