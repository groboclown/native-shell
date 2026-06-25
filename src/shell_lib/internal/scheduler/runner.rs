//SPDX:MIT

use std::collections::HashMap;
use std::sync::{Arc, mpsc};
use std::thread;

use super::old_state::GlobalState;
use crate::shell_lib::structure::event::{EventPayload, EventRef};
use crate::shell_lib::structure::job::{
    ExitBehavior, JobDescription, JobRef, JobRunnerContext, JobThreadRef, OnExitBehavior,
    ScheduleStep, ScriptExit,
};

/// Schedules jobs and runs the job threads.
/// This only runs jobs in separate OS threads; the "job thread" handling happens by stepping through all
/// the active threads at once, one step at a time.  It includes a job state change poll mechanism, too.
pub fn run_threads(
    main_threads: &[JobThreadRef],
    state: &mut GlobalState,
) -> Result<ScriptExit, String> {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tx);

    let mut spawned_jobs = Vec::new();
    let mut t_steps = Vec::new();
    let mut active_main_count = 0;
    for t_id in main_threads {
        match state.start_thread(*t_id)? {
            None => return Err(format!("job thread {} already running", t_id)),
            Some(step) => {
                t_steps.push(StepState::new(*t_id, step, true));
                active_main_count += 1;
            }
        }
    }

    while active_main_count > 0 {
        // Pull in all pending job completions and events without waiting.

        //  pending_events: a map of event ref -> list of payloads.
        let mut pending_events = HashMapVec::new();
        loop {
            match rx.try_recv() {
                Ok(v) => match v {
                    JobEvent::JobDone(j_id, j_exit) => {
                        state.job_stopped(j_id, j_exit)?;
                    }
                    JobEvent::Event(e_id, e_data) => {
                        pending_events.insert(e_id, e_data);
                    }
                },
                Err(mpsc::TryRecvError::Empty) => {
                    break;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err("event queue unexpectedly closed".to_string());
                }
            }
        }

        // Perform all thread steps, keeping track of what's waiting on other things.
        // TODO If this was really good, it'd have a thread wait DAG to discover deadlocks.

        let mut waiting_steps = Vec::new();
        let mut next_steps = Vec::new();

        for mut step in t_steps.drain(0..t_steps.len()) {
            // For each thread, handle its current step.
            // If the thread needs to wait on something happening, then it goes in the wait list.
            // If the thread can advance to another step, it goes in the next steps list.

            match step.current() {
                ScheduleStep::SendEvent(event_ref, event_payload) => {
                    // Add the pending event.
                    pending_events.insert(*event_ref, event_payload.clone());
                    // Advance the step.
                    next_steps.push((step, 1));
                }
                ScheduleStep::WaitForEvent(event_ref, _) => {
                    // Event waiting is done after the loop, so that a thread that sends an event
                    // in the same loop can be properly captured.
                    waiting_steps.push(step);
                }
                ScheduleStep::SpawnJob(job_ref) => {
                    if let Some(job_desc) = state.start_job(*job_ref)? {
                        // Job is now marked as started, so we need to be careful with errors.
                        match run_job(*job_ref, job_desc, tx) {
                            Ok(handle) => {
                                spawned_jobs.push(handle);
                            }
                            Err(e) => {
                                state.job_stopped(
                                    *job_ref,
                                    ScriptExit {
                                        code: 1,
                                        message: Some(e),
                                    },
                                )?;
                            }
                        }
                    }
                    // Advance the step.
                    next_steps.push((step, 1));
                }
                ScheduleStep::WaitForJob(job_id, _) => {
                    // Add to the wait list for a unified check.
                    waiting_steps.push(step);
                }
                ScheduleStep::SpawnJobThread(thread_id) => {
                    match state.start_thread(*thread_id)? {
                        Some(s) => {
                            // Add a new thread.
                            next_steps.push((StepState::new(*thread_id, s, false), 0))
                        }
                        None => (),
                    }
                    // Advance the thread.
                    next_steps.push((step, 1));
                }
                ScheduleStep::WaitForJobThread(_, _) => {
                    // Add to the wait list for a unified check.
                    waiting_steps.push(step);
                }
                ScheduleStep::WaitForAll(items, items1, exit_behavior) => {
                    // Add to the wait list for a unified check.
                    waiting_steps.push(step);
                }
            }
        }

        if active_main_count < 0 {
            break;
        }

        // Loop over the waits, advancing if wait is over.
        // Note that though this could complete the thread wait
        // (A waits for thread B, then we discover thread B advances and is at end),
        // but the later loops will solve it.
        // Each thread that is not at an end is added into t_steps for the next loop,
        // and, if the step.is_main is true, increment the active_main_count.
        active_main_count = 0;
        t_steps.clear();

        todo!()
    }

    // Send out an abort event.
    // Wait on all job threads to end, with a maximum time limit.

    todo!()
}

enum JobEvent {
    JobDone(JobRef, ScriptExit),
    Event(EventRef, EventPayload),
}

fn run_job(
    id: JobRef,
    job: Arc<JobDescription>,
    tx: Arc<mpsc::Sender<JobEvent>>,
) -> Result<thread::JoinHandle<Result<(), String>>, String> {
    let (comm_tx, comm_rx) = mpsc::channel();
    thread::Builder::new()
        .name(format!("job-{}", id))
        .spawn(move || {
            let res = job.runner.run(Box::new(EventSender { tx: tx.clone() }));
            tx.send(JobEvent::JobDone(id, res)).map_err(|e| {
                format!(
                    "failed inter-process communication on job {} end: {}",
                    id, e
                )
            })
        })
        .map_err(|e| format!("failed spawning job {}: {}", job.source.name, e))
}

struct EventSender {
    tx: Arc<mpsc::Sender<JobEvent>>,
}

impl JobRunnerContext for EventSender {
    fn send_event(&self, event_ref: EventRef, payload: EventPayload) -> Result<(), ScriptExit> {
        self.tx
            .send(JobEvent::Event(event_ref, payload))
            .map_err(|e| ScriptExit {
                code: 1,
                message: Some(format!("failed sending event: {}", e)),
            })
    }
}

struct StepState {
    pub id: JobThreadRef,
    pub is_main: bool,
    step: ScheduleStep,
    wait: HashMap<JobOrThread, Option<ScriptExit>>,
}

#[derive(Clone, Debug, PartialEq)]
enum JobOrThread {
    Job(JobRef),
    Thread(JobThreadRef),
}

impl StepState {
    pub fn new(id: JobThreadRef, step: ScheduleStep, is_main: bool) -> Self {
        let mut ret = Self {
            id,
            is_main,
            step: step.clone(),
            wait: HashMap::new(),
        };
        ret.set_step(step);
        ret
    }

    pub fn current(&mut self) -> &ScheduleStep {
        &self.step
    }

    pub fn set_step(&mut self, step: ScheduleStep) {
        if let ScheduleStep::WaitForAll(jobs, threads, _) = &step {
            self.wait.clear();
            for job in jobs {
                self.wait.insert(JobOrThread::Job(*job), None);
            }
            for t in threads {
                self.thread_wait.insert(JobOrThread::Thread(*t), None);
            }
        }
        self.step = step;
    }

    pub fn test_wait_stopped(
        &mut self,
        jt: JobOrThread,
        exit: Option<ScriptExit>,
    ) -> Option<(ScriptExit, ExitBehavior)> {
        match exit {
            // Did not actually stop.
            None => None,
            Some(exit) => match jt {
                JobOrThread::Thread(t) => self.thread_exit(t, exit),
                JobOrThread::Job(j) => self.job_exit(j, exit),
            },
        }
    }

    fn job_exit(&mut self, job: JobRef, exit: ScriptExit) -> Option<(ScriptExit, ExitBehavior)> {
        match self.step {
            ScheduleStep::WaitForJob(j, e) => {
                if j == job {
                    Some((exit, e))
                } else {
                    None
                }
            }
            ScheduleStep::WaitForAll(_, _, e) => match self.wait.get_mut(&JobOrThread::Job(job)) {
                Some(v) => {
                    v.insert(exit);
                    self.assemble_wait_state().map(|s| (s, e))
                }
                None => None,
            },
            _ => None,
        }
    }

    fn thread_exit(
        &mut self,
        t_ref: JobThreadRef,
        exit: ScriptExit,
    ) -> Option<(ScriptExit, ExitBehavior)> {
        match self.step {
            ScheduleStep::WaitForJobThread(t, e) => {
                if t == t_ref {
                    Some((exit, e))
                } else {
                    None
                }
            }
            ScheduleStep::WaitForAll(_, _, e) => {
                match self.wait.get_mut(&JobOrThread::Thread(t_ref)) {
                    Some(v) => {
                        v.insert(exit);
                        self.assemble_wait_state().map(|s| (s, e))
                    }
                    None => None,
                }
            }
            _ => None,
        }
    }

    fn assemble_wait_state(&self) -> Option<ScriptExit> {
        let mut errs = 0;
        let mut msg = String::new();
        for e in self.wait.values() {
            match e {
                // Early exit; at least one waiting-on isn't ready.
                None => return None,
                Some(e) => {
                    if e.code != 0 {
                        errs += 1;
                    }
                    if let Some(m) = e.message {
                        if !msg.is_empty() {
                            msg.push('\n');
                        }
                        msg.push_str(m.as_str());
                    }
                }
            }
        }
        if msg.is_empty() {
            Some(ScriptExit {
                code: errs,
                message: None,
            })
        } else {
            Some(ScriptExit {
                code: errs,
                message: Some(msg),
            })
        }
    }
}
