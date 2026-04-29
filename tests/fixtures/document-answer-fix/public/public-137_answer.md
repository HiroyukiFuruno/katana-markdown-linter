# `MD034` - Les URL brutes doivent utiliser des chevrons ou une syntaxe de lien

Étiquettes: `links`, `url`

Alias: `no-bare-urls`

Paramètres:

Aucun.

## Vue d'ensemble

Les URL brutes doivent utiliser des chevrons ou une syntaxe de lien. Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.

## Configuration

Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.

## Exemples

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

## Raison

Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.
