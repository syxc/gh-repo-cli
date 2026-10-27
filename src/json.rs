//! JSON writer that byte-matches Go's `encoding/json` (`MarshalIndent(v, "", "  ")`).
//!
//! Go escapes `<`, `>`, `&` in strings by default and distinguishes nil slices
//! (`null`) from empty slices (`[]`); output files written by the Go build
//! depend on both details, so this writer reproduces them instead of using a
//! general-purpose JSON crate.

/// A JSON value. Object keys are written in the given order: Go structs keep
/// field order and Go maps sort keys, so callers pass keys already ordered.
pub enum Json {
    Null,
    Int(i64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(&'static str, Json)>),
}

impl Json {
    /// Renders the value with two-space indentation, like `MarshalIndent`.
    pub fn marshal_indent(&self) -> String {
        let mut out = String::new();
        write_value(self, 0, &mut out);
        out
    }
}

fn write_value(value: &Json, depth: usize, out: &mut String) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Int(number) => out.push_str(&number.to_string()),
        Json::Str(text) => write_string(text, out),
        Json::Array(items) if items.is_empty() => out.push_str("[]"),
        Json::Array(items) => {
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                indent(depth + 1, out);
                write_value(item, depth + 1, out);
                if index + 1 < items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(depth, out);
            out.push(']');
        }
        Json::Object(fields) if fields.is_empty() => out.push_str("{}"),
        Json::Object(fields) => {
            out.push_str("{\n");
            for (index, (key, field)) in fields.iter().enumerate() {
                indent(depth + 1, out);
                write_string(key, out);
                out.push_str(": ");
                write_value(field, depth + 1, out);
                if index + 1 < fields.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(depth, out);
            out.push('}');
        }
    }
}

fn indent(depth: usize, out: &mut String) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

/// Escapes exactly like Go's `encoding/json` with the default `escapeHTML`
/// setting: control characters, `<`, `>`, `&` and the U+2028/U+2029 line
/// separators.
fn write_string(text: &str, out: &mut String) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            character if (character as u32) < 0x20 => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", character as u32);
            }
            character => out.push(character),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_html_characters_like_go() {
        let value = Json::Str("a<b>&c\"d\\e".to_owned());
        assert_eq!(
            value.marshal_indent(),
            "\"a\\u003cb\\u003e\\u0026c\\\"d\\\\e\""
        );
    }

    #[test]
    fn escapes_control_characters_like_go() {
        let value = Json::Str("a\u{7}b\n".to_owned());
        assert_eq!(value.marshal_indent(), "\"a\\u0007b\\n\"");
    }

    #[test]
    fn keeps_non_ascii_characters_unescaped() {
        let value = Json::Str("中文".to_owned());
        assert_eq!(value.marshal_indent(), "\"中文\"");
    }

    #[test]
    fn distinguishes_empty_and_null_arrays() {
        assert_eq!(Json::Array(Vec::new()).marshal_indent(), "[]");
        assert_eq!(Json::Null.marshal_indent(), "null");
    }

    #[test]
    fn writes_object_keys_in_given_order() {
        let value = Json::Object(vec![
            ("z", Json::Int(1)),
            ("a", Json::Array(vec![Json::Str("x".to_owned())])),
        ]);

        assert_eq!(
            value.marshal_indent(),
            "{\n  \"z\": 1,\n  \"a\": [\n    \"x\"\n  ]\n}"
        );
    }
}
