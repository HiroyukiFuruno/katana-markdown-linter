# `MD033` - N'utilisez pas de HTML en ligne

Étiquettes: `html`

Alias: `no-inline-html`

Paramètres:

- `allowed_elements`: valeur de configuration (`string[]`, défaut `[]`)
- `table_allowed_elements`: valeur de configuration (`string[]`, défaut `[]`)

## Vue d'ensemble

N'utilisez pas de HTML en ligne. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

```markdown
<h1>Inline HTML heading</h1>
```

```markdown
# Markdown heading
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
