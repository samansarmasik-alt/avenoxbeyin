use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedNote {
    pub metadata: HashMap<String, Value>,
    pub body: String,
}

/// Extract frontmatter (JSON or simple YAML) and markdown body.
pub fn parse_frontmatter(text: &str) -> Result<ParsedNote, String> {
    let clean = text.trim_start_matches('\u{feff}');
    let mut lines = clean.lines();

    match lines.next() {
        Some(first) if first.trim() == "---" => {}
        _ => {
            return Ok(ParsedNote {
                metadata: HashMap::new(),
                body: text.to_string(),
            });
        }
    }

    let mut header_lines = Vec::new();
    let mut found_end = false;
    let mut body_lines = Vec::new();

    for line in lines {
        if !found_end {
            if line.trim() == "---" {
                found_end = true;
            } else {
                header_lines.push(line);
            }
        } else {
            body_lines.push(line);
        }
    }

    if !found_end {
        return Err("unterminated frontmatter".to_string());
    }

    let header_str = header_lines.join("\n").trim().to_string();
    let body = body_lines.join("\n");

    let mut metadata = HashMap::new();

    if header_str.starts_with('{') {
        let parsed: Value = serde_json::from_str(&header_str)
            .map_err(|e| format!("invalid JSON frontmatter: {}", e))?;
        if let Value::Object(obj) = parsed {
            for (k, v) in obj {
                metadata.insert(k, v);
            }
        } else {
            return Err("frontmatter must be an object".to_string());
        }
    } else {
        // Parse simple key-value YAML header
        for line in &header_lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((key, val)) = trimmed.split_once(':') {
                let k = key.trim().to_string();
                let v_str = val.trim();
                let v = if v_str.starts_with('"') && v_str.ends_with('"') && v_str.len() >= 2 {
                    Value::String(v_str[1..v_str.len() - 1].to_string())
                } else if v_str.starts_with('\'') && v_str.ends_with('\'') && v_str.len() >= 2 {
                    Value::String(v_str[1..v_str.len() - 1].to_string())
                } else if let Ok(n) = v_str.parse::<i64>() {
                    Value::Number(n.into())
                } else if let Ok(b) = v_str.parse::<bool>() {
                    Value::Bool(b)
                } else if v_str.is_empty() {
                    Value::Null
                } else {
                    Value::String(v_str.to_string())
                };
                metadata.insert(k, v);
            }
        }
    }

    Ok(ParsedNote { metadata, body })
}
