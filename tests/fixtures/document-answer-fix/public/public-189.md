# `MD033` - Non usare HTML inline

Tag: `html`

Alias: `no-inline-html`

Parametri:

- `allowed_elements`: valore di configurazione (`string[]`, predefinito `[]`)
- `table_allowed_elements`: valore di configurazione (`string[]`, predefinito `[]`)

## Panoramica

Non usare HTML inline. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
<h1>Inline HTML heading</h1>
```

```markdown
# Markdown heading
```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
