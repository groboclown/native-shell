# Trials for the Script Language

The script language needs to be:

1. Simple.  It must have very few control structures to make debugging very simple, and those that do exist must be extremely simple in how they work.
2. Parallel.  In order to help with speeding up execution, the script should provide a DAG job processing system.
3. Non-ignorable errors.  Commands that can generate errors need to force error handling to ensure the script is safe.

The script language should provide:

1. Pluggable.  People should be able to create new modules that the script can take advantage of.
2. Basic typing.  Should support JSON like types, so that those file structures can be loaded into the script.



# Try 2

The primary use case for the language are container authors needing to create glue code between components, primarily for initializing the container state before executing the primary process in the container.  In some cases, the authors need to deal with historical tools that don't fit neatly into containerization.

With the first cut, it should try to cover common use cases related to containers.  While behaviors like "run a thing when a file changes" occur in some systems, it's not the primary use case we're covering.  This intends to cover:

* Acting on signals from the OS.
* Running server programs or complex setup programs
  * Redirecting input and output file descriptors to other programs or files.
  * Acting on exit codes
  * Allowing program restarts.

The "service" handling includes basic tee and file and shell stream redirects.  Actions like filtering from these should be possible, but it's a later priority.

```yaml

command-sets:
  # Command sets define a runtime environment for programs.  A program execution can use
  # a pre-defined command set, or use a custom command set, or modify a pre-defined command set.
  redis:
    cwd: "/var/lib/redis"
    env:
      REDIS_HOST:
        value: "localhost"
      REDIS_PORT:
        value: "${REDIS_PORT}"
    # Can also include:
    # on-error: for a list of things to do on the process exiting with non-zero error code.
    # on-success: for list of things to do on the process exiting with zero exit code.
    # on: for acting on a specific exit code value or range of values.
    # streams: for how to redirect file descriptors.
  web-server:
    cwd: "/var/www/html"

shell:
  # Shell reflects the execution shell environment, how it
  # manages OS interaction forwarding from the managed processes.
  signals:
    # When the shell receives a signal, it will forward it to the
    # processes who listen to a specific signal.
    SIGTERM: term
    SIGINT: term
    SIGHUP: reload

    # "+NAME" means a built-in behavior to send the given signal all running processes.
    SIGKILL: +kill
  env:
    # List of required or optional environment variables.  The system will
    # fail to start if required ones are missing.
    HOME:
      required: true
      # required: true cannot have a "default" field.
    REDIS_PORT:
      # providing a default implies required: false
      default: 6379
    ALLOW_USER_PAGES:
      default: "false"
    REDIS_ARGS:
      required: false
      default: []
  values:
    # Constructed values to use in the script.  Unlike environment variables, these aren't passed to child processes.
    # They can also have an 'array' structure, to split the value into an array, for use in injecting its value
    # into array parameters.
    # Also, these are scope sensitive, meaning they are evaluated at time of use.  If associated to a process,
    # then it reads from the environment variables passed to the process.
  
  # By default, stdout and stderr are line merged from all incoming
  # streams.
  stdout:
    merge: line
  stderr:
    merge: line
  stdin:
    close-on-start: true
  
  # If needed, also allows 'fd:' for a list of numbered file descriptors.
  # these must be declared as 'in' or 'out' types.  The 'std*' items are syntax sugar.


steps:
  # List of named steps to perform in-between the processes.
  "redis server config":
    run: filter-file
    with:
      source-file: /etc/redis.d/default.config
      output-file: /etc/redis.d/config
    replace:
      - text: "${BASEDIR}"
        with: /etc/redis.d
      - text: "#require-tls: false"
        with: "require-tls: true"
    on-error:
      - shell: abort
        err-message: "Invalid file replacement: ${err}"
  "web server modules":
    run:
      - if:
          oper: equal
          items:
            - "${ALLOW_USER_PAGES}"
            - "true"
        then:
          - run: cp
            with:
              - source-file: /etc/apache.d/modules/mod_userdir
              - output-dir: /etc/apache.d/active-modules
  "ensure db server running":
    run: retry
    with:
      execute:
        - /usr/local/bin/redis-cli
        - -u
        - "${REDIS_USERNAME}"
        - -p
        - "${REDIS_PASSWORD}"
        - -H
        - "${REDIS_HOST}:${REDIS_PORT}"
      expects-exit: 0
      retry: 3
      initial-delay: 5
      retry-delay: 2
      send-stdout: /dev/null
      send-stderr: shell.stdout
    on-error:
      - shell: abort
        err-message: "Timed out waiting for database to start: ${err}"


processes:
  # List of *possible* processes to run.
  # The 'shell' group's streams reflect some processes that must run,
  # by having 'required: true' set; this also implies all their dependencies
  # must run.  Other processes must have a 'default: true' set to force
  # execution, which also implies all their dependencies must run.
  redis-server:
    type: process
    # 'default' means it's a primary execution process.
    # As one of the sources of the shell, it implies 'default'.
    default: true
    command: /usr/local/bin/redis
    pre-steps:
      # List of steps to run before this starts.
      - run: redis server config

    command-set: "redis"
    # The command set describes many defaults, which
    # the process may overwrite.
    env:
      # Environment variables later overwrite earlier ones.
      - name: "REDIS_PASSWORD"
        value: "secret"
      - name: "REDIS_USERNAME"
        value: "admin"
    streams:
      - name: stdout
        into:
          - stream: shell.stdout
          - file: /var/log/redis/access.log
            # Could also include standard logging things like rolling backups
      - name: stderr
        into:
          - stream: shell.stderr
    on-actions:
      term:
        # What to do when the shell receives a signal that
        # forwards on to the 'term' actions.
        - type: send-signal
          signal: SIGTERM

  - name: "web-server"
    type: process
    command: /usr/local/bin/httpd
    default: true
    before:
      - parallel:
          - run: ensure db server running
          - run: web server modules
    on-halt:
      type: signal
      signal: SIGTERM
    command-set: "web-server"
    streams:
      output:
        - fd: 1
          into-file: /var/log/httpd/access.log
        - fd: 2
          into-file: /var/log/httpd/error.log
        - fd: 3
          into-file: /var/log/httpd/debug.log

  - name: "redis-active"
    type: module
    module: network-connects
    cmd-set: "redis"
    args:
      - name: address
        env: REDIS_HOST
      - name: port
        env: REDIS_PORT
```


# Try 1

Make the DAG job processing the top level language semantic.  Split the execution into "jobs", with dependencies declared within.  With the "job" being the execution primitive, it would then be easy to construct corresponding function names to make debugging easier.

This particular attempt is getting an idea of the parsed out data structure.  The script language itself can take many forms, but would be transformed into this as a kind of Abstract Syntax Tree.

Interestingly, because we define the AST as a well formed JSON style data structure, this means we should be able to construct a json schema for it, which directly informs the internal data structure creation of the transpiler.

The idea is that each keyed item relates to one of a collection of code snippet generators, whose generated text should be considered like an "object".  Every value passed to the item is itself another code snippet generator, with a few built in "atoms" (like references and constant values).  This is strongly typed, mostly because it needs to be to construct a valid program.

We can also envision that each code snippet generator provides:

* Value validators, to ensure the value is within an expected value.
* Custom types, such as enums, which need to be injected into the typing system.
* Execution simulation, so that tooling can allow testing of the model and interactive simulations.  This could additionally be configurable to allow automated tests to simulate different conditions.


```yaml
# Top level items are "sections".
# Sections are essentially objects with a primary action.
# The sections create a global variable space.

# The "main" section is special.  It is the only object called
# by the environment.  It could be an action or a job runner or
# something else that performs an action.
main:
  type: job runner
  on failure:
    # If the job runner fails and no failure handler takes
    # care of the error, then this is the final, outer failure
    # handler.
    "*":
      type: run action
      action:
        type: ref
        ref: ["exit", "action"]
  entrypoint:
    type: string
    const: "flow"
  jobs:
    type: job lookup
    # Each job is also a single instance, but here tied to
    # the "main" global space.
    setup:
      # sequential jobs run actions in order.
      type: sequential
      actions:
        # parallel jobs run actions one after the other.
        - type: parallel
          actions:
            - type: run action
              action:
                type: ref
                ref: ["defaults", "action"]
            - type: sequential
              actions:
                - type: run action
                  action:
                    type: ref
                    ref: ["json secrets", "action"]
                - type: run action
                  action:
                    type: ref
                    ref: ["parse secrets", "action"]
        - type: run action
          action:
            type: ref
            ref: ["env", "action"]
        - type: run action
          action:
            type: ref
            ref: ["create config", "action"]
      
      # If a failure happens in this or any sub-job, one of the
      # handlers runs next.  This is only run if none of the
      # chain of handlers takes care of it, or if it propagates another
      # error up.
      on failure:
        "*":
          type: run action
          action:
            type: ref
            ref: ["main", "jobs", "setup failure"]

    setup failure:
      type: sequential
      actions:
        - type: log
          level:
            type: log-level
            const: error
          message:
            type: string
            const: Setup failed.  Aborting.
        - type: exit
          code:
            type: integer
            const: 1

    wait for dependencies:
      type: parallel
      actions:
        # wrapper style jobs take care of what otherwise could be a source for
        # bugs due to tricky logic and timing issues.
        - type: repeat-until
          # The action to perform in the repeater.
          action:
            type: run action
            action:
              type: ref
              ref: ["contact database", "action"]
          
          # Action to perform after a repeat is triggered, before the next
          #   repeated action.
          action-on-repeat:
            type: sleep
            seconds:
              # Rather than just a hard coded number, this instead looks up
              # the sleep seconds in another location.
              # In this case, it's an exponentially increasing timeout, which
              #   is an instance with its own variables that change with each
              #   call.
              type: exponentially increasing number
              start:
                type: number
                const: 1
              exponential:
                type: number
                const: 1.2
              maximum:
                type: number
                const: 30

          conditions:
            - type: condition
              # Looks at the global section "contact database" result value.
              test:
                type: ref
                ref: ["contact database", "result"]
              matches:
                type: failure
                name:
                  type: string
                  const: "network unreachable"
              result:
                type: repeat-until-result
                # "repeat" means keep trying.
                const: repeat
            - type: condition
              test:
                type: ref
                ref: ["contact database", "result"]
              matches:
                type: failure
                name:
                  type: string
                  const: "connected"
              result:
                type: repeat-until-result
                # "complete" means all done with the loop and it's okay.
                const: complete
            - type: counter condition
              # Idea: the counter would be another instance with a local variable.
              greater-than: 3
              result:
                type: failure
                name:
                  type: string
                  const: "too many retries"
                parameters:
                  type: named store
                  count:
                    type: ref
                    # This shows that the AST view of referencing data
                    #   can also have relative positions.
                    # "." == this relative reference in the "count".
                    # "@" == the name "count"
                    # "^" == the "parameters" value.
                    # "^", "@" == the name "parameters"
                    # "^", "^" = the "result" value.
                    ref: ["^", "^", "count"]
            - type: timeout condition
              minutes:
                type: 
              failure: "database connection timeout"
        - type: run action
          action:
            type: ref
            ref: ["check dependent services", "action"]


contact database:
  type: try tcpip connect
  address: ["env", "values", "database host"]
  port: ["env", "values", "database port"]


check dependent services:
  # This is a template that constructs sub-items for arguments to another object.
  # Meta types take an AST and produce another AST.
  meta-type: template generator
  generated-type:
    type: list type
    item types:
      type: type
      type name:
        type: string
        const: repeat-until
  value type:
    type: structure type
    keys:
      address:
        type: string
      port:
        type: integer
  values:
    # The generator uses each of these sets of values (they must )
    type: list
    item types:
      type: ref
      ref: ["^", "^", "values"]
    items:
      - address:
          type: string
          const: "my-dns.local.name"
        port:
          type: integer
          const: 53
  template:
    # The generator is given this and transforms it.
    type: repeat-until
    # The action to perform in the repeater.
    action:
      type: run action
      action:
        # Because it's templatized, we build out the action here.
        type: try tcpip connect
        address: 
        ref: ["contact database", "action"]
    
    # Action to perform after a repeat is triggered, before the next
    #   repeated action.
    action-on-repeat:
      type: sleep
      seconds:
        # Rather than just a hard coded number, this instead looks up
        # the sleep seconds in another location.
        # In this case, it's an exponentially increasing timeout, which
        #   is an instance with its own variables that change with each
        #   call.
        type: exponentially increasing number
        start:
          type: number
          const: 1
        exponential:
          type: number
          const: 1.2
        maximum:
          type: number
          const: 30

    conditions:
      - type: condition
        # Looks at the global section "contact database" result value.
        test:
          type: ref
          ref: ["contact database", "result"]
        matches:
          type: failure
          name:
            type: string
            const: "network unreachable"
        result:
          type: repeat-until-result
          # "repeat" means keep trying.
          const: repeat


consts:
  # This is a "data store".  It's just storing constants for reusable
  #   lookups.  The AST should keep these for easier source-to-AST reference.
  #   It's the equivalent of a Json dictionary.  However, due to the linking
  #   nature of things, this can't be used for dynamic name lookups,
  #   like an in-memory hashtable could.
  type: named store


```

Based on this, some notes:

* "ref" types are essentially pointers to variable contents.  They allow for peeking into data loaded from files or environment variables which themselves may have been altered by other actions.
* The names from an AST are turned into essentially `path_item_key_another_item` names.  This will help make the names unique and linkable to the source.
* During code generation, it will need to be done in two passes - first the meta-type translation, then a post order visiting of the tree to construct the final generated code.  The code generation will need to store the in-flight code snippets in the node so that the higher level can plop it in.  Better yet would be to have the code generators not have access to the leaf generated code, and instead create a tree of generated code.

