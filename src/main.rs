use std::fs::{OpenOptions, File};
use std::io::{Write, Error, BufReader, BufRead};

fn main() {
    
}

async fn send_data(path: &str, data: &str) -> bool {
    let mut file: File = match OpenOptions::new().write(true).create(true).append(true).open(path) {
        Ok(file) => file,
        Err(e) => { println!("An error occurred while opening the file! {}", e); return false; },
    };

    match writeln!(&mut file, "{}", data) {
        Ok(_) => return true,
        Err(e) => { println!("An error occurred while writing the file! {}", e); return false; },
    };
}

async fn get_data(file_path: &str) -> Result<String, Error> {
    let mut contents = Vec::new();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let mut lines = BufReader::new(&file).lines();
    if let Some(Ok(first_line)) = lines.next() {
        contents.push(first_line.clone());

        // Read the rest of the file
        contents.extend(lines.filter_map(|line| line.ok()));
    }

    // Reopen the file and write back the contents without the first line (truncate it)
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    if contents.len() > 0 {
        for line in &contents[1..] {
            writeln!(&mut file, "{}", line)?;
        }
    }

    let answer = if contents.len() > 0 { contents[0].clone() } else { String::new() };

    Ok(answer)
}