use crate::app::{read_db, Options};

pub fn run(opts: &Options) -> Result<(), String> {
    let pairs = read_db(opts)?;
    println!(
        "{}",
        pairs
            .iter()
            .map(|(k, _)| k.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}
