#!/usr/bin/env python3
from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DOC_ROOT = ROOT / "upstream_docs"
LOCALE_ROOT = ROOT / "src" / "i18n" / "locales"


def load_json(path: Path) -> dict[str, Any] | list[dict[str, Any]]:
    return json.loads(path.read_text())


def languages() -> list[str]:
    entries = load_json(LOCALE_ROOT / "languages.json")
    if not isinstance(entries, list):
        raise TypeError("languages.json must be an array")
    return [str(entry["code"]) for entry in entries]


def locale_catalog(locale: str) -> dict[str, Any]:
    catalog = load_json(LOCALE_ROOT / f"{locale}.json")
    if not isinstance(catalog, dict):
        raise TypeError(f"{locale}.json must be an object")
    return catalog


def metadata(source: str) -> tuple[str, str, str, list[str], list[str]]:
    lines = source.splitlines()
    rule_match = re.match(r"# `(MD\d+)` - ", lines[0])
    if rule_match is None:
        raise ValueError(f"invalid rule title: {lines[0]}")
    rule_id = rule_match.group(1)
    tags = next((line.removeprefix("Tags:").strip() for line in lines if line.startswith("Tags:")), "")
    aliases = next(
        (line.removeprefix("Aliases:").strip() for line in lines if line.startswith("Aliases:")),
        "",
    )
    params: list[str] = []
    current_param: list[str] = []
    for line in lines:
        if line.startswith("- `"):
            if current_param:
                params.append(" ".join(current_param))
            current_param = [line]
        elif current_param and line.startswith("  "):
            current_param.append(line.strip())
        elif current_param and line.strip() == "":
            params.append(" ".join(current_param))
            current_param = []
        elif current_param:
            params.append(" ".join(current_param))
            current_param = []
    if current_param:
        params.append(" ".join(current_param))
    examples = re.findall(r"```.*?```", source, flags=re.DOTALL)
    return rule_id, tags, aliases, params, examples


def render_params(params: list[str], label: str, default_label: str, none_label: str) -> str:
    if not params:
        return none_label
    rendered = []
    for param in params:
        key_match = re.match(r"- `([^`]+)`:", param)
        if key_match is None:
            raise ValueError(f"invalid parameter entry: {param}")
        value_type = re.search(r"\(`([^`]+)`", param)
        default = re.search(r"default `([^`]*)`", param)
        details = f"(`{value_type.group(1)}`" if value_type else ""
        if default:
            details += f", {default_label} `{default.group(1)}`"
        details += ")" if details else ""
        rendered.append(f"- `{key_match.group(1)}`: {label} {details}".rstrip())
    return "\n".join(rendered)


def render_doc(source: str, labels: dict[str, str], description: str) -> str:
    rule_id, tags, aliases, params, examples = metadata(source)
    examples_text = "\n\n".join(examples) if examples else labels["none"]
    params_text = render_params(params, labels["setting"], labels["default"], labels["none"])
    separator = "" if labels["period"] == "。" else " "
    return f"""# `{rule_id}` - {description}

{labels["tags"]}: {tags}

{labels["aliases"]}: {aliases}

{labels["parameters"]}:

{params_text}

## {labels["overview"]}

{description}{labels["period"]}{separator}{labels["overview_text"]}

## {labels["config"]}

{labels["config_text"]}

## {labels["examples"]}

{examples_text}

## {labels["rationale"]}

{labels["rationale_text"]}
"""


def main() -> None:
    english_docs = sorted(DOC_ROOT.glob("md*.md"))
    for locale in languages():
        if locale == "en":
            continue
        catalog = locale_catalog(locale)
        rule_doc = catalog["rule_doc"]
        if rule_doc["generation"] != "generated":
            continue
        labels = rule_doc["labels"]
        descriptions = catalog["rule_descriptions"]
        locale_dir = DOC_ROOT / locale
        locale_dir.mkdir(exist_ok=True)
        for path in english_docs:
            rule_id, _, _, _, _ = metadata(path.read_text())
            (locale_dir / path.name).write_text(
                render_doc(path.read_text(), labels, descriptions[rule_id])
            )


if __name__ == "__main__":
    main()
