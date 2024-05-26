# gleam2_add_issue_2024_05_26

This project reproduces a problem where a project can get into a state where `gleam add` consistently produces the following error:

```
  Resolving versions
error: Fatal compiler bug!

This is a bug in the Gleam compiler, sorry!

Please report this crash to https://github.com/gleam-lang/gleam/issues/new
and include this error message with your report.

Panic: /private/tmp/nix-build-gleam-1.1.0.drv-0/gleam-1.1.0-vendor.tar.gz/pubgrub/src/internal/partial_solution.rs:131
	add_derivation should not be called after a decision
Gleam version: 1.1.0
Operating system: macos

If you can also share your code and say what file you were editing or any
steps to reproduce the crash that would be a great help.

You may also want to try again with the `GLEAM_LOG=trace` environment
variable set.
```

## Reproduction steps

1. `gleam new my_project`
1. `gleam remove gleeunit`
1. `gleam add --dev startest`
1. `gleam add wisp` 💥
