use zed_extension_api::{self as zed, Result};

struct MyExtension {
    // ... state
}

impl zed::Extension for MyExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Err("Hello World! I am not implemented".into())
    }
}

zed::register_extension!(MyExtension);