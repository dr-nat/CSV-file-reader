use std::fs;
use std::env;


fn read_args() -> String {
    let args: Vec<String> = env::args().collect();
    let first_argument = &args[1];

    let file_content = fs::read_to_string(&first_argument)
        .expect("invalid input");

    file_content
}

fn main() {
    let argument = read_args();
    
    for record in argument.lines() {
        println!("{}", record);
    }
}
