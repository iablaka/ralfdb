use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use std::time::{Duration, Instant};
use ralfdb::select;
 

fn use_command(db: &str) {
    let mut path: String = env::var("RALF_PATH").unwrap_or(String::from(".")).to_owned();
    path.push_str("/");
    path.push_str(db);
    path.push_str(".db");
    if Path::new(&path).exists() {
        env::set_var("RALF_DB", path);
        println!("Now using database {}", db);
    }  
}

fn select_command(_fields: String, table_name: String) {
    let start = Instant::now();
    let rows = select(env::var("RALF_DB").unwrap(), table_name);
    let duration = start.elapsed();
    if rows.len() > 0 {
        format_rows([32,32].to_vec(), rows, duration);
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
    loop {
        print!("rdc> ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Unknown command");
        match cmd.to_lowercase().trim_end()  {
            "quit" => break,
            "exit" => break,
            _ => parse_command(&cmd.trim()),
        }
    }
}

fn format_rows(col_sizes: Vec<usize>, rows: Vec<String>, duration: Duration) {
    format_header(&col_sizes,["name", "website"].to_vec());
    for row in &rows {
        let cells: Vec<&str> = row.split(',').collect();
        for (i, cell) in cells.iter().enumerate() {
            let size = col_sizes[i];
            print!("|{:size$}", cell.trim());
        }
        println!("|");
    }
    io::stdout().flush().unwrap();
    println!("\nFound {} record(s) in {} sec", rows.len(), duration.as_secs_f32());
}

fn format_header(col_sizes: &Vec<usize>, col_names: Vec<&str>) {
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