use fission::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedWorkspaceState {
    pub title: String,
}

impl GlobalState for GeneratedWorkspaceState {}
