use std::{fs::File, path::Path};

use crate::error::Result;

pub async fn run(_token: String, file: String) -> Result<()> {
    let path = Path::new(&file);
    println!("batch file path: {}", path.display());

    let _batch_file = File::open(&path)?;

    Ok(())
}
