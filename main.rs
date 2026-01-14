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

    for lines in file_reader.lines() {
        let record = lines?;

        if record.trim().is_empty() {
            continue;
        }
        println!("{}", record);
    }

    Ok(())


}



fn main() {
    if let Err(e) = read_args() {
        eprintln!("Error: {}", e);
    }
}
