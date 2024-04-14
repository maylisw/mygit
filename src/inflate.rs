use anyhow::{anyhow, Result};
use miniz_oxide::inflate;
use std::io::{stdin, stdout, Read, Write};

fn main() -> Result<()> {
    let mut input = vec![];
    stdin().read_to_end(&mut input)?;

    let data = match inflate::decompress_to_vec(&input) {
        Ok(data) => data,
        Err(err) => return Err(anyhow!("inflate: {}", err)),
    };

    stdout().write_all(&data)?;
    return Ok(());
}
