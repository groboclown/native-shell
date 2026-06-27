# Provided Functionality

The script executable enables an approach to manage how multiple processes interact with each other and the operating system.  The programs have requirements for environment construction that the executable must fulfill.

## Value Construction

Throughout use of the script, the user may need to construct input values to various components.  This may have specialized versions of the values, such as name value pairs.

All resources in the script that require a value pull it from a *closed stream*.

While the following items will generally use more efficient methods, they show the different mechanisms which can use the streaming for value construction:

* `echo value` - outputs a hard-coded value to the stream;
* `echo foo-${name}-bar` - outputs a value constructed from a variable;
* `set` - outputs the script executable's (parent) environment variables to the stream;
* `cat file` - outputs the contents of a file to the stream;
* `curl http://example.com` - outputs the contents of a URL to the stream;
* `jq -r '.[]|.name'` - outputs the value of a JSON element from an array of JSON objects to the stream.

Values should have different types, with simple rules for converting between types.  For the purposes of this document, it conforms to the JSON typing standard.  Note that conversion from lists or maps to strings should require a formatting function.

Note that values differ from *variables*.  Variables hold values.


## Streaming

A "stream" represents a pipeline that sends streaming JSON values from exactly one stream node to exactly one other.  The stream nodes perform generation, sink, merge, filter, or fork on the stream.

The script does not perform schema definition conformity over the values.  However, a script could include a filter that parses a stream and only allows conforming values, or perhaps errors out on invalid values.

Most stream processors intrinsically use line-based operators, meaning that the new line separates values, and each value represents a single string.

A stream may mark as "cached", which means the script executable can keep the value of the stream contents unless an upstream node makes notice of a change.

The script abstracts all value use to streaming, though in many cases it optimizes this.

The script may rely on OS processes to act as stream nodes, such as `cat`, `jq`, or `curl`.  It may also use internal modules to handle some stream nodes, such as line-to-name value pair mapping.

Any resource that requires a stream can only begin executing after construction of both ends of the stream.  A partial stream (only one end is defined) is an illegal state and prohibits the script from running.  A stream waiting for both ends to be ready is called a *pending stream*.  There may exist circumstances where a pending stream can only complete if one node can only start after the other end completes; this state is a *deadlock*; where possible, the script should identify the potential for these circumstances at compile time.

Streams need to include the concept of "closing".  Some modules require acting only on closed streams (such as sorting).  A stream *closes* when the input node side of the stream completes execution.  If some upstream process prevents a node from starting, then the stream is essentially closed without writing to it.  A stream with one end closed is a "partially closed" stream, and can further be described as a "send closed" and "receive closed" (or "read closed" and "write closed") stream.

Read ends of a stream may terminate before the write end of the stream.  Some nodes that send to a stream may require transmitting all of its data regardless of the receive end's closed state, or it may terminate early if the stream enters receive closed state, and vice versa.  For example, `/dev/urandom` will continuously send values and halt when the receive end closes, and `/dev/null` will consume all values until the write end closes.  The system should consider a stream connected between two "until other is closed" nodes as a valid script, as a common use case has a server send log data to a file.


## Action Groups and Events

An action group uses a name to indicate receivers of some event that originated from the script executable.  In most cases, this reflects OS signals sent to the shell, like SIGINT or SIGTERM.  It can also come from events such as a process exiting, or a file system change.  The script executable constructs monitors that send events to action groups based on the monitored event.

Most events contain no data, they simply emit that an action happened.  Some things, like file changes, can include data about the change, such as the file name and the type of change.

If something generates an event, it passes through the script executable's monitors, which map the event to action groups.  For OS signals, the script defines how to map these to action group names and events.  For events generated within the script, it defines the action group name and the event data.

The group itself just defines a name and an event schema.  The script executable monitors define how to forward events into the correct action group.  The script executable provides hooks for executing actions when an event on an action group occurs.  It also provides ways to map events to action groups.

Examples:

* A file generator merges two files together.  It adds a file monitor to both parent files, such that if either changes, it should rerun.
* A web server adds a file monitor to its configuration file, such that if the configuration file changes, the script executable sends a SIGHUP to the web server.
* A user presses Ctrl-C in the terminal, which sends a SIGINT to the script executable, which runs all actions on the `term` action group.
* The script executable runs once every 5 seconds a network connection check, to monitor if a database server still exists.  If not, it sends an event to the `db-down` action group.  This triggers a process to then retry connections until the database server is back up, at which point it sends an event to the `db-up` action group.  That causes the web servers to restart.
* A 'test' process has action groups associated with different exit codes.  Each collection of exit codes (or a default group for those not specified) generates a different event data with a different action group name.

Note that not all process failures trigger action groups.  In some cases, special action handlers allow for retries with exponential backoff, or allow for a fallback process to run.

Events work a little like streams (an event enters, and is rebroadcast to all listeners), but have some very key differences.  Particularly, action groups do not have a notion of "closed", and may have 0 or more receivers who only receive events while listening.

### Action Listeners

An action listener is associated with a [node](#nodes).  It receives the events from the associated action group to trigger a behavior.  The node may only listen to the action group during specific periods of its life cycle.

The trigger can include any number and mix of these behaviors:

* Execute an action on the bound-to node.  For example, sending a SIGHUP to its associated process PID.
* Send an event to an action group.
* Send a value to a stream attached to the node.  This allows turning events into stream values.
* Inspect the event and node state to determine which course of actions to take.


## Nodes

A stream requires exactly one sender and one receiver node.  Nodes, on the other hand, take 0 or more incoming streams, and generate values to 0 or more outgoing streams.  When the node completes execution, it closes the stream ends the node controls.

The node's stream ends are collected into indexed stream bundles; a bundle can contain 0 or more stream, with the node dictating the minimum and maximum allowed.

Nodes also declare actions that may run on them.  The node actions have a name and optional set of ordered parameters.

### Value Nodes

A *value node* is a special kind of node that provides keyed evaluated values, which allows for "plugging into" nodes that use a value.  These have *N* incoming streams, zero outbound streams, and only provide the value after all incoming streams are closed.  This means that the node has behavior that, when complete, closes the incoming streams on the receiving end, and on completion generates the keyed values.

In the trivial case, a value node has zero incoming streams and represents a constant value.

### Stream Graphs

As the streams imply a directed acyclic graph of nodes, the independent collection of these streams constructs a "stream graph".  The graph must execute as a collective whole.

### Value Graphs

While a stream graph may contain value nodes, other nodes may receive a value from a value node in the stream graph.  Because values received are immutable, this means the nodes with relationships through values constructs another, larger directed acyclic graph.

A node may receive a value from a value node within the same stream graph as itself, but only if the value node is a strict ancestor node of degree 2 - if it is the write end of a stream directly received by the node that accepts the value, then the child node cannot start until the parent has completed, which creates a deadlock state.

### Launch Group

The entire graph of the Value Graph can also be called a "launch group".  If an action happens that requires one process to run due to an action, that causes its entire launch group to rerun.

Launch groups have their own run states that reflect the run states of their contained nodes.  A node may depend on a specific node within a launch group to complete, which allows for enhanced parallelism (it doesn't need to wait for the whole graph to complete).

### Node Owner

A node may, as part of its execution, own the execution of zero or more launch groups, and may rerun a launch group.

This allows for behaviors such as "retry N times with an exponential backoff".

Such a node owner may only terminate when all of its owned launch groups have terminated.

The node must manage the owned launch group execution state itself.

### Node and Graph Execution States

The execution state of a node broadcasts as values into a stream.  With each execution state change, the new state enters the stream, in an associated value of `{ "node": "(node ID)", "state": "(state)", "code": 0 }`.  When the node completes execution, the node sends the `completed` state along with its exit code, then closes the write end of the stream.

Likewise, a Stream or Value Graph can have an associated state node.  This state node contains a value which describes the list of all node identifiers within the graph, as well as an output stream that sends the state changes for each node in the graph.

When considering the stream graph, the execution state makes a "sub graph" barrier.  It allows for the non-execution state streams and values to be considered one group, and dependencies of these state streams as a super graph.

The execution state stream for nodes and graphs can optionally have a receiver node.  If it doesn't, it can still communicate to outside nodes through action groups.

### Node Execution

A node executes by running a *module* within the script.  The script provides many small modules to perform basic functions. like accumulate values, or write data to a log.  They can also generate events based on activity they discover within their execution.  They also have listeners that react to action events.

The most critical module executes and monitors an external application.  For this, the module must:

* Define the executable file and arguments.
    The script executable extracts the executable file from the value node associated with the executable file in the process definition.  This must evaluate to either a file, a string defining the file and its arguments, or an array of the file and its arguments.
    The file must include executable permissions by the user assigned to run the process.
    Implicit in this definition is the use of the `$PATH`.  The script executable requires an absolute path to the executable file, which means the stream to generate this value may include path searching.
* Define the environment variables.
    When starting the process, the script executable must construct the environment variable name value pairs.  These must come from value construction, with the requirement of a schema of a map of string -> string values.  Various formatting processors may layer on the stream processing of input.
* Set the current working directory.
    The working directory comes from a value stream and must match to an existing directory.  The directory must also have the correct permissions for the user running the process.
* Prepare the File Descriptors.
    The process requires correct file descriptor assignment.  Usually, this takes the form of stdin, stdout, and stderr, but some processes may require additional file descriptors.
    The file descriptors align to streams, so the script executable must determine their shape, whether FIFO queues or files or other OS primitives.  While commonly a shell will construct these OS primitives, the script executable may instead use internal modules to handle some stream nodes, in which case the script executable handles the data forwarding.
* Assign correct user and group IDs.
    The process may require running as a specific user or group.  The script executable must ensure that the user and group IDs are set correctly before starting the process.  This may involve changing the user and group IDs of the current process, or it may involve using a different mechanism to run the process under the correct user and group.
* Act on special action group events.
    The process may require receiving OS signals based on other actions sent through the system.  For example, a web server may require a SIGHUP if a configuration file changes, or a SIGINT if the user wants to stop the process.
    The script executable must include the forked process in all associated action groups, and connect them to run the appropriate OS signal on the process.
    The action group listener may need state awareness - how to handle the action group if the process is running, has stopped, has never run, is waiting on dependencies, has stopped but tear-down actions running, and so on.  Only the process itself has direct knowledge of the process' state.
* Generate correct events based on setup errors.
    If the script executable encounters an error while preparing the process, it must handle the error condition appropriately.  The script allows for each error condition to trigger an event.  In general, these just lead to a general process execution start failure event, which normally leads to script execution failure.
* Launch the executable.
* Monitor the executable's state.
    Once the process starts, the script executable must monitor the process state.  This includes checking if the process is still running, if it has exited, and if it has exited with an error.  The script executable must also handle any signals sent to the process, and ensure that the process is cleaned up correctly when it exits.
    On exit, the exit code must be sent to the node state stream.


## Ordered Execution

The script executable must ensure that processes run in the correct order.  This means that if a process depends on another process, the script executable must ensure that the dependent process starts after the other process has started.  It also means proper management of parallel processes.


## Construction by Declaration

The script functionality prefers a declarative approach to constructing the data execution method.  This puts severe limitations on the allowed scripting behaviors:

* Variable value mutation.  As marked above, nodes may access a value through a value node.  Therefore, by definition, all variables are non mutable; value changing implies construction of a new value.  A script implementation could use weird copy by value rules, however this can lead to surprising behavior, which leads to developer mistakes.  Note that this makes refactoring tougher and more error prone in some cases.
* Loops:
    * Loop until detection of an external state.  For example, a URL retry, or restart a server if it dies unless a file exists.  The scripting engine does not allow nodes to perform this kind of behavior, as it requires value mutation.  Instead, a custom node would need to perform the monitoring, and instruct the re-execution of a launch group.
    * Looping over a collection of streams.
    * Looping over values from a stream.
    * Looping over indices.
    * Joining streams defined within the loop.  A loop can construct streams and nodes, where the read end of the streams are used outside the loop.
    * Loops for groups of values.  If the script allows for constructing loops (say, run 100 different threads to generate an image for each number), then this means a requirement for sets of the processes and streams created within the loop.  This has down-stream effects of needing to reference these from outside the loop.  The set of values constructed from a loop must be handled as a complete set.  Note that action groups can help here, as an event sent to an action group is handled by every listener on that action group.  Behaviors such as testing if any, all, or none of the processes failed require their own join handling.
    * Loops for indexed values.  In some cases, a loop helps simply a script by constructing N objects which should be considered individually.  The script allows for this through indexed values where the range of indexed values is clearly known at start time.  These, though, also generate an implied group of values.  The primary difference is the ability of outside nodes to reference indexed only by groups.  The primary difference comes from length known at start time versus at loop execution time; running the loop based on the value generated by a stream only allows for groups, not indices.


# Implementation Notes

While the structure defined above presents a nice abstraction to cover all the necessary behaviors, the implementation has an opportunity to bypass much of the indirection.  As it turns the script into native code, it can directly construct file descriptor linkage, inject low level values directly into modules, perform `select` on many file descriptors across many processes, and directly call signals on the affected PIDs.  Note that the action groups still generally need a broker due to the dynamic add/remove listening capability (a few cases may have exactly 1 listener, which allows for optimizations).
