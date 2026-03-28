#[derive(Debug, Default)]
pub struct McpStdioAdapter;

impl McpStdioAdapter {
    pub fn runtime_supported(&self) -> bool {
        false
    }
}
