use super::*;

#[tokio::test]
async fn check_text_reports_structured_diagnostics() {
    let server = KmlMcpServer::new();
    let Json(response) = server
        .check_text(Parameters(CheckTextRequest {
            content: "# title\n\n### skipped\n".to_string(),
            locale: Some("ja-JP".to_string()),
        }))
        .await
        .expect("check_text should succeed");

    assert!(response.issue_count > 0);
    let diagnostic = response
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.rule_id == "MD001")
        .expect("MD001 should be reported");
    assert_eq!(diagnostic.message_id, "rule.MD001.heading_increment");
    assert!(diagnostic.message.contains("見出しレベル"));
}

#[tokio::test]
async fn fix_text_returns_fixed_content_without_file_tools() {
    let server = KmlMcpServer::new();
    let Json(response) = server
        .fix_text(Parameters(FixTextRequest {
            content: "#Title\n".to_string(),
            locale: Some("ja".to_string()),
        }))
        .await
        .expect("fix_text should succeed");

    assert!(response.applied_fixes > 0);
    assert_eq!(response.content, "# Title\n");
}

#[tokio::test]
async fn config_validate_reports_structured_errors() {
    let server = KmlMcpServer::new();
    let Json(response) = server
        .config_validate(Parameters(ConfigValidateRequest {
            config: json!({ "MD999": true }),
            locale: Some("ja".to_string()),
        }))
        .await;

    assert!(!response.valid);
    assert_eq!(response.error_count, 1);
    assert_eq!(response.errors[0].kind, "unknown_rule");
    assert_eq!(response.errors[0].rule_id.as_deref(), Some("MD999"));
    assert_eq!(response.errors[0].message_id, "config.unknown_rule");
    assert_eq!(
        response.errors[0].message,
        "未知の markdownlint rule です: MD999"
    );
}

#[tokio::test]
async fn rule_tools_expose_catalog_metadata() {
    let server = KmlMcpServer::new();
    let Json(list) = server
        .rule_list(Parameters(RuleListRequest {
            locale: Some("ja".to_string()),
        }))
        .await;
    assert_eq!(list.locale, "ja");
    assert!(list.rules.iter().any(|rule| rule.id == "MD001"));
    assert!(list.rules.iter().any(|rule| {
        rule.id == "MD003" && rule.description == "見出しのスタイルを統一してください"
    }));

    let Json(rule) = server
        .rule_get(Parameters(RuleGetRequest {
            rule_id: "md001".to_string(),
            locale: Some("ja-JP".to_string()),
        }))
        .await
        .expect("MD001 should exist");
    assert_eq!(rule.id, "MD001");
    assert_eq!(rule.locale, "ja");
    assert_ne!(rule.description, rule.english_description);
}

#[tokio::test]
async fn rule_doc_tool_returns_localized_markdown() {
    let server = KmlMcpServer::new();
    let Json(response) = server
        .rule_doc_get(Parameters(RuleDocRequest {
            rule_id: "MD013".to_string(),
            locale: Some("fr-FR".to_string()),
        }))
        .await
        .expect("MD013 documentation should exist");

    assert_eq!(response.rule_id, "MD013");
    assert_eq!(response.locale, "fr");
    assert!(response.content.contains("La longueur de ligne"));
    assert!(response.content.contains("## Vue d'ensemble"));
    assert!(!response.content.contains("# `MD013` - Line length"));
}
