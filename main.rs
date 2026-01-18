use std::env;
use std::io::{ BufRead, BufReader};
use std::error::Error;
use std::fs::File;


struct CsvRows {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl CsvRows {
    fn num_of_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_of_columns(&self) -> usize {
        self.header.len()

    }

    fn is_csv_empty(&self) -> bool {
        if self.header.is_empty() && self.rows.is_empty() {
            true
        } else {
            false
        }
    }

    fn get_rows(&self, index: usize) -> Option<&Vec<String>> {
       self.rows.get(index) 
    }

    fn get_column(&self, index: usize) -> Option<&String>{
        self.header.get(index)
    }

    fn get_fields(&self, record: usize, fields: usize) -> Option<&String> {
        self.rows.get(record)?.get(fields)
    }
    
}



fn read_args() -> Result<CsvRows, Box<dyn Error>>{
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
    
    Ok(CsvRows {
        header: header,
        rows: rows,
    })

}



fn main() {
    if let Err(e) = read_args() {
        eprintln!("Error: {}", e);
    }
}
