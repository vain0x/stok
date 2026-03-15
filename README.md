# stok
> AI-usage: *AI-generated code* (designed and reviewed by human), *AI-translated documentation* (originally written by human)

Environment variable injector to *avoid hard-coding tokens* in repository files.

Invoke a shell command with *environment variables resolved* to their stored values.

## Workflow
- Assume you have an API token (that looks like `tk-xxx`) and a server application `myserver` to use it
- Store the token associated with a name
    - `stok --set MY_API_TOKEN`
        - On prompt, enter the token value
- Set an environment variable (or add it to a `.env` file) to mark it for injection
    - `export MY_API_TOKEN=stok:MY_API_TOKEN`
- Invoke `stok myserver` to start the server
    - The `stok` command resolves `MY_API_TOKEN` to its actual value and invokes the `myserver` command
    - The `myserver` process reads the environment variable `MY_API_TOKEN` to retrieve the actual token (`tk-xxx`)

## CLI Interface
### `stok <COMMAND>`
Execute a shell command with environment variables resolved.

Options must be specified before `COMMAND`.

- `--env-file <FILE>`
    - Load `FILE` in `.env` format to extend environment variables.
        - Only variables whose value starts with `stok:` are loaded.
        - Takes priority over auto-loaded env files.
- `--no-env`
    - Don't load `.env`/`.env.local` files on local (See [#Auto-loading Env Files](#auto-loading-env-files).)

### `stok --set <NAME>`
Store a key-value pair.

You will be prompted to enter the value. (Note you don't want to save secret values like API tokens in the command line history.)

- `--value <VALUE>`: Specify the value (non-secret)

### `stok --unset <NAME>`
Delete the key.

- Alias: `--delete, -d`, `--remove`, `--rm`

### `stok --ls`
Display all stored keys.

### Help/Version
- `stok [help|--help|-h|/?]` Show help
- `stok [version|--version|-V]` Show version

## Implicit behavior
### Auto-loading Env Files
By default, the stok command loads the env files (if any) in the current directory where it's invoked.

- `.env.local` (prioritized)
- `.env`

Only variables whose value starts with `stok:` are loaded from these files.

Opt-out to specify `--no-env` or `STOK_NO_ENV=1`.

## Storage
- Data is stored in the user's local state directory
    - Windows: `%LOCALAPPDATA%\stok\.env`
    - Unix: `$XDG_STATE_HOME/stok/.env` (defaults to `/home/<USERNAME>/.local/state/stok/.env`)

## Security Concerns
The application assumes the following:

- You don't want to write down the values in .env or export as environment variables directly
- You don't need to back up (or sync) these values across machines
- It's okay to store them in *plaintext*

## Development Documentation
See [docs/dev/architecture.md](docs/dev/architecture.md)
