# Runtime Design

The native shell has a specific model for representing the script actions.  A good understanding of the [high level design](design.md) helps before digging into this document.

## General Flow

During execution, the scheduler maintains a list of active job threads.  A job thread represents a sequential ordering of actions to take.  Each action can either spawn something (a job, another thread, an event) or wait for something (a job, another thread, an event), or abort the script, or terminate the thread.

The jobs run asynchronously of each other, and the job thread only pauses for a job if the job action requires it.

Each job has a unique execution context, meaning that it can only run serially with itself.  If something spawned a job to run, then asked to spawn the same unique job again before it completed, then the job will not start.  In some cases, a job may describe itself as not rerunnable, in which case even if it has finished running, it will not spawn again.

For actions that wait on a job or job thread, the action can react based on how the waited-on completed, such as aborting the script if the job finished with an error.

## Interaction With a Script

In general, a script generates two groups of threads:

* A logical sequence of behaviors.
* A pipe-connected group of executions, or "chain".  The Bash command `echo foo | tee out.txt` represents running two commands, `echo` and `tee`, but they run inter-connected such that the output of `echo` pipes into the input of `tee`.  These two commands must launch together so their file descriptors mix correctly.  This usually follows the pattern of:
  * Job 0: construct the interconnected pipes in a shared structure.
  * Job 1-n: run each execution and monitor its progress.
  * Step 0: spawn job 0
  * Step 1: wait for job 0
  * Step 2-n+1: spawn job 1-n
  * Step ...: wait for each spawned job.
