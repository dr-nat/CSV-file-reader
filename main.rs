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

    fn get_rows(&self, index: usize) -> Result<&Vec<String>, String> {
       self.rows.get(index) 
            .ok_or(format!("Error: row index {} is out of bounds", index))
    }

    fn get_column(&self, index: usize) -> Result<&String, String>{
        self.header.get(index) 
            .ok_or(format!("Error: column index {} is out of bounds", index))
    }

    fn get_fields(&self, row: usize, col: usize) -> Result<&String, String> {
        match self.rows.get(row) {
            Some(vec_row) => match vec_row.get(col) {
                Some(value) => Ok(value),
                None => Err(format!("Error: column index {} is out of bounds", col)),
            }
            None => Err(format!("Error: row index {} is out of bounds", row)),
        }
    }
    
}



    // Accepting arguments from the command line.

fn read_args() -> Result<CsvRows, Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err(format!("No arguments(filepaths) were provided").into());
    }
    let first_arg = &args[1];
    
    let file = File::open(first_arg)?;
    
    let file_reader = BufReader::new(file);

    // Creating an iterator, using the .line() 
    let mut line = file_reader.lines();

    // we use .next to skip the first value in the vector and then iteration should start from the next value.
    // .next returns an optionand a result nested in it, ? cannot handle the returned value, so ok_or is used 
    let header_row = &line.next().ok_or("CSV file has no Header row")??;

    let header: Vec<String> = header_row
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();


    //we use double vectors to handle rows and columns, outer vec are rows and inner vec columns.
    let mut rows: Vec<Vec<String>> = Vec::new();

    for lines in line {
        match lines {
            Err(e) => {
                eprintln!("Error reading a line from the file: {}", e);
                continue;
            }
            Ok(record) => {
                if record.trim().is_empty() {
                continue;
                }

                let fields: Vec<String> = record
                    .split(",")
                    .map(|f| f.trim().to_string())
                    .collect();

                if &fields.len() == &header.len() {
                    rows.push(fields); 
                } else {
                    eprintln!("Skipping this row: it had {} columns but header has {}", fields.len(), header.len());
                }
            }
        }
    }
    
    Ok(CsvRows {
        header: header,
        rows: rows,
    })

}

fn main() {
    let Ok(csv_data) = read_args().map_err(|e| { 
        eprintln!("Error: {}", e);
        e
    }) else {
        return;
    };

    let row_count = &csv_data.num_of_rows();

    let column_count = &csv_data.num_of_columns();

    let rows_value = &csv_data.get_rows(0);

    let columns = &csv_data.get_column(0);

    let fields = &csv_data.get_fields(1, 2);

    let empty = &csv_data.is_csv_empty();


    println!("\n{:?}, \n\n{:?}, \n\n{:?}, \n\n{:?}, \n\n{:?}, \n\n{:?}", row_count, column_count, fields, empty, rows_value, columns);
    }
