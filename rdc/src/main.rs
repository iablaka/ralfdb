use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use ralfdb::select;

//TODO: format function that prints the result in a table-like format
//with -, + and |


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

fn select_command(_fields: String, table: String) {
    println!("{:?}", select(env::var("RALF_DB").unwrap(), table));
}


fn parse_command(cmd: &str) {
    let words: Vec<&str>= cmd.split(" ").collect();
    match words[0].to_lowercase().as_str() {
        "insert" | "update" | "delete" => println!("to be implemented"),
        "use" => use_command(&words[1]),
        "select" => select_command(words[1].to_lowercase(), words[3].to_lowercase()),
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
            "exit\n" => break,
            _ => parse_command(&cmd.trim()),
        }
    }
    Ok(())
}