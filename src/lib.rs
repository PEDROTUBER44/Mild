use std::{
    io::prelude::Write, // Import Write Command to write files
    process::exit, // Import Exit Command to exit to program
    path::Path, // 
    fs::File, //
    fs // Import standard files and folders manipulation
};

pub fn text(text: &str, path: &str, appnotfound: &str) {
    
    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();

    match bools {

        true => {

            match fs::remove_file(&cpath) {

                Err(e) => println!("Could not remove file: {} ERROR: {}",display,e),
                Ok(_) => println!("File removed successfully")

            };

            let mut file = match File::create(&cpath) {
                
                Err(e) => {
                    println!("Could not remove file: {} ERROR: {}",display,e);
                    exit(0)
                },
                Ok(file) => file

            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {
                    println!("Could not write to file: {} ERROR: {}",display,e);
                    exit(0);
                },
                Ok(_) => println!("File successfully written")

            };

        },
        
        false => {
            println!("{}",appnotfound);
        }

    }

}