# `MD034` - Gli URL nudi devono usare parentesi angolari o sintassi di link

Tag: `links`, `url`

Alias: `no-bare-urls`

Parametri:

Nessuno.

## Panoramica

Gli URL nudi devono usare parentesi angolari o sintassi di link. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
For more info, visit https://www.example.com/ or email user@example.com.
```

```markdown
For more info, visit <https://www.example.com/> or email <user@example.com>.
```

```markdown
Not a clickable link: `https://www.example.com`
```

```markdown
[https://www.example.com]
```

```markdown
[text [shortcut] text](https://example.com)
```

```markdown
[link \[text\] link](https://example.com)
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
