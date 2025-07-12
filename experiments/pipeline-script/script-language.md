# Trials for the Script Language

First off, the native modules must define their interfaces.  While we can put this into code, by making it something independent, it allows for constructing code templates and documentation.

```yaml
module:
  name: NAME
  version: VER.SION
  authors: [author, list]
  # and so on with basic header stuff.

  streams:
    slot1-name:
      # The kind of slot.  "in" allows binding to a single receiving stream,
      # "out" allows binding to a single sending stream.  "req" means required
      # (script authors, if they use this module, must provide a binding),
      # "opt" means optional.  "multi" means it allows multiple streams to bind,
      # and the engine will interleave values from the senders.
      type: in-req | out-req | in-opt | out-opt | in-multi

      # The module must declare the types of values allowed on the stream.
      # This can be done with a direct schema, or a reference to an external schema.
      schema:
        # JSON schema for allowed values.
      schema-ref: ref-name
  
  actions:
    # Set of named actions that bound listeners can invoke.
    name:
      description: text
      parameters:
        # Actions have 0 or more ordered parameters.
        - name: bar
          schema:
          schema-ref: x
  
  state:
    # The module can provide event listeners with information into the state of the
    # module at the time of the event.
    name:
      description: text
      schema:
      schema-ref:

  config:
    # Values that the script writer can configure.
    - name: CONFIG_NAME
      schema:
      schema-ref:
```


The language itself comes at the angle that we want to focus on container authors as the primary consumer, who want to set up an initial state for the container at start time before running the primary program, and possibly also allow controlling the program as the container receives OS signals.

While this could define a DSL, instead it will structure the basic language through a yaml document; the language construct can come later, but this shows organization of concepts, along with pointers to how this construct leads to the engine functionality reworking.


```yaml
nodes:
  - decl:
      # Define a module to run.  The module references a built-in kind,
      # and the script can reference it by the alias.
      kind: MODULE_NAME
      alias: REFERENCE_NAME

      config:
        # Configure the module.
        value1: "foo"

        # Values can reference value nodes.
        # This includes sugar to make value construction easier.
        value2: "the ${env.USER}"

        value3:
          - "${env.HOME}/bin"
          - $ref:
              # The DSL should allow some kind of spread syntax (e.g. "...env.PATH")
              node: env
              value: PATH
            
          # value interpolation allows simple things like providing defaults.
          - "${env.TEMP:/tmp}/bin"

    # All nodes allow for binding their streams to other nodes.
    # 'exec' nodes have an enhanced form of this, described with the 'exec' node
    # example below.
    bind:
      # script authors bind the node's outbound
      # stream slots to other nodes' inbound streams.
      - name: slot1-name
        target-node:
          alias: NODE_ALIAS
          slot: SLOT_NAME
      # As a short-hand, the slot allows for direct
      # file reference.
      - name: slot2-name
        target-file: the/filename.txt
      # Similarly, as a short-hand, an input slot
      # can reference a file.
      - name: input-slot-name
        source-file: the/source.file

    # As a short-hand for waiting for another node send an event when it
    # completes (which would then send an action to run this node).
    depends-on:
      - node: ALIAS
        # The status is the node state object.  All given items here
        # must match.
        # This should allow for more than just exact match; numerics should
        # allow for ranges (such as "> 0" or "1-6")
        status:
          exit-code: 0
    
    listens:
      - on: ACTION_GROUP
        # The listener behavior can reference the event object
        # as well as the node state and value nodes.
        # This `as` references the event object.
        as: EVENT_ALIAS
        when:
          # Optional condition logic here.
          oper: and
          items:
            - oper: or
              items:
                - oper: equal
                  items:
                    - "${ev.signal}"
                    - SIGHUP
        perform:
          # Can trigger a stream graph to execute.
          - action: run
            node: ALIAS
          # Can send an event to an action group.
          - action: emit
            event: "the event value"
          # Can also interact with this node.
          - action: signal
            name: sighup

  - decl:
      # Execute a process.
      # With a DSL, it makes sense to have this be a shorthand distinct from internal modules.
      # Technically, a process is just another kind of module.
      kind: exec
      alias: ALIAS_NAME

      # The command execution works like a module, but sugar should make it easier to
      # set up, because most people will be spending their time writing exec steps.
      config:
        cwd: "${HOME}"
        env:
          $ref:
            node: env
            value: env
        # Do different but exclusionary forms of command are allowed:
        cmd:
          - /usr/sbin/echo
          - foo
          - bar
        # And
        exe: /usr/sbin/echo
        args:
          - foo
          - bar
    
    bind:
      # the 'std' streams can be used.
      - name: out
        target-node:
          alias: NODE_ALIAS
          slot: SLOT_NAME

      # Bind allows for arbitrary numbers, which act like file descriptors.
      - fd: 6
        # Whether it's a in or out stream is determined by using "target" or "source".
        target-file: the/file/name
      - fd: 7
        # Source streams, because they're implicit, aren't required, even for fd,
        # but can be given to help with ensuring consistency with the script.
        source: true
        allows-multiple: false

    # Listens is exactly the same.

values:
  # Value nodes interact with the nodes in the dependency graph.
  # They have a "kind" which allows logic to manage what they do
  # with the collected streams.
  # They have only inbound streams, and do not allow listeners.
  - decl:
      name: ALIAS
      kind: VALUE_NODE_KIND

      config:
        # They allow config.

    # Binding only makes sense for the sugar for inbound files.
    bind:
      - name: source
        source-file: /the/file.name

serials:
  # Serial allows for a short-hand for job management.
  - decl:
      alias: NAME

      # All serial have a common on-status trigger.
      status:
        exit-code: 0

      nodes:
        # Each item is an ordered node declaration.
        # These trigger the whole stream graph.
        # A later node might be a node lower in the graph from a previous
        # node in the serial list.
        - decl:

        # It also allows a special form.
        - $ref: NODE_ALIAS

iter-loops:
  - from: NUMBER
    to: NUMBER
    step-by: NUMBER
    as: ALIAS

    # This alias acts as a value node.

    # 'nodes' and 'serial' can be contained here.
    # I don't think "values" belong here.

stream-loops:
  # Loop over each item in the stream
  - from: STREAM NAME

    # Stream looped groups allow for essentially the loop
    # as a node that can be used as a sender.

    alias: NAME

    outputs:
      # These do not need the schema, because it's
      # implicit in what pumps into it.
      - name: NAME

    # This can have 'nodes' and 'serials' and 'iter-loops' and other 'stream-loops'.
    # I don't think "values" belongs here.

```
