# native-shell

A shell like language compiled to help secure your container and image.

## About

`native-shell` sprouted out of [precision-shell](https://github.com/groboclown/precision-shell) in an effort to make your Docker / Podman container even more secure.

It uses a shell-like language with some built in commands to act as a wrapper around your primary container program.  The native-shell compiler then turns this into a native program to be a replacement for the standard `docker-entrypoint.sh` script.

By being a native program rather than a script run by an interpreter, it means your container doesn't have another executable that's an attack vector for running other scripts.  Instead, it only does the one thing your script tells it to do, and can't be used with a different script to do something different.

## Current State

The program is in a very pre-alpha stage.  It's being designed and experimented with.

