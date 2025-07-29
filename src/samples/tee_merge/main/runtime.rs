//! Shared runtime state, used by the job sequences.
use std::sync::Arc;

use crate::shell_lib::modules::{cat, echo, merge, shell, tee};
use crate::shell_lib::helpers;

/// All the nodes described by the AST.
pub struct Nodes {
    pub main: shell::ShellModule,
    pub data_file_1: cat::CatModule,
    pub data_file_2: cat::CatModule,
    pub data_text_1: echo::EchoModule,
    pub data_text_2: echo::EchoModule,
    pub tee1: tee::TeeModule,
    pub merge1: merge::MergeModule,
    pub merge2: merge::MergeModule,
    pub merge3: merge::MergeModule,
}

/// All the runtime parameters described by the AST.
pub struct RuntimeParams {
    // main: shell defines these as None
    pub data_file_1: cat::CatModuleRuntimeParams,
    pub data_file_2: cat::CatModuleRuntimeParams,
    pub data_text_1: echo::EchoModuleRuntimeParams,
    pub data_text_2: echo::EchoModuleRuntimeParams,
    // tee1: tee defines these as None
    pub merge1: merge::MergeModuleRuntimeParams,
    pub merge2: merge::MergeModuleRuntimeParams,
    pub merge3: merge::MergeModuleRuntimeParams,
}

#[derive(Clone)]
pub struct Runtime {
    pub nodes: Arc<Nodes>,
    pub params: helpers::state_guard::StateGuard<RuntimeParams>,
}
