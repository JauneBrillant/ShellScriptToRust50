use std::{fs, io};

fn main() -> io::Result<()> {
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in entries {
        if let Some(file) = entry.file_name() {
            println!("{}", file.to_string_lossy());
        }
    }

    Ok(())
}
