use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result, serde_json, serde_json::json, settings::LspSettings};

struct TyLspExtension;

impl zed::Extension for TyLspExtension {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match worktree.which("ty") {
            Some(path) => Ok(zed::Command {
                command: path,
                args: vec!["server".into()],
                env: worktree.shell_env(),
            }),
            None => Err("Unable to find ty executable".into()),
        }
    }
    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        let settings = json!({
            "ty": settings
        });

        Ok(Some(settings))
    }
}

zed::register_extension!(TyLspExtension);
