use serde::Serialize;

pub fn render_yaml<T: Serialize>(value: &T) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(value)
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::render_yaml;

    #[derive(Serialize)]
    struct Payload {
        name: &'static str,
        count: u32,
    }

    #[test]
    fn render_yaml_serializes_values() {
        let rendered = render_yaml(&Payload { name: "canon", count: 2 })
            .expect("yaml rendering should succeed");

        assert!(rendered.contains("name: canon"));
        assert!(rendered.contains("count: 2"));
    }
}
