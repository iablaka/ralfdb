use std::io;
use std::io::Write;

fn parse_command(cmd: &str) {
    let words: Vec<&str>= cmd.split(" ").collect();
    match words[0].to_lowercase().as_str() {
        "insert" | "select" | "update" | "delete" => println!("to be implemented"),
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