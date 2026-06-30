//! Construct the threads as defined in the lls.yaml
//! Because this script is supposed to be "compiled", it
//! hard-codes the ThreadRef and JobRef.
//!
//! Jobs:
//!    j0x0 == 0
//!    j0x1 == 1
//!
//! Threads:
//!    default == 0
//!    jt1     == 1

use crate::shell_lib::structure;

pub fn create_threads() -> structure::thread::ThreadStore {
    structure::thread::ThreadStore::new_explicit(vec![
        structure::thread::ThreadDescription {
            // default == 0
            source: structure::source::Resource::new("default", "thread", "script.ns", 0, 0),
            steps: vec![
                structure::thread::ScheduleStep::SpawnThread(1), // jt1 == 1
                structure::thread::ScheduleStep::WaitForThread(
                    1, // jt1 == 1
                    structure::thread::ExitBehavior {
                        never_started: structure::thread::OnExitBehavior::AbortScript(None),
                        default_behavior: structure::thread::OnExitBehavior::AbortScript(None),
                        exit_code_behaviors: vec![structure::thread::ExitCodeRangeBehavior::new(
                            structure::thread::ExitCodeRange::Exact(0),
                            structure::thread::OnExitBehavior::JumpBy(1),
                        )],
                    },
                ),
            ],
        },
        structure::thread::ThreadDescription {
            // jt1 == 1
            source: structure::source::Resource::new("jt1", "thread", "script.ns", 0, 0),
            steps: vec![
                structure::thread::ScheduleStep::SpawnJob(0), // j0x0 == 0
                structure::thread::ScheduleStep::WaitForJob(
                    0, // j0x0 == 0
                    structure::thread::ExitBehavior {
                        never_started: structure::thread::OnExitBehavior::AbortScript(None),
                        default_behavior: structure::thread::OnExitBehavior::StopThread(None),
                        exit_code_behaviors: vec![structure::thread::ExitCodeRangeBehavior::new(
                            structure::thread::ExitCodeRange::Exact(0),
                            structure::thread::OnExitBehavior::JumpBy(1),
                        )],
                    },
                ),
                structure::thread::ScheduleStep::SpawnJob(1), // j0x1 == 1
                structure::thread::ScheduleStep::WaitForAll(
                    vec![1], // jobs: j0x1 == 1
                    vec![],  // threads
                    structure::thread::ExitBehavior {
                        never_started: structure::thread::OnExitBehavior::AbortScript(None),
                        default_behavior: structure::thread::OnExitBehavior::StopThread(None),
                        exit_code_behaviors: vec![structure::thread::ExitCodeRangeBehavior::new(
                            structure::thread::ExitCodeRange::Exact(0),
                            structure::thread::OnExitBehavior::JumpBy(1),
                        )],
                    },
                ),
            ],
        },
    ])
}
