use std::io;
use std::io::Write;
use std::time::{Duration};

use ralfdb::{Metadata};

pub fn format_rows(tbl_meta: Metadata, fields: Vec<String>, rows: Vec<String>, duration: Duration) {
    format_header(&tbl_meta, &fields);
    for row in &rows {
        let cells: Vec<&str> = row.split(',').collect();
        for (i, cell) in cells.iter().enumerate() {
            let size = this_col_size(&tbl_meta, &fields[i]);
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

fn format_header(tbl_meta: &Metadata, col_names: &Vec<String>) {
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

fn this_col_size(table_metadata: &Metadata, col: &String) -> usize {
    let col_idx = table_metadata.col_names.iter().position(|c| c == col).unwrap();
    table_metadata.col_sizes[col_idx]
}