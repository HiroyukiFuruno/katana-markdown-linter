# `MD040` - Les blocs de code clôturés doivent indiquer un langage

Étiquettes: `code`, `language`

Alias: `fenced-code-language`

Paramètres:

- `allowed_languages`: valeur de configuration (`string[]`, défaut `[]`)
- `language_only`: valeur de configuration (`boolean`, défaut `false`)

## Vue d'ensemble

Les blocs de code clôturés doivent indiquer un langage. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

````markdown
```

```
```

````markdown
```

```
```

````markdown
```

```
```

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
