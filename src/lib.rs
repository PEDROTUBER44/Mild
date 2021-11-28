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
                Err(e) => println!("Nao foi possivel remover o arquivo: {} ERRO:{}",display,e),
                Ok(_) => println!("Arquivo excluido com sucesso"),
            };

            let mut file = match File::create(&cpath) {
        
                Err(e) => {
                    println!("Nao foi possivel criar o arquivo: {} ERRO:{}",display,e);
                    process::exit(0x0100);
                },
        
                Ok(file) => file,
        
            };

            match file.write_all(text.as_bytes()) {

                Err(e) => {
        
                    println!("Nao foi possivel escrever no arquivo: {} ERRO:{}",display,e);
                    process::exit(0x0100);
        
                },
        
                Ok(_) => println!("Arquivo escrito com sucesso"),
        
            };

        },

        false => {
            println!("{}",appnotfound);
        },
    }
}
