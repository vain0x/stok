use crate::app::{read_db, write_db, Options};

pub fn run(opts: &Options, name: &str, value: Option<&str>) -> Result<(), String> {
    let value = match value {
        Some(v) => v.to_string(),
        None => rpassword::prompt_password(format!("Enter value: "))
            .map_err(|e| format!("Failed to read value: {}", e))?,
    };

    let mut pairs = read_db(opts)?;
    if let Some(pos) = pairs.iter().position(|(k, _)| k == name) {
        pairs[pos] = (name.to_string(), value);
    } else {
        pairs.push((name.to_string(), value));
    }
    write_db(opts, &pairs)?;
    Ok(())
}
