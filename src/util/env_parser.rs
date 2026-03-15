use crate::app::Pair;

/// Parse a .env-format string into key-value pairs.
pub fn parse(content: &str) -> Vec<Pair> {
    let mut pairs = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some(eq_pos) = line.find('=') else {
            continue;
        };
        let key = line[..eq_pos].trim();
        if key.is_empty() {
            continue;
        }
        let raw_value = line[eq_pos + 1..].trim();
        let value =
            if raw_value.len() >= 2 && raw_value.starts_with('"') && raw_value.ends_with('"') {
                &raw_value[1..raw_value.len() - 1]
            } else {
                raw_value
            };
        pairs.push((key.to_string(), value.to_string()));
    }
    pairs
}

/// Serialize key-value pairs into a .env-format string.
pub fn to_string(pairs: &[Pair]) -> String {
    let mut out = String::new();
    for (key, value) in pairs {
        out.push_str(key);
        out.push('=');
        if value.contains(' ') || value.contains('"') {
            out.push('"');
            out.push_str(value);
            out.push('"');
        } else {
            out.push_str(value);
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(key: &str, value: &str) -> (String, String) {
        (key.to_string(), value.to_string())
    }

    #[test]
    fn test_parse() {
        let input = r#"KEY1=value1
KEY2 = value2
KEY3="quoted value"
KEY4="has=equals"
no_equals_line
KEY5=
KEY6 = spaced
"#;
        let pairs = parse(input);
        assert_eq!(
            pairs,
            vec![
                p("KEY1", "value1"),
                p("KEY2", "value2"),
                p("KEY3", "quoted value"),
                p("KEY4", "has=equals"),
                p("KEY5", ""),
                p("KEY6", "spaced"),
            ]
        );
    }

    #[test]
    fn test_build() {
        let pairs = vec![
            p("KEY1", "simple"),
            p("KEY2", "has space"),
            p("KEY3", r#"has"quote"#),
            p("KEY4", ""),
        ];
        let output = to_string(&pairs);
        assert_eq!(
            output,
            r#"KEY1=simple
KEY2="has space"
KEY3="has"quote"
KEY4=
"#
        );
    }
}
