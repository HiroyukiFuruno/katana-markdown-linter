# `MD011` - Correggi la sintassi di link invertita

Tag: `links`

Alias: `no-reversed-links`

Parametri:

Nessuno.

## Panoramica

Correggi la sintassi di link invertita. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
(Incorrect link syntax)[https://www.example.com/]
```

```markdown
[Correct link syntax](https://www.example.com/)
```

```markdown
For (example)[^1]
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
