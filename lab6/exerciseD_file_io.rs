use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn write_log(path: &str, entries: &[String]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for entry in entries {
        writeln!(file, "{}", entry)?;
    }
    Ok(())
}

fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

fn list_rs_files<P: AsRef<Path>>(dir: P) -> io::Result<Vec<String>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            result.extend(list_rs_files(&path)?);
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            result.push(path.display().to_string());
        }
    }
    Ok(result)
}

fn main() -> io::Result<()> {
    let path = "output.log";
    let entries = vec![
        "INFO Server started".to_string(),
        "WARN High memory usage".to_string(),
        "ERROR Disk full".to_string(),
        "INFO Backup complete".to_string(),
    ];
    write_log(path, &entries)?;

    let n = count_lines(path)?;
    println!("Wrote {} lines to {}", n, path);

    let content = fs::read_to_string(path)?;
    let errors: Vec<&str> = content.lines().filter(|l| l.starts_with("ERROR")).collect();
    println!("Error lines: {:?}", errors);

    fs::remove_file(path)?;

    let rs_files = list_rs_files(".")?;
    println!("Found {} .rs files:", rs_files.len());
    for f in &rs_files {
        println!("  {}", f);
    }
    Ok(())
}
