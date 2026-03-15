use std::{env, path::PathBuf, process};
mod app;
mod cmd;
mod util;

fn print_help() {
    println!(
        r#"stok - Environment variable injector

USAGE:
    stok [OPTIONS] <COMMAND> [ARGS...]  Execute command with env vars resolved
    stok [OPTIONS] --set <NAME>         Store a key-value pair
    stok [OPTIONS] --unset <NAME>       Remove a key
    stok [OPTIONS] --ls [QUERY]         List stored keys

OPTIONS:
    --env-file <FILE>   Load env file (higher priority than .env/.env.local)
    --no-env            Don't auto-load .env/.env.local

HELP/VERSION:
    stok help | --help | -h | /?        Show this help
    stok version | --version | -V       Show version"#
    );
}

fn print_version() {
    println!("stok {}", env!("CARGO_PKG_VERSION"));
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0;

    if args.is_empty() {
        print_help();
        return;
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" | "/?" => {
            print_help();
            return;
        }
        "version" | "--version" | "-V" => {
            print_version();
            return;
        }
        _ => {}
    }

    let mut opts = app::Options::new();

    let result = match args[0].as_str() {
        "--set" => {
            i += 1;
            if i >= args.len() {
                eprintln!("error: --set requires a NAME argument");
                process::exit(1);
            }
            let name = args[i].to_string();
            let remaining = &args[i + 1..];
            let value = remaining
                .windows(2)
                .find(|w| w[0] == "--value")
                .map(|w| w[1].to_string());
            cmd::set::run(&opts, &name, value.as_deref())
        }
        "--unset" | "--delete" | "-d" | "--remove" | "--rm" => {
            i += 1;
            if i >= args.len() {
                eprintln!("error: --unset requires a NAME argument");
                process::exit(1);
            }
            cmd::unset::run(&opts, &args[i])
        }
        "--ls" => cmd::ls::run(&opts),
        _ => {
            while i < args.len() {
                match args[i].as_str() {
                    "--env-file" => {
                        i += 1;
                        if i >= args.len() {
                            eprintln!("error: --env-file requires a FILE argument");
                            process::exit(1);
                        }
                        opts.env_file = Some(PathBuf::from(&args[i]));
                        i += 1;
                        continue;
                    }
                    "--no-env" => {
                        i += 1;
                        opts.no_env = true;
                        continue;
                    }
                    arg if arg.starts_with('-') => {
                        eprintln!("error: unknown option '{}'", arg);
                        process::exit(1);
                    }
                    _ => break,
                }
            }

            cmd::run::run(&opts, &args[i], &args[i + 1..])
        }
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
