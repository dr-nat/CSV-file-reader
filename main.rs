use std::fs;
use std::env;
use std::error::Error;

fn read_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Error: No argument(file) provided".into());
    }
    
    let file_name  = &args[1];

    let content = fs::read_to_string(&file_name)?; 
    
    if content.trim().is_empty() {
        return Err(format!("The {} file provided is empty", file_name).into());
    } 

    for record in content.lines() {
        println!("{}", record);
    }

    Ok(content)

}

fn main() {

    if let Err(e) = read_args() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
