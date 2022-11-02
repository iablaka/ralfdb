use regex::Regex;
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

use ralfdb::{is_valid_sql, select, table_metadata, Metadata};

mod formatting;

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
    let fields: Vec<String> = match query_fields.as_str() {
        "*" => meta.col_names.clone(),
        _ => query_fields.split(',').map(|s| s.trim().to_string()).collect()
    };
    let start = Instant::now();
    let rows = select(env::var("RALF_DB").unwrap(), table_name, &fields);
    let duration: Duration = start.elapsed();
    if rows.len() > 0 {
        formatting::format_rows(meta, fields, rows, duration);
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
                let re: Regex;
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