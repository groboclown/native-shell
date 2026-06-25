# Design of the Shell

The shell is broken into these primary parts:

* The scripting language: What the script authors care about.  This design document covers this only cursory, as it focuses on the rest of the system.
* [The abstract syntax tree (AST)](#abstract-syntax-tree-ast): the internal representation of the script.  It allows the language to develop semi-independently of the compiler and engine.
* [The builder](#builder): turns the AST into native code.  Sometimes incorrectly referred to as a "compiler", as this leads directly to compiling the native code into an executable program.
* [The job scheduler](#job-scheduler): manages the jobs described by the builder.
* [Modules](#modules): supply the equivalent of command-line tools and bits of shell behavior, like file redirection.
* [The integration code](#integration-code): the code constructed by the builder.
* [Utility libraries](#utility-libraries): pre-built helper code to make the constructed code reliant on a solid foundation of well tested code.


## Abstract Syntax Tree (AST)

The AST allows script language designers to construct a format that best suits script authors, without the rest of the system needing to change.  This AST describes the capabilities supported by the system such that it requires no additional parsing.

The AST allows for:

* Defining a set of named "nodes".  These nodes work as a configured version of a module, and represent a script's command execution.  For example, the command `cp -R a b` would use the `cp` module with `recursive: true` and `source: a` and `target: b`.
* Defining a set of named "events".  Named events allow for triggering behavior when actions such as SIGINT happen within the script.  These may come from external sources (like OS signals) or from internal modules.  The script can associate an event listener to a node to allow for certain behaviors.
* Defining how nodes send streams of data between each other and the shell itself.
* Defining lists of actions to take.  These can run when a node quits with certain exit codes, or on events.
* Setting parameters to nodes by referencing the state of other nodes.  The AST defines explicit operations allowed in the parameter generation.  It fully defines the operations, without leaving any room for post AST parsing.
* Denoting the source for every part of the AST.  This allows for thorough runtime debugging, if needed.


## Builder

The builder takes the AST and constructs source code that allows for direct implementation with as little need for runtime interpreter execution as possible.  It uses the predefined [modules](#modules) and other library code and generates glue code.

The modules tell the builder how they work, so the builder can construct the dynamic code to work with them.

The builder creates all the source files necessary to build the script into an executable.

**The first version of the builder is expected to just lay down files, and lets the user compile it.  Later versions will have the builder perform these steps in ways easier and easier for the end user.**


## Job Scheduler

While the AST defines nodes of modules with execution parameters and event handling, the job scheduler looks at all that behavior in a different way.  It instead manages a collection of job step sequences and jobs.  This runs at a different abstraction level than the AST, meaning some parts implicit in the AST, such as constructing the streams between nodes, become their own jobs.  So, while the AST has an implicit strict ordering, the compiler must build the corresponding job sequences such that it guarantees this ordering where the job scheduler does not.

The system does not allow for dynamic construction of new jobs (such as through loops over arbitrary, run-time values).  It also (currently) does not allow for managing spawned processes that live beyond the life of the script.

The builder has a responsibility to break the ordered actions and the node executions into trees of jobs and job sequences.  The [runtime](runtime.md) document describes this model.

## Modules

Modules define functionality that takes the place of what a scripting language provides (such as if statements or executable running or file redirection).

Modules must bridge between running within the compiled code, and providing information so that the builder can create source code to interact with them.  As such, they provide meta-data in code, and must follow strict coding guidelines to allow the builder to make assumptions about how to call it.

Some modules may also need to allow for external events to stop the execution early, so that it can cleanly stop.

**For now, see the [`meta.rs`](../src/shell_lib/compile/meta.rs) source for details on what's expected from modules.**

### The Main Module

Of note is the `main` module.  This represents the primary program that interacts with the OS.  It hosts the `stdin`, `stdout`, and `stderr` streams (or additional ones as necessary), parses command-line arguments, and registers OS signal monitoring.

Because of this special responsibility, the builder has different expectations on its behavior than other modules.  From the AST perspective, it works the same as other modules, though.


## Integration Code

The [builder](#builder) generates integration code to act as the compiled script.

The integration code currently splits the code into three main parts:

* the `main.rs` file.  This runs the whole program.
* the `runtime.rs` file.  Contains the shared runtime parameters, so that the jobs can correctly populate runtime parameters from other node data.
* one file per job sequence.  Contains the job definitions for each sequence.


## Utility Libraries

The utility libraries help make the integration code smaller by putting shared logic into places easier to test.
