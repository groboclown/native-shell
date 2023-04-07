# Trials for the Script Language

The script language needs to be:

1. Simple.  It must have very few control structures to make debugging very simple, and those that do exist must be extremely simple in how they work.
2. Parallel.  In order to help with speeding up execution, the script should provide a DAG job processing system.
3. Non-ignorable errors.  Commands that can generate errors need to force error handling to ensure the script is safe.

The script language should provide:

1. Pluggable.  People should be able to create new modules that the script can take advantage of.
2. Basic typing.  Should support JSON like types, so that those file structures can be loaded into the script.


## Try 1

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
