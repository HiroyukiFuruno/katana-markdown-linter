# `MD028` - C'è una riga vuota non necessaria dentro la citazione

Tag: `blockquote`, `whitespace`

Alias: `no-blanks-blockquote`

Parametri:

Nessuno.

## Panoramica

C'è una riga vuota non necessaria dentro la citazione. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
> This is a blockquote
> which is immediately followed by

> this blockquote. Unfortunately
> In some parsers, these are treated as the same blockquote.
```

```markdown
> This is a blockquote.

And Jimmy also said:

> This too is a blockquote.
```

```markdown
> This is a blockquote.
>
> This is the same blockquote.
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
