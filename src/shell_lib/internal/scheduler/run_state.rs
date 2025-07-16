//! Maintain sequence and job state as they execute.

use std::{ops::DerefMut, sync::{mpsc, Arc, RwLock}, thread};

use crate::shell_lib::compile::job;

pub enum JobRunState {
    /// The job has not yet started running.
    NeverStarted,

    /// The node is currently running.
    Running,

    /// The node has stopped execution.
    Stopped,

    /// The node has been aborted.
    Aborted,
}

/// Information about the job's running state.
pub struct JobState {
    pub state: JobRunState,

    /// The most recent exit code of the node.
    /// If the job was aborted, this will reflect the aborted exit code.
    pub exit_code: Option<job::ExitCode>,

    pub desc: job::JobDescription,

    /// Listeners for the job completion.
    /// Will be cleared when the job completes or aborts.
    completion: Vec<(job::JobSequenceRef, mpsc::Sender<(job::JobSequenceRef, job::ExitCode)>)>,
}

#[derive(Clone, Debug)]
pub enum JobSequenceRunState {
    /// The job sequence has not yet started running.
    NeverStarted,

    /// The job sequence is currently running.
    Running,

    /// The job sequence has stopped execution.
    Stopped,
}

pub struct JobSequenceState {
    pub state: JobSequenceRunState,

    /// The most recent exit code of the node.
    pub exit_code: Option<job::ExitCode>,

    active_action_index: usize,

    /// Listeners for the job sequence.
    /// Will be cleared when the job completes or aborts.
    completion: Vec<mpsc::Sender<job::ExitCode>>,

    // Rather than embedding the job sequence description object,
    // this copies it so it can share the steps without copying it.

    /// The name of the job sequence; for debugging.
    pub name: String,

    /// The source code or script associated with the job sequence; for debugging.
    pub source: String,

    /// The list of steps in the job sequence.
    steps: Arc<Vec<job::ScheduleStep>>,
}

struct GlobalJobState {
    active_sequences: usize,
    aborted: bool,
    completion_listeners: Vec<mpsc::Sender<Vec<Option<job::ExitCode>>>>,
}

/// JobExecutionState stores the underlying state of the job execution.
/// It's intended to be shared, as the raw data is read-only.  However, the
/// data within the vectors are wrapped in a lock, as the whole
/// state object must be managed as a whole for correct thread safety.
/// 
/// Outside functions can only directly access the sequences.  As such,
/// the channels on the sequences are owned by the outside functions,
/// and the channels on the jobs are owned by the sequences.
struct JobExecutionState {
    jobs: Vec<RwLock<JobState>>,
    sequences: Vec<RwLock<JobSequenceState>>,
    event_groups: super::event_group::EventBus,
    global: RwLock<GlobalJobState>,
}

impl JobExecutionState {
    fn new(
        jobs: Vec<job::JobDescription>,
        sequences: Vec<job::JobSequenceDescription>,
        event_groups: Vec<job::EventGroup>,
    ) -> Self {
        let jobs = jobs.into_iter()
            .map(|desc| RwLock::new(JobState {
                state: JobRunState::NeverStarted,
                exit_code: None,
                desc,
                completion: Vec::new(),
            }))
            .collect();
        let sequences = sequences.into_iter()
            .map(|desc| RwLock::new(JobSequenceState {
                state: JobSequenceRunState::NeverStarted,
                exit_code: None,
                active_action_index: 0,
                name: desc.name,
                source: desc.source,
                steps: Arc::new(desc.steps),
                completion: Vec::new(),
            }))
            .collect();
        Self {
            jobs,
            sequences,
            event_groups: super::event_group::EventBus::new(event_groups),
            global: RwLock::new(GlobalJobState {
                active_sequences: 0,
                aborted: false,
                completion_listeners: Vec::new(),
            }),
        }
    }

    fn get_job_state(&self, job_ref: job::JobRef) -> &RwLock<JobState> {
        self.jobs.get(job_ref).expect("invalid job reference")
    }

    /// Get the state of the job sequence.
    /// Note: to use the lock on this sequence requires gaining a lock on the global state first.
    fn get_sequence_state(&self, seq_ref: job::JobSequenceRef) -> &RwLock<JobSequenceState> {
        self.sequences.get(seq_ref).expect("invalid sequence reference")
    }

    /// Receive the job's exit code through a channel.
    /// If the job has not run, then this will return None.
    /// If the job has stopped, then this will return the exit code immediately.
    /// If the job aborts, then this will receive an exit code of MIN, but testing for that
    /// specific value is not a guarantee of the job being aborted.
    fn job_exit_channel(&self, parent: job::JobSequenceRef, job_ref: job::JobRef) -> Option<mpsc::Receiver<(job::JobSequenceRef, job::ExitCode)>> {
        let mut state = self.get_job_state(job_ref)
            .write().expect("failed locking job state");
        match state.state {
            JobRunState::Running => {
                let (tx, rx) = mpsc::channel();
                state.completion.push((parent, tx));
                Some(rx)
            }
            JobRunState::Stopped => {
                // If the job has stopped, we can return the exit code immediately.
                let (tx, rx) = mpsc::channel();
                let code = match state.exit_code {
                    Some(code) => code,
                    None => job::ExitCode::MIN, // Technically a bad state
                };
                tx.send((parent, code)).expect("failed sending exit code");
                Some(rx)
            }
            JobRunState::Aborted => {
                // If the job has been aborted, we can return the exit code immediately.
                let (tx, rx) = mpsc::channel();
                tx.send((parent, job::ExitCode::MIN)).expect("failed sending exit code");
                Some(rx)
            }
            JobRunState::NeverStarted => {
                // If the job has never run, then there's nothing to monitor.
                None
            }
        }
    }

    fn job_sequence_exit_channel(&self, seq_ref: job::JobSequenceRef) -> Option<mpsc::Receiver<job::ExitCode>> {
        // Must obtain the global lock first, as the sequence state is dependent on it.
        let _unused = self.global
            .read().expect("failed locking global state");
        let mut state = self.get_sequence_state(seq_ref)
            .write().expect("failed locking job state");
        match state.state {
            JobSequenceRunState::Running => {
                let (tx, rx) = mpsc::channel();
                state.completion.push(tx);
                Some(rx)
            }
            JobSequenceRunState::Stopped => {
                // If the sequence has stopped, we can return the exit code immediately.
                let (tx, rx) = mpsc::channel();
                let code = match state.exit_code {
                    Some(code) => code,
                    None => job::ExitCode::MIN, // Technically a bad state
                };
                tx.send(code).expect("failed sending exit code");
                Some(rx)
            }
            JobSequenceRunState::NeverStarted => {
                // If the sequence has never run, then there's nothing to monitor.
                None
            }
        }
    }

    fn listener_map<F>(&self, event_ref: job::EventRef, f: F) -> Vec<String>
    where
        F: Fn(job::JobRef, &job::EventHandler) -> Result<(), String>,
    {
        self.event_groups.listener_map(event_ref, f)
    }

    fn add_global_completion_listener(&self, tx: mpsc::Sender<Vec<Option<job::ExitCode>>>) {
        self.global
            .write().expect("failed locking global completion listeners")
            .completion_listeners.push(tx);
    }
}


pub struct Scheduler {
    state: Arc<JobExecutionState>,
}

impl Scheduler {
    pub fn new(
        jobs: Vec<job::JobDescription>,
        sequences: Vec<job::JobSequenceDescription>,
        event_groups: Vec<job::EventGroup>,
    ) -> Self {
        let state = JobExecutionState::new(jobs, sequences, event_groups);
        Self {state: Arc::new(state)}
    }

    pub fn context(&self, job_ref: job::JobRef) -> SchedulerContext {
        SchedulerContext {
            job_ref,
            state: self.state.clone(),
        }
    }

    pub fn add_global_completion_listener(&self, tx: mpsc::Sender<Vec<Option<job::ExitCode>>>) {
        self.state.add_global_completion_listener(tx);
    }

    pub fn start_job_sequence_named(&self, name: &str) -> Result<(), String> {
        self.context(0).schedule_sequence(self.get_sequence_id_named(name)
            .ok_or_else(|| format!("No job sequence named {}", name))?)
    }

    fn get_sequence_id_named(&self, name: &str) -> Option<job::JobSequenceRef> {
        // Find the sequence by name.
        for (i, seq) in self.state.sequences.iter().enumerate() {
            // Requires global lock to read the sequence state.
            let _unused = self.state.global
                .read().expect("failed locking global state");
            let seq = seq.read().expect("failed locking sequence state");
            if seq.name == name {
                return Some(i);
            }
        }
        None
    }
}

pub struct SchedulerContext {
    job_ref: job::JobRef,
    state: Arc<JobExecutionState>,
}

impl job::JobRunnerContext for SchedulerContext {
    fn send_event(&self, event_ref: job::EventRef, payload: job::EventPayload) -> Result<(), String> {
        self.handle_event(event_ref, payload)
    }
}

impl job::JobScheduler for SchedulerContext {
    fn start_job_sequence(&self, seq_ref: job::JobSequenceRef) -> Result<(), String> {
        self.schedule_sequence(seq_ref)
    }

    /// Checks if the job sequence has ever or is currently running.
    fn get_job_sequence_state(&self, seq_ref: job::JobSequenceRef) -> job::JobSequenceRunState {
        // Must obtain the global lock first, as the sequence state is dependent on it.
        let _unused = self.state.global
            .read().expect("failed locking global state");
        let state = self.state.get_sequence_state(seq_ref)
            .read().expect("failed locking job sequence state");
        match state.state {
            JobSequenceRunState::Running => job::JobSequenceRunState::Running,
            JobSequenceRunState::Stopped => job::JobSequenceRunState::Finished(state.exit_code.expect("bad state: stopped sequence has no exit code")),
            JobSequenceRunState::NeverStarted => job::JobSequenceRunState::NeverStarted,
        }
    }

    /// Wait for a job sequence to finish executing.
    /// Returns immediately if the sequence has not started.
    fn wait_for_job_sequence(&self, seq_ref: job::JobSequenceRef, timeout: Option<std::time::Duration>) -> Result<Option<job::ExitCode>, String> {
        let rx = self.state.job_sequence_exit_channel(seq_ref);
        match rx {
            Some(rx) => match timeout {
                Some(duration) => match rx.recv_timeout(duration) {
                    Ok(code) => Ok(Some(code)),
                    Err(_) => Err("Timeout waiting for job sequence".to_string()),
                }
                None => match rx.recv() {
                    Ok(code) => Ok(Some(code)),
                    Err(_) => Err("Failed receiving job sequence exit code".to_string()),
                }
            }
            None => Ok(None),
        }
    }

    /// Send a signal to stop all jobs in progress.
    fn abort(&self) {
        todo!()
    }

}

impl SchedulerContext {
    fn schedule_sequence(&self, seq_ref: job::JobSequenceRef) -> Result<(), String> {
        // Need to first capture the global lock before anything else,
        // and maintain it through the whole function.
        let mut global = self.state
            .global.write().expect("failed locking global state");
        if global.aborted {
            return Err("Cannot start job sequence after abort".to_string());
        }
        let mut state = self.state.get_sequence_state(seq_ref)
            .write().expect("failed locking sequence state");
        match state.state {
            JobSequenceRunState::Running =>
                // This condition acts like a no-op.
                Ok(()),
            JobSequenceRunState::NeverStarted | JobSequenceRunState::Stopped => {
                // Start the sequence.
                let state = state.deref_mut();
                state.state = JobSequenceRunState::Running;
                global.active_sequences += 1;
                self.spawn_sequence(seq_ref);
                Ok(())
            }
        }
    }

    fn handle_event(&self, event_ref: job::EventRef, payload: job::EventPayload) -> Result<(), String> {
        let errs = match payload {
            job::EventPayload::Message(msg) =>
                self.state.listener_map(event_ref, |job_ref, handler| {
                    if let job::EventHandler::Message(handler) = handler {
                        self.handle_message_listener(job_ref, msg.clone(), handler)
                    } else {
                        panic!("Attached signal handler to message event")
                    }
                }),
            job::EventPayload::Signal(code) =>
                self.state.listener_map(event_ref, |_, handler| {
                    if let job::EventHandler::Signal(signal_handler) = handler {
                        self.schedule_sequence(signal_handler.behavior_for(code))
                    } else {
                        panic!("Attached message handler to signal event")
                    }
                }),
        };
        if errs.len() > 0 {
            return Err(errs.join(", "));
        }
        Ok(())
    }

    fn handle_message_listener<'a>(&self, job_ref: job::JobRef, msg: String, handler: &'a Box<dyn job::MessageEventHandler + Sync + Send>) -> Result<(), String> {
        // Note: called directly in-thread during the sequence walk.
        // So this might block the thread.  The caller must take care to not block the thread for too long.
        let context: Box<dyn job::EventHandlerContext> = self.context(job_ref);
        handler.handle_message(msg, &context)
    }

    /// Wait for all the job sequences to complete.
    /// 
    /// This can include an abort causing the job sequences to stop prematurely.
    /// Each job sequence reference has its exit code in that index of the returned vector.
    fn wait_for_completion(&self, timeout: Option<std::time::Duration>) -> Result<Vec<Option<job::ExitCode>>, String> {
        let (tx, rx) = mpsc::channel();
        self.state.add_global_completion_listener(tx);

        // If no sequence has started, or they've all stopped, then this will trigger the end.
        self.send_global_completion();

        match timeout {
            Some(duration) => match rx.recv_timeout(duration) {
                Ok(codes) => Ok(codes),
                Err(_) => Err("Timeout waiting for job sequences".to_string()),
            }
            None => match rx.recv() {
                Ok(codes) => Ok(codes),
                Err(_) => Err("Failed receiving job sequence exit codes".to_string()),
            }
        }
    }
    
    /// Internal handler for running a job.
    /// Must be here, rather than in the state, as it needs access to the Arc.
    fn schedule_job(&self, parent: job::JobSequenceRef, job_ref: job::JobRef) -> Result<mpsc::Receiver<(job::JobSequenceRef, job::ExitCode)>, String> {
        let mut state = self.state.get_job_state(job_ref)
            .write().expect("failed locking job state");
        match state.state {
            JobRunState::Running => {
                // This condition acts like a no-op.
                let (tx, rx) = mpsc::channel();
                state.completion.push((parent, tx));
                Ok(rx)
            }
            JobRunState::Aborted =>
                // Aborted jobs can never be restarted.
                Err(format!("Job {} has been aborted", job_ref)),
            JobRunState::Stopped | JobRunState::NeverStarted => {
                let (tx, rx) = mpsc::channel();
                state.completion.push((parent, tx));
                state.state = JobRunState::Running;
                self.spawn_job(job_ref);
                Ok(rx)
            }
        }
    }

    /// Internal handler for spawning a job.
    /// The caller must ensure that the job is not already running and set the state to Running.
    fn spawn_job(&self, job_ref: job::JobRef) {
        let context = SchedulerContext {
            job_ref,
            state: self.state.clone(),
        };
        // TODO use a thread pool.
        // Need to split up the lock to avoid deadlocks.
        thread::Builder::new()
            .name(format!("job-{}", job_ref))
            .spawn(move || {
                context.run_job(job_ref);
            })
            .expect("failed spawning job thread");
    }

    /// Internal handler for running a job.
    /// Must be called only by the spawn_job function.
    fn run_job(&self, job_ref: job::JobRef) {
        let ret;
        {
            ret = self.state.get_job_state(job_ref)
                .read().expect("failed locking job state")
                .desc.runner.run(self.context(job_ref));
        }
        let mut state = self.state.get_job_state(job_ref)
            .write().expect("failed locking job state");
        match ret {
            Ok(code) => {
                // Job ran to completion.
                state.exit_code = Some(code);
                state.state = JobRunState::Stopped;
                // Notify all completion listeners.
                for (parent, tx) in state.completion.drain(..) {
                    // A closed back-end channel should not cause a panic.
                    // It generally means the listener timed out.
                    let _ = tx.send((parent, code));
                }
            }
            Err(e) => {
                // The job could not complete.
                state.state = JobRunState::Aborted;
                state.exit_code = None;
                // Notify all completion listeners with an aborted exit code.
                for (parent, tx) in state.completion.drain(..) {
                    // A closed back-end channel should not cause a panic.
                    // It generally means the listener timed out.
                    let _ = tx.send((parent, job::ExitCode::MIN));
                }
                // TODO log the error correctly.
                eprintln!("Job {} failed to run: {}", job_ref, e);
            }
        }
    }

    fn abort_job(&self, job_ref: job::JobRef) {
        let mut state = self.state.get_job_state(job_ref)
            .write().expect("failed locking job state");
        match state.state {
            JobRunState::Running => {
                // Abort the job.
                state.state = JobRunState::Aborted;
                state.exit_code = None;
                // Notify all completion listeners with an aborted exit code.
                for (parent, tx) in state.completion.drain(..) {
                    // A closed back-end channel should not cause a panic.
                    // It generally means the listener timed out.
                    let _ = tx.send((parent, job::ExitCode::MIN));
                }
            }

            // Any other state means the job does not need to be aborted.
            JobRunState::Stopped | JobRunState::Aborted | JobRunState::NeverStarted => (),
        }
    }

    /// Internal handler for spawning a sequence.
    /// The caller must ensure that the job is not already running and set the state to Running.
    fn spawn_sequence(&self, sequence_ref: job::JobSequenceRef) {
        let context = SchedulerContext {
            job_ref: sequence_ref,
            state: self.state.clone(),
        };
        // TODO investigate a single thread to monitor the sequences.
        // This may not be the right approach; running an alert's logic
        // currently happens at the time of the alert step, and that might lead
        // to a deadlock or potentially millions of alert steps which would
        // starve the listening pool.
        thread::Builder::new()
            .name(format!("sequence-{}", sequence_ref))
            .spawn(move || {
                context.run_sequence(sequence_ref);
            })
            .expect("failed spawning job sequence thread");
    }

    fn run_sequence(&self, sequence_ref: job::JobSequenceRef) {
        // For each step in the sequence, spawn the job and wait for it to complete.
        // This needs to carefully manage the locks to prevent deadlocks.
        let steps;
        {
            // Getting the sequence state requires the global lock.
            let _unused = self.state.global
                .read().expect("failed locking global state");
            steps = self.state.get_sequence_state(sequence_ref)
                .read().expect("failed locking sequence state")
                .steps.clone();
        }
        let mut final_code = job::ExitCode::MIN;
        let mut skip_step = false;
        let mut aborted = false;
        let mut index = 0;
        for step in steps.iter() {
            // Right now, just loop over the steps and run incrementally for a single sequence.
            // This might change and share a single thread for all sequences in the future.
            index += 1;

            {
                // Set the current state.
                // Getting the sequence state requires the global lock.
                let global = self.state.global
                    .read().expect("failed locking global state");
                if global.aborted {
                    // Something else aborted the script.  Do not change global abort state.
                    break;
                }
                let mut state = self.state.get_sequence_state(sequence_ref)
                    .write().expect("failed locking sequence state");
                state.active_action_index = index;
            }
            {
                let prev_skip_step = skip_step;
                skip_step = false;
                if prev_skip_step {
                    // Skip the step if the previous step requested it.
                    continue;
                }
            }

            match step {
                job::ScheduleStep::SendEvent(event_ref, payload) => {
                    // Send the event to the event group.
                    let res = self.handle_event(*event_ref, payload.clone());
                    // FIXME handle the error properly.
                    if let Err(e) = res {
                        eprintln!("Failed to send event {}: {}", event_ref, e);
                    }
                }
                job::ScheduleStep::SpawnJob(job_ref) => {
                    // Schedule the job to run.
                    let res = self.schedule_job(sequence_ref, *job_ref);
                    if res.is_err() {
                        // Currently only means that the job is aborted.
                        // So abort this sequence.
                        final_code = job::ExitCode::MIN;
                        break;
                    }
                }
                job::ScheduleStep::WaitForJob(job_ref, exit_behavior) => {
                    // Wait for the job to complete.
                    let mut job_code = job::ExitCode::MIN;
                    let res = self.state.job_exit_channel(sequence_ref, *job_ref);
                    let mut on_exit = exit_behavior.default_behavior.clone();
                    if let Some(rx) = res {
                        if let Ok((_, code)) = rx.recv() {
                            job_code = code;
                            on_exit = exit_behavior.behavior_for(code);
                        }
                    }
                    // Else the job was never started.
                    match on_exit {
                        job::OnExitBehavior::RunNext => {
                            // Continue to the next step.
                        }
                        job::OnExitBehavior::SkipNext => {
                            // Skip the next step.
                            skip_step = true;
                        }
                        job::OnExitBehavior::SkipAll => {
                            // Skip all remaining steps.
                            final_code = job_code;
                            break;
                        }
                        job::OnExitBehavior::AbortScript => {
                            // Abort the script execution.
                            aborted = true;
                            break;
                        }
                    }
                }
                job::ScheduleStep::SpawnJobSequence(seq_ref) => {
                    // Schedule the job sequence to run.
                    let _ = self.schedule_sequence(*seq_ref);
                }
                job::ScheduleStep::WaitForJobSequence(seq_ref, exit_behavior) => {
                    if *seq_ref == sequence_ref {
                        panic!("Cannot wait for the same sequence that is currently running");
                    }
                    // Wait for the job sequence to complete.
                    let mut on_exit = exit_behavior.default_behavior.clone();
                    let mut seq_code = job::ExitCode::MIN;
                    let rx = self.state.job_sequence_exit_channel(*seq_ref);
                    if let Some(rx) = rx {
                        if let Ok(code) = rx.recv() {
                            seq_code = code;
                            on_exit = exit_behavior.behavior_for(seq_code);
                        }
                        // Else the sequence quit before sending an exit code.
                        // Maybe due to a panic.
                    }
                    match on_exit {
                        job::OnExitBehavior::RunNext => {
                            // Continue to the next step.
                        }
                        job::OnExitBehavior::SkipNext => {
                            // Skip the next step.
                            skip_step = true;
                        }
                        job::OnExitBehavior::SkipAll => {
                            // Skip all remaining steps.
                            final_code = seq_code;
                            break;
                        }
                        job::OnExitBehavior::AbortScript => {
                            // Abort the script execution.
                            aborted = true;
                            break;
                        }
                    }
                }
                job::ScheduleStep::Abort(msg) => {
                    // TODO log the message.
                    aborted = true;
                    break;
                }
            }
        }

        // Finalize the sequence state.
        {
            let mut global = self.state.global
                .write().expect("failed locking global state");
            global.aborted |= aborted;
            global.active_sequences -= 1;
            let mut state = self.state.get_sequence_state(sequence_ref)
                .write().expect("failed locking sequence state");
            state.state = JobSequenceRunState::Stopped;
            state.exit_code = Some(final_code);

            // Notify all completion listeners.
            for tx in state.completion.drain(..) {
                let _ = tx.send(final_code);
            }

            if global.active_sequences == 0 {
                // Note: send global completion with the write lock still held.
                self.send_global_completion();
            }
        }
    }

    fn send_global_completion(&self) {
        // Notify all global completion listeners.
        let mut global = self.state.global
            .write().expect("failed locking global state");
        if global.active_sequences > 0 || global.completion_listeners.is_empty() {
            // Early exit.
        }

        let mut codes = Vec::with_capacity(self.state.sequences.len());
        for seq in self.state.sequences.iter() {
            let seq_state = seq.read().expect("failed locking sequence state");
            codes.push(seq_state.exit_code);
        }
        for tx in global.completion_listeners.drain(..) {
            // A closed back-end channel should not cause a panic.
            // It generally means the listener timed out.
            let _ = tx.send(codes.clone());
        }
    }

    fn context(&self, job_ref: job::JobRef) -> Box<SchedulerContext> {
        Box::new(SchedulerContext {
            job_ref,
            state: self.state.clone(),
        })
    }
}

impl job::EventHandlerContext for SchedulerContext {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell_lib::compile::job::{self, JobScheduler};
    use std::time::Duration;

    struct DelayRunner {
        time: Duration,
        ret: job::ExitCode,
        ran: Arc<RwLock<usize>>,
    }
    impl job::JobRunner for DelayRunner {
        fn run(&self, _context: Box<dyn job::JobRunnerContext>) -> Result<job::ExitCode, String> {
            *self.ran.write().expect("failed to get delay lock") += 1;
            thread::sleep(self.time);
            Ok(self.ret)
        }
        fn abort(&self) -> Result<(), String> { Ok(()) }
    }

    #[test]
    fn test_scheduler_simple_sequence() {
        // Create a job that returns exit code 42
        let ran = Arc::new(RwLock::new(0));
        let job = job::JobDescription {
            name: "dummy".to_string(),
            source: "".to_string(),
            listen: None,
            runner: Box::new(DelayRunner{ran: ran.clone(), time: Duration::from_millis(0), ret: 42}),
        };
        // Sequence: spawn job 0, then wait for it, using SkipAll to propagate exit code
        let seq_desc = job::JobSequenceDescription {
            name: "seq".to_string(),
            source: "".to_string(),
            steps: vec![
                job::ScheduleStep::SpawnJob(0),
                job::ScheduleStep::WaitForJob(
                    0,
                    job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::SkipAll,
                    }
                ),
            ],
        };
        let scheduler = Scheduler::new(vec![job], vec![seq_desc], vec![]);
        // Before start: NeverStarted
        assert_eq!(scheduler.context(0).get_job_sequence_state(0), job::JobSequenceRunState::NeverStarted);
        // Start the sequence
        scheduler.context(0).start_job_sequence(0).unwrap();
        assert_eq!(scheduler.context(0).get_job_sequence_state(0), job::JobSequenceRunState::Running);
        // Wait with no timeout
        let code = scheduler.context(0).wait_for_job_sequence(0, None).unwrap();
        assert_eq!(code, Some(42));
        // After completion: Finished(42)
        assert_eq!(scheduler.context(0).get_job_sequence_state(0), job::JobSequenceRunState::Finished(42));
        assert_eq!(*ran.read().expect("failed to get dummy lock"), 1);
    }

    #[test]
    fn test_scheduler_wait_not_running() {
        // Sequence with no steps never starts, wait returns None
        let ran = Arc::new(RwLock::new(0));
        let job = job::JobDescription {
            name: "".to_string(),
            source: "".to_string(),
            listen: None,
            runner: Box::new(DelayRunner{ran: ran.clone(), time: Duration::from_millis(0), ret: 99}),
        };
        let seq_desc = job::JobSequenceDescription {
            name: "".to_string(),
            source: "".to_string(),
            steps: vec![],
        };
        let scheduler = Scheduler::new(vec![job], vec![seq_desc], vec![]);
        let result = scheduler.context(0).wait_for_job_sequence(0, Some(Duration::from_millis(1)));
        assert_eq!(result.unwrap(), None);
        assert_eq!(*ran.read().expect("failed to get dummy lock"), 0);
    }
    
    #[test]
    fn test_scheduler_wait_timeout() {
        let ran = Arc::new(RwLock::new(0));
        let job = job::JobDescription {
            name: "".to_string(),
            source: "".to_string(),
            listen: None,
            runner: Box::new(DelayRunner{ran: ran.clone(), time: Duration::from_secs(1), ret: 0}),
        };
        let seq_desc = job::JobSequenceDescription {
            name: "".to_string(),
            source: "".to_string(),
            steps: vec![
                job::ScheduleStep::SpawnJob(0),
                job::ScheduleStep::WaitForJob(
                    0,
                    job::ExitCodeBehavior {
                        never_started: job::OnExitBehavior::AbortScript,
                        exit_code_behaviors: vec![],
                        default_behavior: job::OnExitBehavior::RunNext,
                    }
                ),
            ],
        };
        let scheduler = Scheduler::new(vec![job], vec![seq_desc], vec![]);
        scheduler.context(0).start_job_sequence(0).unwrap();
        let result = scheduler.context(0).wait_for_job_sequence(0, Some(Duration::from_millis(1)));
        assert_eq!(result.unwrap_err(), "Timeout waiting for job sequence");
        // Should have triggered the job to run.
        assert_eq!(*ran.read().expect("failed to get dummy lock"), 1);
    }
}
