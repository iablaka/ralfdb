use std::io;
use std::io::Write;
use std::env;
use std::path::Path;

fn use_command(db: &str) {
    env::set_var("RALF_DB", db.to_owned() + ".db");
    let mut path: String = env::var("RALF_PATH").unwrap_or(String::from(".")).to_owned();
    path.push_str("/");
    path.push_str(&env::var("RALF_DB").unwrap());
    if Path::new(&path).exists() {
        println!("Now using database {}", db);
    }
    
}


fn parse_command(cmd: &str) {
    let words: Vec<&str>= cmd.split(" ").collect();
    match words[0].to_lowercase().as_str() {
        "insert" | "select" | "update" | "delete" => println!("to be implemented"),
        "use" => use_command(&words[1]),
        _ => println!("Unknown command"),
    }
}

fn main() -> io::Result<()> {
    loop {
        print!("rdc> ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Unknown command");
        match cmd.to_lowercase().as_str()  {
            "quit\n" => break,
            _ => parse_command(&cmd.trim()),
        }
    }
    Ok(())
}