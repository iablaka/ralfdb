use regex::Regex;
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

use ralfdb::{is_valid_sql, select, table_metadata, Metadata};

fn use_command(db: &str) {
    let mut path: String = env::var("RALF_PATH")
        .unwrap_or(String::from("."))
        .to_owned();
    path.push_str("/");
    path.push_str(db);
    path.push_str(".db");
    if Path::new(&path).exists() {
        env::set_var("RALF_DB", path);
        println!("Now using database {}", db);
    }
}

fn select_command(table_name: String, query_fields: String) {
    let meta: Metadata = table_metadata(env::var("RALF_DB").unwrap(), table_name.clone());
    let mut fields: Vec<String> = Vec::new();
    if query_fields != String::from("*") {
        fields = query_fields.split(',').map(|s| s.trim().to_string()).collect();
    } else {
        fields = meta.col_names;
    }
    let start = Instant::now();
    let rows = select(env::var("RALF_DB").unwrap(), table_name, fields);
    let duration = start.elapsed();
    if rows.len() > 0 {
        format_rows(meta, fields, rows, duration);
    }
}

fn parse_command(cmd: &str) {
    if is_valid_sql(&cmd) {
        let words: Vec<&str> = cmd.split(" ").collect();
        match words[0].to_lowercase().trim_end() {
            "use" => use_command(&words[1]),
            "select" => {
                let simple_re = Regex::new("^select (?P<fields>.+?) from (?P<table>.+?)$").unwrap();
                let where_re = Regex::new(
                    "^select (?P<fields>.+?) from (?P<table>.+?) where (?P<criteria>.+?)$",
                )
                .unwrap();
                let mut re: Regex = Regex::new("").unwrap();
                if where_re.is_match(cmd) {
                    re = where_re;
                } else {
                    re = simple_re;
                }
                let caps = re.captures(cmd).unwrap();
                select_command(
                    caps.name("table").map_or("", |m| m.as_str()).to_lowercase(),
                    caps.name("fields")
                        .map_or("", |m| m.as_str())
                        .to_lowercase(),
                )
            }
            _ => {}
        }
    } else {
        println!("Not a valid query or not implemented yet");
    }
}
fn main() {
    let mut cmd = String::new();
    loop {
        print!("rdc> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unknown command");
        match input.to_lowercase().trim_end() {
            "quit" => break,
            "exit" => break,
            _ => {
                cmd = format!("{} {}", cmd, input.to_lowercase().trim_end())
                    .trim()
                    .to_string();
                if input.trim_end().ends_with(';') {
                    cmd.pop(); //pop the no longer needed final ;
                    parse_command(&cmd.trim());
                    cmd = String::from("");
                }
            }
        }
    }
}

fn format_rows(tbl_meta: Metadata, fields: Vec<String>, rows: Vec<String>, duration: Duration) {
    format_header(tbl_meta, fields);
    for row in &rows {
        let cells: Vec<&str> = row.split(',').collect();
        for (i, cell) in cells.iter().enumerate() {
            let size = tbl_meta.col_sizes[i];
            print!("|{:size$}", cell.trim());
        }
        println!("|");
    }
    io::stdout().flush().unwrap();
    println!(
        "\nFound {} record(s) in {} sec",
        rows.len(),
        duration.as_secs_f32()
    );
}

fn format_header(tbl_meta: Metadata, col_names: Vec<String>) {
    let mut col_sizes: Vec<usize> = Vec::new();
    for col_name in col_names.iter() {
        let size = this_col_size(tbl_meta, col_name);
        col_sizes.push(size);
        print!("|{:size$}", col_name);
    }
    println!("|");
    for col_size in col_sizes.iter() {
        let size = col_size + 1;
        print!("{:-<size$}", "+");
    }
    io::stdout().flush().unwrap();
    println!("+");
}

fn this_col_size(table_metadata: Metadata, col: &String) -> usize {
    let col_idx = table_metadata.col_names.iter().position(|c| c == col).unwrap();
    table_metadata.col_sizes[col_idx]
}