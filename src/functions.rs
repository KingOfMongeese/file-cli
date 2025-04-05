use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use prettytable::{row, Cell, Row, Table};
use sha256::try_digest;
use std::error::Error;
use std::fs::{create_dir_all, metadata, read_dir, remove_dir_all, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, absolute};
use walkdir::WalkDir;

fn get_dir_content(path: &String) -> Vec<String> {
    let mut contents = vec![];

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        contents.push(entry.path().display().to_string());
    }
    contents
}

pub fn remove_all(path: String, print_removed_contents: bool) -> std::io::Result<()> {
    if print_removed_contents {
        let content = get_dir_content(&path);
        for entry in content {
            println!("Removing: {}", entry)
        }
    }

    remove_dir_all(path)?;

    Ok(())
}

pub fn make_dirs(path: String) -> std::io::Result<()> {
    let path_obj = Path::new(&path);
    if !path_obj.exists() {
        create_dir_all(&path)?;
    }

    Ok(())
}

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

pub fn write(text: String, out_file: String) -> std::io::Result<()> {
    let path = Path::new(&out_file);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
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
            if show_line_numbers {
                println!("{}. {}", cnt, line)
            } else {
                println!("{}", line)
            };
        } else {
            eprintln!("Error reading line {}", cnt);
        }
    }
    Ok(())
}

fn process_file(f_path: String) -> Result<Row, Box<dyn Error>> {
    let file_data = metadata(&f_path)?;

    let mut cells = vec![];

    // file name
    cells.push(Cell::new(&f_path));

    // size in bytes
    let size = ByteSize::b(file_data.len());
    cells.push(Cell::new(&size.to_string()));

    // Created At
    let created_at = file_data.created()?;
    let created_at_time: DateTime<Utc> = DateTime::from(created_at);
    cells.push(Cell::new(&created_at_time.to_string()));

    // Modifed At
    let modded_at = file_data.modified()?;
    let modded_at_time: DateTime<Utc> = DateTime::from(modded_at);
    cells.push(Cell::new(&modded_at_time.to_string()));

    // sha256 hash
    let file_path = Path::new(&f_path);
    let hash_value = try_digest(file_path)?;
    cells.push(Cell::new(&hash_value));

    Ok(Row::new(cells))
}

fn process_dir(f_path: String) -> Result<Row, Box<dyn Error>> {
    let file_data = metadata(&f_path)?;
    let content = get_dir_content(&f_path);

    let mut cells = vec![];

    // file name
    cells.push(Cell::new(&f_path));

    let mut size = ByteSize::b(0);
    for file in &content {
        let new_file_data = metadata(file)?;
        size += ByteSize::b(new_file_data.len())
    }
    cells.push(Cell::new(&size.to_string()));

    // Created At
    let created_at = file_data.created()?;
    let created_at_time: DateTime<Utc> = DateTime::from(created_at);
    cells.push(Cell::new(&created_at_time.to_string()));

    // Modifed At
    let modded_at = file_data.modified()?;
    let modded_at_time: DateTime<Utc> = DateTime::from(modded_at);
    cells.push(Cell::new(&modded_at_time.to_string()));

    Ok(Row::new(cells))
}

fn process_files(table: &mut Table, f_path: String) -> Result<(), Box<dyn Error>> {

    let file_path = Path::new(&f_path);
    let mut files_to_process: Vec<String> = vec![];

    if file_path.is_dir() {
        for entry in read_dir(file_path)? {
            let entry = entry?;
            let path = absolute(entry.path())?;
            files_to_process.push(path.display().to_string());
        }
    }

    for file in files_to_process {
        let path_obj = Path::new(&file);
        if path_obj.is_dir() {
            process_wrapper(table, file, &process_dir);
        } else {
            process_wrapper(table, file, &process_file);
        }

    }

    Ok(())
}

fn process_wrapper(table: &mut Table, file: String, func: &dyn Fn(String) -> Result<Row, Box<dyn Error>>) {
    match func(file.clone()) {
        Ok(row) => {
            table.add_row(row);
        },
        Err(err) => println!("Error getting data for {}\nError: {}\n----------------", file, err),
    }
}

pub fn show_info(in_file: String) {
    let mut table = Table::new();

    table.add_row(row![
        "Name",
        "Size",
        "Created At",
        "Modified At",
        "sha256 Hash"
    ]);

    let _ = process_files(&mut table, in_file);

    table.printstd();
}
