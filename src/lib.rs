use std::io::prelude::Write;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::process;

pub fn texto(text: &str, path: &str, appnotfound: &str) {
    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();

    match bools {
        true => {
            match fs::remove_file(&cpath) {
                Err(e) => println!("Unable to remove file: {} ERRO:{}",display,e),
                Ok(_) => println!("File removed successfully"),
            };

            let mut file = match File::create(&cpath) {
        
                Err(e) => {
                    println!("Could not write to file: {} Error:{}",display,e);
                    process::exit(0x0100);
                },
        
                Ok(file) => file,
        
            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {
        
                    println!("Could not write to file: {} Error:{}",display,e);
                    process::exit(0x0100);
        
                },
        
                Ok(_) => println!("File written successfully"),
        
            };

        },

        false => {
            println!("{}",appnotfound);
        },
    }
}

pub fn dir(path: &str, dirnotfound: &str) {

    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();

    match bools {

        true => {

            match fs::remove_dir_all(&cpath) {

                Err(e) => println!("Unable to remove folder: {} Error:{}",display,e),
                Ok(_) => println!("Folder removed successfully"),

            };

        },

        false => {

            println!("{}",dirnotfound);

        },

    }

}