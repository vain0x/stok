use crate::app::{read_db, Options};
use crate::util::{env_parser, exec};
use std::{collections::HashMap, env, fs, process::Command};

pub fn run(opts: &Options, command: &str, args: &[String]) -> Result<(), String> {
    let db = read_db(opts)?;
    let db_map: HashMap<&str, &str> = db.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();

    // Build merged env with priority: .env < .env.local < --env-file < current process env
    let mut env: HashMap<String, String> = HashMap::new();

    if !opts.no_env {
        if let Ok(content) = fs::read_to_string(".env") {
            for (k, v) in env_parser::parse(&content) {
                if v.starts_with("stok:") {
                    env.insert(k, v);
                }
            }
        }
        if let Ok(content) = fs::read_to_string(".env.local") {
            for (k, v) in env_parser::parse(&content) {
                if v.starts_with("stok:") {
                    env.insert(k, v);
                }
            }
        }
    }

    if let Some(env_file) = &opts.env_file {
        let content = fs::read_to_string(env_file)
            .map_err(|e| format!("Failed to read env file {}: {}", env_file.display(), e))?;
        for (k, v) in env_parser::parse(&content) {
            if v.starts_with("stok:") {
                env.insert(k, v);
            }
        }
    }

    for (k, v) in env::vars() {
        env.insert(k, v);
    }

    // Resolve stok: references
    for value in env.values_mut() {
        if let Some(key_name) = value.strip_prefix("stok:") {
            match db_map.get(key_name) {
                Some(resolved) => *value = resolved.to_string(),
                None => return Err(format!("Key '{}' not found in store.", key_name)),
            }
        }
    }

    let mut cmd = Command::new(command);
    cmd.args(args);
    cmd.env_clear();
    cmd.envs(&env);

    exec::exec(&mut cmd).map_err(|e| format!("Failed to execute '{}': {}", command, e))
}
