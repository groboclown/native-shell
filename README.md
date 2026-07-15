# native-shell

A shell like language compiled to help secure your container and image.

## About

`native-shell` sprouted out of [precision-shell](https://github.com/groboclown/precision-shell) in an effort to make your Docker / Podman container even more secure.

It uses a shell-like language with some built in commands to act as a wrapper around your primary container program.  The native-shell compiler then turns this into a native program to be a replacement for the standard `docker-entrypoint.sh` script.

By being a native program rather than a script run by an interpreter, it means your container reduces its attack surface.  Instead, it only does the one thing your script tells it to do, and can't be used with a different script to do something different.

## Current State

The program is in a very pre-alpha stage.  It's being designed and experimented with.

It has these parts:

* Low Level Script.  The underlying representation that the "builder" part of the code uses to compile the native application.
* Script Parser.  Turns the shell script into the Low Level Script.  This also allows for different script languages to be supported, if anyone cares enough.
    * The first cut will have a custom shell script syntax, based on simplified `sh`.  Long term goal will allow for multiple shell script parsing.
* Builder.  Takes the Low Level Script and compiles it into a native executable.
* Script Tooling.  A generic category for things that can perform other actions on the syntax tree.  Some ideas brewing are:
    * Data inspection.  Allows for gaining insights into available values and process flow.
    * Interactive simulation.  Allow for the user to simulate different conditions and behaviors from the invocations, and see how the script behaves.
