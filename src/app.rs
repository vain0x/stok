use crate::util::env_parser;
use std::env::home_dir;
use std::io;
use std::{env, fs, path::PathBuf};

/// (key, value)
pub type Pair = (String, String);

pub struct Options {
    pub env_file: Option<PathBuf>,
    pub no_env: bool,
    pub state_dir: PathBuf,
}

impl Options {
    pub fn new() -> Self {
        Options {
            env_file: None,
            no_env: env::var("STOK_NO_ENV").map(|v| v == "1").unwrap_or(false),
            state_dir: default_state_dir(),
        }
    }
}

fn default_state_dir() -> PathBuf {
    #[cfg(windows)]
    {
        let base = match env::var("LOCALAPPDATA") {
            Ok(local) => PathBuf::from(local),
            Err(_) => home_dir().unwrap().join(".local/state"),
        };
        base.join("stok")
    }
    #[cfg(not(windows))]
    {
        let base = match env::var("XDG_STATE_HOME") {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => home_dir().unwrap().join(".local/state"),
        };
        base.join("stok")
    }
}

pub fn db_path(opts: &Options) -> PathBuf {
    opts.state_dir.join(".env")
}

pub fn read_db(opts: &Options) -> Result<Vec<Pair>, String> {
    let path = db_path(opts);
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => Ok(env_parser::parse(&content)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(vec![]),
            Err(e) => Err(format!("Failed to read db: {}", e)),
        }
    } else {
        Ok(vec![])
    }
}

pub fn write_db(opts: &Options, pairs: &[Pair]) -> Result<(), String> {
    let path = db_path(opts);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create state directory: {}", e))?;
    }
    let content = env_parser::to_string(pairs);
    fs::write(&path, content).map_err(|e| format!("Failed to write db: {}", e))
}
