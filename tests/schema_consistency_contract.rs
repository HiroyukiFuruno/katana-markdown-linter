use katana_markdown_linter::RuleCatalogService;
use serde_json::Value;

#[test]
fn test_schema_consistent_with_cli_output() {
    let schema_content = std::fs::read_to_string("schema/markdownlint.schema.json")
        .expect("Failed to read schema/markdownlint.schema.json");
    let schema_json: Value =
        serde_json::from_str(&schema_content).expect("Failed to parse schema JSON");

    // Check if the schema has the expected title
    assert_eq!(
        schema_json["title"], "katana-markdown-linter configuration",
        "Schema title mismatch"
    );

    // Check if some core rules are present in properties
    let properties = schema_json["properties"]
        .as_object()
        .expect("Schema properties should be an object");

    assert!(
        properties.contains_key("default"),
        "Schema should contain 'default' property"
    );
    assert!(
        properties.contains_key("MD001"),
        "Schema should contain 'MD001' property"
    );
    assert!(
        properties.contains_key("heading-increment"),
        "Schema should contain 'heading-increment' alias"
    );
}

#[test]
fn test_schema_rule_metadata_consistency() {
    let schema_content = std::fs::read_to_string("schema/markdownlint.schema.json")
        .expect("Failed to read schema/markdownlint.schema.json");
    let schema_json: Value =
        serde_json::from_str(&schema_content).expect("Failed to parse schema JSON");
    let properties = schema_json["properties"].as_object().unwrap();

    for rule in RuleCatalogService::available_rules() {
        let rule_id = &rule.id;
        assert!(
            properties.contains_key(rule_id),
            "Rule {} missing from schema properties",
            rule_id
        );

        // Check if description is present and contains reference link if available
        let rule_prop = &properties[rule_id];
        let description = rule_prop["description"].as_str().unwrap_or("");
        assert!(
            !description.is_empty(),
            "Description for rule {} should not be empty",
            rule_id
        );

        for alias in &rule.aliases {
            assert!(
                properties.contains_key(alias),
                "Alias {} for rule {} missing from schema properties",
                alias,
                rule_id
            );
        }
    }
}

#[test]
fn test_schema_compatibility_v0_18_0() {
    let current_schema_content = std::fs::read_to_string("schema/markdownlint.schema.json")
        .expect("Failed to read schema/markdownlint.schema.json");
    let current: Value =
        serde_json::from_str(&current_schema_content).expect("Failed to parse current schema");

    let v0_18_0_content = std::fs::read_to_string("tests/fixtures/schema/v0.18.0.schema.json")
        .expect("Failed to read tests/fixtures/schema/v0.18.0.schema.json");
    let v0_18_0: Value =
        serde_json::from_str(&v0_18_0_content).expect("Failed to parse v0.18.0 schema");

    // Compatibility check: current schema must be a superset of v0.18.0 (additive changes allowed)
    let current_props = current["properties"].as_object().unwrap();
    let v0_18_0_props = v0_18_0["properties"].as_object().unwrap();

    for (key, v0_18_0_val) in v0_18_0_props {
        let current_val = current_props.get(key).unwrap_or_else(|| {
            panic!(
                "Breaking change: property '{}' from v0.18.0 missing in current schema",
                key
            )
        });

        // Basic type/enum compatibility check
        // Note: This is a simple check, could be more elaborate
        if v0_18_0_val["type"] != current_val["type"] {
            panic!(
                "Breaking change: type of '{}' changed from {:?} to {:?}",
                key, v0_18_0_val["type"], current_val["type"]
            );
        }
    }
}
