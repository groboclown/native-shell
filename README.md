# native-shell

A shell like language compiled to help secure your container and image.

## About

`native-shell` sprouted out of [precision-shell](https://github.com/groboclown/precision-shell) in an effort to make your Docker / Podman container even more secure.

It uses a shell-like language with some built in commands to act as a wrapper around your primary container program.  The native-shell compiler then turns this into a native program to be a replacement for the standard `docker-entrypoint.sh` script.

By being a native program rather than a script run by an interpreter, it means your container doesn't have another executable that's an attack vector for running other scripts.  Instead, it only does the one thing your script tells it to do, and can't be used with a different script to do something different.

## Current State

The program is in a very pre-alpha stage.  It's being designed and experimented with.

It's intended to be broken into several parts:

* Syntax Tree.  The scripts will be parsed into the syntax tree.  This allows plugins to use a common data structure to inspect script snippets and turn them into the appropriate code for compilation.  This will be used as the *lingua franca* for the tooling.
* Script Parser.  Turns the shell script into the Syntax Tree.  This also allows for different script languages to be supported, if anyone cares enough.
* Script Tooling.  A generic category for things that can perform other actions on the syntax tree.  Some ideas brewing are:
    * Data inspection.  Allows for gaining insights into available values and process flow.
    * Interactive simulation.  Allow for the user to simulate different conditions and behaviors from the invocations, and see how the script behaves.
* Snippet Plugins.  These come in one of two forms:
    * Code Generator: takes the subset of the syntax tree and constructs the code for compilation.
    * Meta Type: takes the subset of the syntax tree and productes another syntax tree.
