use std::path;
use std::env;
use std::fs;
use std::process::exit;

// Module imports
mod util;
mod chip8;

fn main()
{
    // Collect our execution args
    let args: Vec<String> = env::args().collect();
    // let mut inject_mode: bool = false;
    // let mut patch_file_path: &String = &"".to_string();

    // Grab our filepath from our options
    if &args.len() < &2
    {
        // No file given, terminate
        util::print_help();
        exit(0);
    }

    let file_path: &String = &args[1];
    
    if path::Path::new(file_path).exists()
    {
        println!("File exists, reading '{}'...", file_path);
        
        let contents: Result<Vec<u8>, std::io::Error> = fs::read(file_path);
        
        if contents.is_ok()
        {
            let file_data: &Vec<u8> = &contents.expect("");
            if file_data.len() > chip8::MAX_PRG_SIZE.into() {
                eprintln!("Error loading file, program size too large!");
                exit(1);  
            }
            else if file_data.len() == 0 {
                eprintln!("Error loading file, no program data found!");
                exit(1);
            }

            let mut program: [u8; chip8::MAX_PRG_SIZE] = [0; chip8::MAX_PRG_SIZE];
            for i in 0..file_data.len() {
                program[i] = file_data[i];
            }

            println!("Read {} bytes of program data", file_data.len());
            chip8::start_emulation(program);
            
        }
    } else
    {
        println!("[Error] File '{}' does not exist", file_path);
        exit(-1);
    }
}
