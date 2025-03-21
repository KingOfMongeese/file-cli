use std::fs::{File, create_dir_all, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn append(text: String, out_file: String, add_line_numbers: bool) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(out_file)?;

    let reader = BufReader::new(&file);
    let last_line = reader.lines().count();

    for (mut cnt, line) in text.lines().enumerate() {
        if add_line_numbers {
            cnt += 1;
            let line_number = last_line + cnt;
            writeln!(file, "{}. {}", line_number, line)?;
            continue;
        }

        writeln!(file, "{}", line)?;
    }
    Ok(())
}

pub fn write(text: String, out_file: String) -> std::io::Result<()>{

    let path = Path::new(&out_file);
    if let Some(parent) = path.parent() {
        if ! parent.exists() {
            if let Err(value) = create_dir_all(parent) {
                eprintln!("Error creating parrent directories {}", value);
            }
        }
    }

    let mut file = File::create(&out_file)?;
    for line in text.lines() {
        writeln!(file, "{}", line)?;
    }


    Ok(())

}

pub fn read(in_file: String, show_line_numbers: bool) -> std::io::Result<()> {

    let file = File::open(in_file)?;
    let reader = BufReader::new(file);


    for (mut cnt, res) in reader.lines().enumerate() {
        cnt += 1; // lines start at 1, but index starts at 0

        if let Ok(line) = res {
            if show_line_numbers { println!("{}. {}", cnt, line)} else {println!("{}", line)};
        } else {
            eprintln!("Error reading line {}", cnt);
        }
    }
    Ok(())
}