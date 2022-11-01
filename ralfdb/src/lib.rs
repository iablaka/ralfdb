use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Metadata {
    pub col_names: Vec<String>,
    pub col_sizes: Vec<usize>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn select(db_name: String, table_name: String, fields: &Vec<String>) -> Vec<String> {
    let file_path: String = db_name.clone() + "/" + &table_name + ".tbl";
    let meta: Metadata = table_metadata(db_name, table_name);
    let mut searched_cols: Vec<usize> = Vec::new(); //will contains the indices of searched cols in order
    for field in fields.iter() {
        let idx = meta
            .col_names
            .iter()
            .position(|x| x == field.trim())
            .unwrap();
        searched_cols.push(idx);
    }
    let mut rows: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(&file_path) {
        for line in lines {
            if let Ok(record) = line {
                let mut searched_vals: String = String::new();
                let vals: Vec<&str> = record.split(',').map(|s| s.trim()).collect();
                for col in searched_cols.clone() {
                    searched_vals = format!("{},{}", searched_vals, vals[col]);
                }
                searched_vals.remove(0); //remove heading comma
                rows.push(searched_vals);
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
                let bits: Vec<&str> = line.split('=').collect();
                let prm_name: &str = bits[0];
                let prm_vals: Vec<&str> = bits[1].split(',').collect();
                if prm_name == "col_names" {
                    for val in prm_vals.iter() {
                        meta.col_names.push(val.trim().to_string());
                    }
                } else if prm_name == "col_sizes" {
                    for val in prm_vals.iter() {
                        meta.col_sizes.push(val.parse::<usize>().unwrap());
                    }
                }
            }
        }
    }
    meta
}

pub fn is_valid_sql(query: &str) -> bool {
    let mut all_regex: Vec<Regex> = Vec::new();
    all_regex.push(Regex::new("^use .+?$").unwrap());
    all_regex.push(Regex::new("^select .+? from .+?$").unwrap());
    all_regex.push(Regex::new("^select .+? from .+? where .+?$").unwrap());
    for regex in all_regex {
        if regex.is_match(query) {
            return true;
        }
    }
    false
}
