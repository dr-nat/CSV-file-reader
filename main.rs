use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};
use std::error::Error;

fn read_args() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Error: No argument(file) provided".into());
    }
    
    let file_name  = &args[1];

    let content = File::open(&file_name)?; 
    
    let file_reader = BufReader::new(content);

    // outer loop: 
    for line in file_reader.lines() {
        let record = line?;

        if record.trim().is_empty() {
            continue;
        }

        // inner loop:
        for words in record.split(",") {

            println!("{}", words);
        }
    }

    Ok(())

}

fn main() {

    if let Err(e) = read_args() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
