use beyin_core::parse_frontmatter;
use serde_json::Value;

#[test]
fn test_parse_plain_markdown() {
    let doc = "# Plain Markdown\nNo frontmatter here.";
    let parsed = parse_frontmatter(doc).unwrap();
    assert!(parsed.metadata.is_empty());
    assert_eq!(parsed.body, doc);
}

#[test]
fn test_parse_yaml_frontmatter() {
    let doc = "---\ntitle: \"Proje Notu\"\nstatus: active\nrevision: 2\n---\n# Baslik\nGovde metni.";
    let parsed = parse_frontmatter(doc).unwrap();
    assert_eq!(
        parsed.metadata.get("title"),
        Some(&Value::String("Proje Notu".to_string()))
    );
    assert_eq!(
        parsed.metadata.get("status"),
        Some(&Value::String("active".to_string()))
    );
    assert_eq!(parsed.metadata.get("revision"), Some(&Value::Number(2.into())));
    assert_eq!(parsed.body, "# Baslik\nGovde metni.");
}

#[test]
fn test_parse_json_frontmatter() {
    let doc = "---\n{\n  \"id\": \"note-1\",\n  \"kind\": \"receipt\"\n}\n---\nReceipt body content";
    let parsed = parse_frontmatter(doc).unwrap();
    assert_eq!(
        parsed.metadata.get("id"),
        Some(&Value::String("note-1".to_string()))
    );
    assert_eq!(
        parsed.metadata.get("kind"),
        Some(&Value::String("receipt".to_string()))
    );
    assert_eq!(parsed.body, "Receipt body content");
}

#[test]
fn test_unterminated_frontmatter() {
    let doc = "---\ntitle: Eksik\nGovde";
    assert!(parse_frontmatter(doc).is_err());
}
