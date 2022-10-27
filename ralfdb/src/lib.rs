use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
 

pub struct Metadata {
    pub col_names: Vec<String>,
    pub col_sizes: Vec<usize>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn select(db_name: String, table_name: String) -> Vec<String> {
    let file_path: String = db_name + "/" + &table_name + ".tbl";
    let mut rows: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(&file_path) {
        for line in lines {
            if let Ok(record) = line {
                rows.push(record);
            }
        }
    }
    rows
}

pub fn table_metadata(db_name: String, table_name: String) -> Metadata {
    let file_path: String = db_name + "/" + &table_name + ".meta";
    let mut meta: Metadata = Metadata {
        col_names: Vec::new(),
        col_sizes: Vec::new(),
    };
    if let Ok(lines) = read_lines(&file_path) {
        for line in lines {
            if let Ok(line) = line {
                let bits:Vec<&str> = line.split('=').collect();
                let prm_name:&str = bits[0];
                let prm_vals:Vec<&str> = bits[1].split(',').collect();
                if prm_name == "col_names" {
                    for val in prm_vals.iter() {
                        meta.col_names.push(val.to_string());
                    }
                }
                else if prm_name == "col_sizes" {
                    for val in prm_vals.iter() {
                        meta.col_sizes.push(val.parse::<usize>().unwrap());
                    }
                }
            }
        }
    }
    meta
}