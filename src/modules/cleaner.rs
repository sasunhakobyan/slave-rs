use regex::Regex;
use std::fs::{self, DirEntry};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn process_file(entry: &DirEntry) {
    let path = entry.path();
    if let Ok(file) = fs::File::open(&path) {
        let reader = BufReader::new(file);
        let regex = Regex::new(r"console\.log\s*\([^)]*\)").unwrap();

        let contains_console_log = reader.lines().any(|line| {
            if let Ok(l) = line {
                regex.is_match(&l)
            } else {
                false
            }
        });

        if contains_console_log {
            let file = fs::File::open(&path).unwrap();
            let reader = BufReader::new(file);
            let mut new_contents = String::new();

            for line in reader.lines() {
                let line = line.unwrap();
                if !regex.is_match(&line) {
                    new_contents.push_str(&line);
                    new_contents.push('\n');
                }
            }

            fs::write(path, new_contents).expect("Unable to write to file");
        }
    }
}

pub fn run_module(cwd: &String) -> io::Result<()> {
    let dir = format!("{}/src/", cwd);
    let path = Path::new(&dir);

    println!("Are you sure to delete all logs inside current dir? {}, (y/n)", path.display());

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    
    let answer = answer.trim();

    if answer == "y" {
        println!("Workdir {}", path.display());
        visit_dirs(&path, &process_file)?;
    } else {
        println!("Closing...");
    }

    Ok(())
}