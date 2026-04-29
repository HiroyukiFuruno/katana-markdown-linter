# `MD042` - Non usare link vuoti

Tag: `links`

Alias: `no-empty-links`

Parametri:

Nessuno.

## Panoramica

Non usare link vuoti. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
[an empty link]()
```

```markdown
[a valid link](https://example.com/)
```

```markdown
[an empty fragment](#)
```

```markdown
[a valid fragment](#fragment)
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
