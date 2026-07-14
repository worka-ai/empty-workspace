pub mod actions;
pub mod app;
pub mod broker;
pub mod components;
pub mod data;
pub mod screens;
pub mod state;

use app::GeneratedWorkspaceApp;
use state::GeneratedWorkspaceState;
use worka::app::{
    WorkaApp, WorkaStaticArtifactPolicy, WorkaStaticPluginCapabilities,
};
use worka::export_worka_app;

mod personaldb_schema {
    include!(concat!(env!("OUT_DIR"), "/workspace_personaldb_schema.rs"));
}

export_worka_app! {
    WorkaApp::<GeneratedWorkspaceState, _>::new(GeneratedWorkspaceApp)
        .with_title("Generated Workspace App")
        .with_async(worka::broker::fission::register_broker_jobs)
        .with_capabilities(WorkaStaticPluginCapabilities::all())
        .with_personaldb_schema(personaldb_schema::workspace_personaldb_schema())
        .with_artifact_policy(WorkaStaticArtifactPolicy::new(true, true, 10_485_760, &[]))
}
