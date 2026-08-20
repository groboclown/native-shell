# Design of the Shell

The shell contains these primary parts:

* The scripting language: What the script authors care about.  This design document covers this only cursory, as it focuses on the rest of the system.
* [The low-level source (LLS)](#low-level-source-lls): the internal representation of the script.  It allows the language to develop semi-independently of the compiler and engine.
* [The builder](#builder): turns the AST into native code.  Sometimes incorrectly referred to as a "compiler", as this leads directly to compiling the native code into an executable program.
* [The job scheduler](#job-scheduler): manages the jobs described by the builder.
* [Modules](#modules): supply the equivalent of command-line tools and bits of shell behavior, like file redirection.
* [The integration code](#integration-code): the code constructed by the builder.
* [Utility libraries](#utility-libraries): pre-built helper code to make the constructed code reliant on a solid foundation of well tested code.


## Low Level Source (LLS)

The LLS allows script language designers to construct a format that best suits script authors, without the rest of the system needing to change.  This LLS describes the capabilities supported by the system such that it requires no additional parsing.

The LLS allows for:

* Configuring a set of named "jobs".  These jobs work as a configured version of a module, and represent a script's command execution.  For example, the command `cp -R a b` would use the `cp` module with `recursive: true` and `source: a` and `target: b`.
* Defining a set of named "events".  Named events allow for triggering behavior when actions such as SIGINT happen within the script.  These may come from external sources (like OS signals) or from internal modules.  The script can associate an event listener to a node to allow for certain behaviors.
* Defining how jobs send streams of data between each other and the shell itself.
* Describes "steps" that run an action.  Each step performs one of a small set of actions, such as starting a job or sending an event.  It can wait for a job to finish, then run a behavior based on how that job finished.
* Defining lists of actions to take, or "threads".  Threads execute ordered steps.
* Setting parameters to jobs by referencing the state of other jobs.  The LLS defines explicit operations allowed in the parameter generation.  It fully defines the operations, without leaving any room for post LLS parsing.
* Denoting the source for every part of the LLS.  This allows for thorough runtime debugging and post-compilation tracing, if needed.


## Builder

The builder takes the LLS and constructs source code that allows for direct implementation with as little need for runtime interpreter execution as possible.  It uses the predefined [modules](#modules) and other library code and generates glue code.

The modules tell the builder how they work, so the builder can construct the dynamic code to work with them.

The builder creates all the source files necessary to build the script into an executable.

**The first version of the builder is expected to just lay down files, and lets the user compile it.  Later versions will have the builder perform these steps in ways easier and easier for the end user.**


## Job Scheduler

While the LLS defines nodes of modules with execution parameters and event handling, the job scheduler handles executing the configured threads and jobs.  The [runtime](runtime.md) document describes this in detail.


## Modules

Modules define functionality that takes the place of what a scripting language provides (such as if statements or executable running or file redirection).

Modules must bridge between running within the compiled code, and providing information so that the builder can create source code to interact with them.  As such, they provide meta-data in code, and must follow strict coding guidelines to allow the builder to make assumptions about how to call it.

Some modules may also need to allow for external events to stop the execution early, so that it can cleanly stop.

**For now, see the [`meta.rs`](../src/shell_lib/compile/meta.rs) source for details on what's expected from modules.**


## Commands

A script defines one or more "commands".  These allow the user to select an alternative set of behaviors.  The commands at similar to a module, but have interactions with the OS - specifically, it hosts the `stdin`, `stdout`, and `stderr` streams (or additional ones as necessary).  It should also handle configuring OS handle registration.


## Integration Code

The [builder](#builder) generates integration code to act as the compiled script.

The integration code currently splits the code into three main parts:

* the `main.rs` file.  This runs the whole program.
* the `runtime.rs` file.  Contains the shared runtime parameters, so that the jobs can correctly populate runtime parameters from other node data.
* one file per job sequence.  Contains the job definitions for each sequence.


## Utility Libraries

The utility libraries help make the integration code smaller by putting shared logic into places easier to test.
