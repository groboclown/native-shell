# Ideas

The environment that runs the script must provide:

* Exquisite logging.  You should be able to tweak the logging levels for capturing the right amount of data.  This should also have sufficient protections to prevent secrets from leaking into the logs.
* Debuggable.  The compiled code needs to allow for stepping through and monitoring the state of the execution.  This includes having clear mappings between the script file and the place in the debugging.
* Testable.  The script must allow for writing tests against it, and running it in a testable, controlled environment.
* Inspectable for security.  The script must transform into a format that can be examined for security.

Secondary goals:

1. Fast.  As a wrapper around code that brings up a container, operations wants the container up and running as quickly as possible to minimize down time.
2. Small.  The executable must be kept very small to prevent image bloat.  Additionally, by keeping the code at a minimum, it shrinks the attack surface, and makes security inspection have less code to review.
3. Extensible.  New "plugins" should be addable, such as pulling information from a cloud service.

Some nice things:

* For complex jobs, it could have tools to help see the execution plan and execution flow.
* It could include test helpers to experiment with conditions on what would happen if an action behaves in a particular way.  This could be added into the testing tools for automation.
* It could also provide a data explorer to see a mapping of everything the script contains and can do.  For example:
    * failure analysis
        * these blocks of code can generate these failures.
        * these blocks handle these failures, and these other failures bubble out.
        * this knowledge would need to be built into the tooling anyway, to enforce all failures are handled.
