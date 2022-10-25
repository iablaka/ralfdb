use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
 

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn select(db_name: String, table: String) -> Vec<String> {
    let file_path: String = db_name + "/" + &table + ".tbl";
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

//TODO a function that returns the column sizes
//I need to share the path to the db folder to read things inside


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
