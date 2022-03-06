use std::io::prelude::Write;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::process;

pub fn text(text: &str, path: &str, appnotfound: &str) {
    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();

    match bools {
        true => {
            match fs::remove_file(&cpath) {
                Err(e) => println!("Could not remove the file: {} ERROR:{}",display,e),
                Ok(_) => println!("File deleted successfully"),
            };

            let mut file = match File::create(&cpath) {
        
                Err(e) => {
                    println!("Could not create the file: {} ERROR:{}",display,e);
                    process::exit(0x0100);
                },
        
                Ok(file) => file,
        
            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {
        
                    println!("Could not written the file: {} ERROR:{}",display,e);
                    process::exit(0x0100);
        
                },
        
                Ok(_) => println!("File successfully written"),
        
            };

        },

        false => {
            println!("{}",appnotfound);
        },
    }
}
