use fission::prelude::*;

#[derive(Clone)]
pub struct GeneratedWorkspaceApp;

impl From<GeneratedWorkspaceApp> for Widget {
    fn from(_app: GeneratedWorkspaceApp) -> Self {
        Container::new(Text::new("Generated workspace app").size(30.0))
            .padding_all(24.0)
            .into()
    }
}
