use serde::Serialize;

pub fn render_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::render_json;

    #[derive(Serialize)]
    struct Payload {
        name: &'static str,
        count: u32,
    }

    #[test]
    fn render_json_pretty_prints_serializable_values() {
        let rendered = render_json(&Payload { name: "canon", count: 2 })
            .expect("json rendering should succeed");

        assert!(rendered.contains("\n  \"name\": \"canon\","));
        assert!(rendered.contains("\n  \"count\": 2"));
    }
}
