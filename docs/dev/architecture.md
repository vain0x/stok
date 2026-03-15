# architecture.md

Supplements README.md (the user-facing documentation) with implementation guidelines and details.

## Tech Stack
- Rust

## Guidelines
- Cross-platform (Windows and WSL)
- Minimize cargo dependencies
- No over-abstraction
- Integration testing

## Modules
- `main.rs`: Application entry point
    - CLI argument parsing and routing to subcommands
- `app.rs`: Core application logic
    - `struct Options`: Consolidated application options
    - Database file read/write
- `cmd/`: Subcommand implementations (one module per command)
- `util/`: General-purpose utilities

## Behavior
- `.env` parser
    - No special escape sequences. Values containing spaces or `"` are enclosed in `""`. Newlines in values are not supported.
- Database file
    - Uses the same format as `.env`. This reuses the `.env` parsing logic and is efficient enough (the number of stored entries is expected to be small).
- `--set` command: value is prompted without echo (`rpassword` crate) to avoid leaving tokens in the console history.
- `stok:` reference notation
    - No escape mechanism is provided, as name collisions are unlikely.
- Subprocess execution
    - Do signals like Ctrl-C and SIGTERM propagate to the subprocess?
        - On Unix, replacing the process via `exec` handles this naturally. On Windows, spawning a child process should work correctly (to be verified).
