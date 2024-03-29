use std::path;
use std::env;
use std::fs;
use std::process::exit;

// Module imports
mod util;

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
            let _bytes: &Vec<u8> = &contents.expect("");
            // let magic_num: &[u8] = &bytes[0..4];   
        }
    } else
    {
        println!("[Error] File '{}' does not exist", file_path);
        exit(-1);
    }
}
