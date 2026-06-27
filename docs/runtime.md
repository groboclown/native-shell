# Runtime Design

The native shell has a specific model for representing the script actions.  A good understanding of the [high level design](design.md) helps before digging into this document.

## General Flow

During execution, the scheduler maintains a list of active job threads.  A job thread represents a sequential ordering of actions to take.  Each action can either spawn something (a job, another thread, an event) or wait for something (a job, another thread, an event), or abort the script, or terminate the thread.

The jobs run asynchronously of each other, and the job thread only pauses for a job if the job action requires it.

Each job has a unique execution context, meaning that it can only run serially with itself.  If something spawned a job to run, then asked to spawn the same unique job again before it completed, then the job will not start.  In some cases, a job may describe itself as not rerunnable, in which case even if it has finished running, it will not spawn again.

For actions that wait on a job or job thread, the action can react based on how the waited-on completed, such as aborting the script if the job finished with an error.

## Interaction With a Script

In general, a script generates two groups of threads:

* A logical sequence of behaviors - run command "foo", then run command "bar"; if "foo" fails, then exit the script with its error.
* A pipe-connected group of executions, or "chain".  The Bash command `echo foo | tee out.txt` represents running two commands, `echo` and `tee`, but they run inter-connected such that the output of `echo` pipes into the input of `tee`.  These two commands must launch together so their file descriptors mix correctly.  This usually follows the pattern of:
  * Job 0: construct the interconnected pipes in a shared structure.
  * Job 1-n: run each execution and monitor its progress.
  * Step 0: spawn job 0
  * Step 1: wait for job 0
  * Step 2-n+1: spawn job 1-n
  * Step n+3: wait for all spawned jobs (1-n).

A script specifies 1 or more initial threads to start running.  When these threads stop running, then the script exits, regardless of whether the system still has other threads or jobs still running.

## Events

During execution, the system may generate "events".  These may come from the OS, like the end-user pressed `ctrl-c`, or they may come from within the program.  Jobs can act on the events to alter their behavior, as well as generate their own events.

To use the `ctrl-c` example, the main shell job may add in OS signal hooks, which, when received, will send out an "abort" event.  All jobs should listen to that "abort" event, and, when received, force the job to stop.

In another example, the script setup may include a logging job that listens to "warning" and "error" events.  When it receives any of these, it outputs text to stderr with the message.

## Jobs

The system considers each job as an individual unit of work.  It cannot run parallel with itself.  If a thread requests a job to run ("spawn") while it's already running, the spawn request will not do anything.  Some jobs can mark themselves as run once, so that if a job as already completed running, a second request to spawn it will also do nothing.  A "wait for job" request first looks at the current state of the job - if it isn't running, either because it has never started or it has already stopped, then the "wait for job" does not wait.

Only when a job changes to running does it receive events.
