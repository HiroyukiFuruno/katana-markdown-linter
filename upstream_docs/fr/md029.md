# `MD029` - Numérotez correctement les listes ordonnées

Étiquettes: `ol`

Alias: `ol-prefix`

Paramètres:

- `style`: valeur de configuration (`string`, défaut `one_or_ordered`)

## Vue d'ensemble

Numérotez correctement les listes ordonnées. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

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

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
