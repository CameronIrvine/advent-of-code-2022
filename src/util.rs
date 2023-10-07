use std::fs;
use std::process::exit;

pub fn read_file_to_string(filename: String) -> Vec<String> {
    let filename = format!(
        "{}\\inputs\\{filename}",
        std::env::current_dir().unwrap().display()
    );
    let read_res = &fs::read_to_string(filename);
    match read_res {
        Ok(data) => {
            let lines: Vec<String> = data.lines().map(String::from).collect();
            lines
        }
        Err(err) => {
            dbg!(err);
            exit(1);
        }
    }
}
