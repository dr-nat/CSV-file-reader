use std::env;
use std::io::{ BufRead, BufReader};
use std::error::Error;
use std::fs::File;


fn read_args() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("No arguments(filepaths) were provided").into());
    }
    let first_arg = &args[1];
    
    let file = File::open(first_arg)?;
    
    let file_reader = BufReader::new(file);

    let mut rows = file_reader.lines();

    let header_row = &rows.next().ok_or("CSV file has no Header row")??;

    let header: Vec<&str> = header_row.split(",").collect();
    println!("{:?}", header);


    for lines in rows {
        let record = lines?;

        if record.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = record.split(",").collect();
        println!("{:?}", fields);
    }

    Ok(())


}



fn main() {
    if let Err(e) = read_args() {
        eprintln!("Error: {}", e);
    }
}
