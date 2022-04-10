use std::{
    io::{
        prelude::{
            Write // Import standard write library to write files
        },
    },
    process::{
        exit // Import standard exit library to exit to program
    },
    path::{
        Path // Import standard manipulator of directory's
    },
    fs::{
        File // Import standard manipulator of files
    },
    fs // Import standard manipulator of files and folders
};

pub fn create_dir(path: &str) {

    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();
    
    match bools {

        true => {

            let remove_dir = fs::remove_dir(&cpath);

            match remove_dir {

                Err(e) => {

                    println!("Could not remove folder: {}  Error: {}",display,e);

                },

                Ok(_) => {

                    println!("Folder successfully removed");

                    let create = fs::create_dir(&cpath);

                    match create {

                        Err(e) => {

                            println!("Error creating folder: {}  Error: {}",display,e);
                            exit(1);

                        },

                        Ok(_) => {

                            println!("Directory: {} successfully created",display);

                        }

                    }

                }

            };

        },

        false => {

            let create = fs::create_dir(&cpath);

            match create {

                Err(e) => {
                    println!("Error creating folder: {}  Error: {}",display,e);
                    exit(1);
                },

                Ok(_) => {
                    println!("Directory: {} successfully created",display);
                }

            }

        }

    }

}

pub fn text(text: &str, path: &str) {
    
    let cpath = Path::new(&path);
    let bools = cpath.exists();
    let display = cpath.display();

    match bools {

        true => {

            match fs::remove_file(&cpath) {

                Err(e) => {

                    println!("Could not remove file: {}  Error:{}",display,e);
                    exit(1);

                },

                Ok(_) => {

                    println!("File: {} removed successfully",display);

                }

            };

            let mut file = match File::create(&cpath) {
                
                Err(e) => {

                    println!("Could not remove file: {} Error: {}",display,e);
                    exit(0);

                },

                Ok(file) => file

            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {

                    println!("Could not write to file: {} Error: {}",display,e);
                    exit(0);

                },

                Ok(_) => {

                    println!("File: {} written successfully",display);

                }

            };

        },
        
        false => {
            
            let mut file = match File::create(&cpath) {
                
                Err(e) => {

                    println!("Could not create file: {} Error: {}",display,e);
                    exit(0);

                },

                Ok(file) => file

            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {

                    println!("Could not write to file: {} Error: {}",display,e);
                    exit(0);

                },

                Ok(_) => {

                    println!("File: {} written successfully",display);
                    
                }

            };

        }

    }

}