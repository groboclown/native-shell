# Pending Things Left To Do

## Current Codebase Fixes

* The `jobs::JobSettings` structure should just use a single place for all commands and jobs, to make the generated structure and field names simple and less conditional.
* Parameter generation for lists needs fixing.
* The module generation needs some work.
* Plug in the `todo!()` scattered throughout the code.  Current goal fills in enough functionality to get the cat_cp example to build and run correctly.

## Cargo.toml Generation

Some old code artifacts still exist for this, but it needs refactoring based on the new model.

## Macro Generation

Currently, the macro generation is scare to non-existent.  This needs some love.

## Big Ticket Items

More out-of-the-box modules.
