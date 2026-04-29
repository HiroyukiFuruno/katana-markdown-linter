use super::KmlMcpRemoteServer;
use crate::model::{RuleDocRequest, RuleGetRequest, RuleListRequest};
use rmcp::{handler::server::wrapper::Parameters, Json};

#[tokio::test]
async fn remote_rule_tools_return_localized_content() {
    let server = KmlMcpRemoteServer::new();
    let Json(list) = server
        .rule_list(Parameters(RuleListRequest {
            locale: Some("es-ES".to_string()),
        }))
        .await;
    assert_eq!(list.locale, "es");
    assert!(list.rules.iter().any(|rule| {
        rule.id == "MD003" && rule.description == "Mantén un estilo de encabezado coherente"
    }));

    let Json(rule) = server
        .rule_get(Parameters(RuleGetRequest {
            rule_id: "MD003".to_string(),
            locale: Some("es".to_string()),
        }))
        .await
        .expect("MD003 should exist");
    assert_eq!(rule.locale, "es");
    assert_ne!(rule.description, rule.english_description);

    let Json(document) = server
        .rule_doc_get(Parameters(RuleDocRequest {
            rule_id: "MD003".to_string(),
            locale: Some("es".to_string()),
        }))
        .await
        .expect("MD003 documentation should exist");
    assert_eq!(document.locale, "es");
    assert!(document
        .content
        .contains("Mantén un estilo de encabezado coherente"));
}
