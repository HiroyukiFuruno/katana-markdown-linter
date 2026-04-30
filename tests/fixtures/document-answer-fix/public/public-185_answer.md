# `MD029` - Numera correttamente gli elenchi ordinati

Tag: `ol`

Alias: `ol-prefix`

Parametri:

- `style`: valore di configurazione (`string`, predefinito `one_or_ordered`)

## Panoramica

Numera correttamente gli elenchi ordinati. Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.

## Configurazione

Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.

## Esempi

```markdown
1. Do this.
1. Do that.
1. Done.
```

```markdown
1. Do this.
2. Do that.
3. Done.
```

```markdown
0. Do this.
1. Do that.
2. Done.
```

```markdown
0. Do this.
0. Do that.
0. Done.
```

```markdown
1. Do this.
3. Done.
```

```markdown
...
08. Item
09. Item
10. Item
11. Item
...
```

```text
Code block
```

```text
   Code block
   ```

## Motivo

Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.
