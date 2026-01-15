use std::env;
use std::io::{ BufRead, BufReader };
use std::fs::File;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let first_arg = &args[1];

    if first_arg.is_empty() {
        return Err(format!("No argument was provided!").into());
    }

    let file = File::open(first_arg)?;

    let file_reader = BufReader::new(file);

    let mut rows = file_reader.lines();

    let header_row = &row.next().ok_or("No header")??;

    let header: Vec<&str> = header_row.split(",").collect();

    for line in rows {
        let record = lines?;

        if record.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = records.split(",").collect();
        println!("{:?}", fields);
    }

    Ok(())
}



fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }

}
