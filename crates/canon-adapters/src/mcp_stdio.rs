#[derive(Debug, Default)]
pub struct McpStdioAdapter;

impl McpStdioAdapter {
    pub fn runtime_supported(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::McpStdioAdapter;

    #[test]
    fn runtime_support_is_disabled() {
        let adapter = McpStdioAdapter;

        assert!(!adapter.runtime_supported());
    }
}
