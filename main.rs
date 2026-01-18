use std::env;
use std::io::{ BufRead, BufReader};
use std::error::Error;
use std::fs::File;


struct CsvRows {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl CsvRows {
    
}



fn read_args() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments(filepaths) were provided").into());
    }
    let first_arg = &args[1];
    
    let file = File::open(first_arg)?;
    
    let file_reader = BufReader::new(file);

    let mut line = file_reader.lines();

    let header_row = &line.next().ok_or("CSV file has no Header row")??;

    let header: Vec<String> = header_row
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();


    let mut rows: Vec<Vec<String>> = Vec::new();

    for lines in line {
        let record = lines?;

        if record.trim().is_empty() {
            continue;
        }

        let fields: Vec<String> = record
            .split(",")
            .map(|f| f.trim().to_string())
            .collect();

        if &fields.len() == &header.len() {
            rows.push(fields); }  else {
            break;
        }
    }
    
    let csv_rows = CsvRows {
        header: header,
        rows: rows,
    };

    Ok(())


}



fn main() {
    if let Err(e) = read_args() {
        eprintln!("Error: {}", e);
    }
}
