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


## Builder

The builder takes the AST and constructs source code that allows for direct implementation with as little need for runtime interpreter execution as possible.  For example, requiring a parameter to use the value `${HOME}/bin` will instead be turned into "get the environment variable HOME, append `/bin` to it," rather than run-time string parsing.


## Job Scheduler

While the AST defines nodes of modules with execution parameters and event handling, the job scheduler looks at all that behavior in a different way.  It instead manages a collection of job step sequences and jobs.  This runs at a different abstraction level than the AST, meaning some parts implicit in the AST, such as constructing the streams between nodes, become their own jobs.  So, while the AST has an implicit strict ordering, the compiler must build the corresponding job sequences such that it guarantees this ordering where the job scheduler does not.

The system does not allow for dynamic construction of new jobs (such as through loops over arbitrary, run-time values).


## Modules

Modules define functionality that takes the place of what a scripting language provides (such as if statements or executable running or file redirection).

Modules must bridge between running within the compiled code, and providing information so that the builder can create source code to interact with them.  As such, they provide meta-data in code, and must follow strict coding guidelines to allow the builder to make assumptions about how to call it.

Some modules may also need to allow for external events to stop the execution early, so that it can cleanly stop.


## Integration Code


## Utility Libraries
