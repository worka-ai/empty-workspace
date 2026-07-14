use anyhow::Result;
use app::app::GeneratedWorkspaceApp;
use app::state::GeneratedWorkspaceState;
use fission::prelude::*;

fn main() -> Result<()> {
    DesktopApp::<GeneratedWorkspaceState, _>::new(GeneratedWorkspaceApp)
        .with_title("Generated Workspace App")
        .with_async(worka::broker::fission::register_broker_jobs)
        .run()
}
