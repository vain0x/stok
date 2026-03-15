use crate::app::{read_db, write_db, Options};

pub fn run(opts: &Options, name: &str) -> Result<(), String> {
    let mut pairs = read_db(opts)?;
    if let Some(index) = pairs.iter().position(|(k, _)| k == name) {
        pairs.remove(index);
        write_db(opts, &pairs)?;
    }
    Ok(())
}
