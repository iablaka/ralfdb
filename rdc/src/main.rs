use std::io;
use std::io::Write;
use std::env;
use std::path::Path;

use std::time::{Duration, Instant};

use ralfdb::{select, Metadata, table_metadata};
 

fn use_command(db: &str) {
    let mut path: String = env::var("RALF_PATH").unwrap_or(String::from(".")).to_owned();
    path.push_str("/");
    path.push_str(db);
    path.push_str(".db");
    if Path::new(&path).exists() {
        env::set_var("RALF_DB", path);
        println!("Now using database {}", db);
    }
    else {
        println!("Could not find database {} at {}", db, path);
    }
}

fn select_command(_fields: String, table_name: String) {
    let meta: Metadata = table_metadata(env::var("RALF_DB").unwrap(), table_name.clone());
    let start = Instant::now();
    let rows = select(env::var("RALF_DB").unwrap(), table_name);
    let duration = start.elapsed();
    if rows.len() > 0 {
        format_rows(meta, rows, duration);
    }
}

fn parse_command(cmd: &str) {
    let words: Vec<&str>= cmd.split(" ").collect();
    match words[0].to_lowercase().trim_end() {
        "insert" | "update" | "delete" => println!("to be implemented"),
        "use" => use_command(&words[1]),
        "select" => select_command(words[1].to_lowercase(), words[3].to_lowercase()),
        _ => println!("Unknown command"),
    }
}
fn main() {
    let mut cmd: String = String::new();
    loop {
        print!("rdc> ");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Unknown command");
        match input.to_lowercase().trim_end()  {
            "quit" => break,
            "exit" => break,
            _ => {
                cmd = format!("{} {}", cmd, input.to_lowercase().trim_end()).trim().to_string();
                if input.trim_end().ends_with(';') {
                    cmd.pop(); //pop no longer needed final ;
                    parse_command(&cmd.trim());
                    cmd = String::from("");
                }
            },
        }
    }
}

fn format_rows(tbl_meta: Metadata, rows: Vec<String>, duration: Duration) {
    format_header(&tbl_meta.col_sizes,&tbl_meta.col_names);
    for row in &rows {
        let cells: Vec<&str> = row.split(',').collect();
        for (i, cell) in cells.iter().enumerate() {
            let size = tbl_meta.col_sizes[i];
            print!("|{:size$}", cell.trim());
        }
        println!("|");
    }
    io::stdout().flush().unwrap();
    println!("\nFound {} record(s) in {} sec", rows.len(), duration.as_secs_f32());
}

fn format_header(col_sizes: &Vec<usize>, col_names: &Vec<String>) {
    for (i, col_name) in col_names.iter().enumerate() {
            let size = col_sizes[i];
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