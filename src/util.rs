use std::fs;
use std::process::exit;

pub fn read_file_to_string(filename: String) -> Vec<String> {
    let mut path = std::env::current_dir().unwrap();
    path.push("inputs");
    path.push(filename);
    let read_res = &fs::read_to_string(path);
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
