/// Stub adapter for Model Context Protocol (MCP) tool invocation via stdio.
///
/// Runtime support is not yet implemented; all operations are no-ops that
/// report themselves as unsupported.
#[derive(Debug, Default)]
pub struct McpStdioAdapter;

impl McpStdioAdapter {
    /// Returns `false` — MCP stdio support is not enabled in this build.
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
