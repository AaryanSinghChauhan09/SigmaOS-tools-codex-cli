/// Codex CLI command generation request/response model for SigmaOS terminal.

#[derive(Debug, Clone)]
pub struct CodexRequest {
    pub natural_language: String,
    pub shell: ShellType,
    pub cwd: String,
    pub sandbox: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

impl CodexRequest {
    pub fn new(nl: &str, shell: ShellType, cwd: &str) -> Self {
        Self {
            natural_language: nl.into(),
            shell,
            cwd: cwd.into(),
            sandbox: true,
        }
    }

    pub fn system_context(&self) -> String {
        format!(
            "Shell: {:?}. CWD: {}. Sandbox: {}",
            self.shell, self.cwd, self.sandbox
        )
    }
}

#[derive(Debug, Clone)]
pub struct CodexResponse {
    pub command: String,
    pub explanation: String,
    pub safe: bool,
}

impl CodexResponse {
    pub fn new(command: &str, explanation: &str, safe: bool) -> Self {
        Self {
            command: command.into(),
            explanation: explanation.into(),
            safe,
        }
    }

    pub fn is_destructive(&self) -> bool {
        ["rm -rf", "mkfs", "dd if=", "> /dev/"]
            .iter()
            .any(|pat| self.command.contains(pat))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_context_contains_shell_aaa() {
        let r = CodexRequest::new("list files", ShellType::Bash, "/home");
        assert!(r.system_context().contains("Bash"));
    }

    #[test]
    fn test_safe_command_not_destructive_aaa() {
        let r = CodexResponse::new("ls -la", "list files", true);
        assert!(!r.is_destructive());
    }

    #[test]
    fn test_rm_rf_is_destructive_aaa() {
        let r = CodexResponse::new("rm -rf /tmp/old", "delete old temp", false);
        assert!(r.is_destructive());
    }

    #[test]
    fn test_codex_request_sandbox_default_true_aaa() {
        let r = CodexRequest::new("show disk usage", ShellType::Zsh, "/");
        assert!(r.sandbox);
    }
}
