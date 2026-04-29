#!/usr/bin/env python3
from __future__ import annotations

import ast
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DOC_ROOT = ROOT / "upstream_docs"
DESCRIPTION_ROOT = ROOT / "src" / "i18n" / "rule_descriptions"

FIELDS = (
    "module",
    "tags",
    "aliases",
    "parameters",
    "none",
    "setting",
    "default",
    "overview",
    "config",
    "examples",
    "rationale",
    "overview_text",
    "config_text",
    "rationale_text",
    "period",
)

LOCALE_ROWS = {
    "zh-CN": ("zh_cn", "标签", "别名", "参数", "无。", "配置值", "默认值", "概览", "配置", "示例", "原因", "此规则用于让 Markdown 文档保持一致、清晰且便于维护。", "可以使用上方列出的配置键调整此规则的行为。", "一致的写法能降低阅读和维护成本，也能让自动化工具更可靠地处理文档。", "。"),
    "zh-TW": ("zh_tw", "標籤", "別名", "參數", "無。", "設定值", "預設值", "概覽", "設定", "範例", "原因", "此規則用來讓 Markdown 文件保持一致、清楚且容易維護。", "可以使用上方列出的設定鍵調整此規則的行為。", "一致的寫法可降低閱讀與維護成本，也能讓自動化工具更可靠地處理文件。", "。"),
    "ko": ("ko", "태그", "별칭", "매개변수", "없음.", "설정값", "기본값", "개요", "설정", "예시", "이유", "이 규칙은 Markdown 문서를 일관되고 읽기 쉬우며 유지보수하기 좋게 유지합니다.", "위에 나열된 설정 키로 이 규칙의 동작을 조정할 수 있습니다.", "일관된 작성 방식은 읽기와 유지보수 비용을 줄이고 자동화 도구가 문서를 더 안정적으로 처리하게 합니다.", "."),
    "pt": ("pt", "Tags", "Aliases", "Parâmetros", "Nenhum.", "valor de configuração", "padrão", "Visão geral", "Configuração", "Exemplos", "Motivo", "Esta regra ajuda a manter documentos Markdown consistentes, legíveis e fáceis de manter.", "Use as chaves de configuração listadas acima para ajustar o comportamento desta regra.", "Uma escrita consistente reduz o custo de leitura e manutenção e torna a automação mais confiável.", "."),
    "fr": ("fr", "Étiquettes", "Alias", "Paramètres", "Aucun.", "valeur de configuration", "défaut", "Vue d'ensemble", "Configuration", "Exemples", "Raison", "Cette règle aide à garder les documents Markdown cohérents, lisibles et faciles à maintenir.", "Utilisez les clés de configuration ci-dessus pour ajuster le comportement de cette règle.", "Une écriture cohérente réduit le coût de lecture et de maintenance et rend l'automatisation plus fiable.", "."),
    "de": ("de", "Tags", "Aliasse", "Parameter", "Keine.", "Konfigurationswert", "Standard", "Überblick", "Konfiguration", "Beispiele", "Begründung", "Diese Regel hält Markdown-Dokumente einheitlich, gut lesbar und wartbar.", "Mit den oben aufgeführten Konfigurationsschlüsseln lässt sich das Verhalten dieser Regel anpassen.", "Einheitliche Schreibweisen senken den Lese- und Wartungsaufwand und machen Automatisierung verlässlicher.", "."),
    "es": ("es", "Etiquetas", "Alias", "Parámetros", "Ninguno.", "valor de configuración", "predeterminado", "Resumen", "Configuración", "Ejemplos", "Motivo", "Esta regla ayuda a mantener los documentos Markdown coherentes, legibles y fáciles de mantener.", "Usa las claves de configuración anteriores para ajustar el comportamiento de esta regla.", "Una escritura coherente reduce el coste de lectura y mantenimiento y hace que la automatización sea más fiable.", "."),
    "it": ("it", "Tag", "Alias", "Parametri", "Nessuno.", "valore di configurazione", "predefinito", "Panoramica", "Configurazione", "Esempi", "Motivo", "Questa regola aiuta a mantenere i documenti Markdown coerenti, leggibili e facili da mantenere.", "Usa le chiavi di configurazione elencate sopra per regolare il comportamento di questa regola.", "Una scrittura coerente riduce il costo di lettura e manutenzione e rende l'automazione più affidabile.", "."),
}

LOCALES = {locale: dict(zip(FIELDS, values)) for locale, values in LOCALE_ROWS.items()}


def descriptions(module: str) -> dict[str, str]:
    source = (DESCRIPTION_ROOT / f"{module}.rs").read_text()
    pattern = re.compile(r'"(MD\d+)"\s*=>\s*Some\((".*?")\)', re.DOTALL)
    return {rule_id: ast.literal_eval(value) for rule_id, value in pattern.findall(source)}


def metadata(source: str) -> tuple[str, str, list[str], list[str]]:
    lines = source.splitlines()
    rule_id = re.match(r"# `(MD\d+)` - ", lines[0]).group(1)
    tags = next((line.removeprefix("Tags:").strip() for line in lines if line.startswith("Tags:")), "")
    aliases = next((line.removeprefix("Aliases:").strip() for line in lines if line.startswith("Aliases:")), "")
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
        key = re.match(r"- `([^`]+)`:", param).group(1)
        value_type = re.search(r"\(`([^`]+)`", param)
        default = re.search(r"default `([^`]*)`", param)
        details = f"(`{value_type.group(1)}`" if value_type else ""
        if default:
            details += f", {default_label} `{default.group(1)}`"
        details += ")" if details else ""
        rendered.append(f"- `{key}`: {label} {details}".rstrip())
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
    for locale, labels in LOCALES.items():
        locale_dir = DOC_ROOT / locale
        locale_dir.mkdir(exist_ok=True)
        localized = descriptions(labels["module"])
        for path in english_docs:
            rule_id, _, _, _, _ = metadata(path.read_text())
            (locale_dir / path.name).write_text(
                render_doc(path.read_text(), labels, localized[rule_id])
            )


if __name__ == "__main__":
    main()
